// Service de gestion du timer Pomodoro
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { timerStore } from '$lib/stores/timer.svelte';
import { settingsStore } from '$lib/stores/settings.svelte';
import { taskStore } from '$lib/stores/tasks.svelte';
import { notifyComplete } from '$lib/services/notification-service';
import type { SessionType, PomodoroSession } from '$lib/types';

/**
 * Fréquence de rafraîchissement : le temps restant est calculé depuis l'échéance,
 * un rafraîchissement fréquent évite simplement les secondes sautées à l'affichage.
 */
const TICK_MS = 250;

/**
 * Délai avant l'enchaînement automatique de la session suivante
 */
const AUTO_ADVANCE_MS = 3000;

let timerInterval: number | null = null;
let autoAdvanceTimeout: number | null = null;
let lastTitleSecond = -1;

/**
 * Callbacks appelés quand une session se termine (effets propres à une vue)
 */
type SessionCompleteCallback = (sessionType: SessionType) => void;
const completeListeners = new Set<SessionCompleteCallback>();

function clearTimers(): void {
	if (timerInterval !== null) {
		clearInterval(timerInterval);
		timerInterval = null;
	}
	if (autoAdvanceTimeout !== null) {
		clearTimeout(autoAdvanceTimeout);
		autoAdvanceTimeout = null;
	}
}

function durationFor(sessionType: SessionType): number {
	const settings = settingsStore.settings;
	switch (sessionType) {
		case 'work':
			return settings.workDuration;
		case 'short_break':
			return settings.shortBreakDuration;
		case 'long_break':
			return settings.longBreakDuration;
	}
}

/**
 * Démarre une nouvelle session Pomodoro
 *
 * @param sessionType - Type de session (work, short_break, long_break)
 * @param taskId - ID de la tâche associée (optionnel, uniquement pour work)
 */
export async function startSession(sessionType: SessionType, taskId?: number): Promise<void> {
	// Arrête toute session en cours
	await stopTimer();

	const duration = durationFor(sessionType);
	const linkedTaskId = sessionType === 'work' ? taskId : undefined;

	try {
		// Crée la session dans la base de données
		const session = await invoke<PomodoroSession>('create_session', {
			taskId: linkedTaskId,
			durationMinutes: duration,
			sessionType
		});

		timerStore.nextType = sessionType;
		timerStore.startSession(duration, sessionType, linkedTaskId, session.id);
		startInterval();
	} catch (error) {
		console.error('Failed to start session:', error);
		throw error;
	}
}

/**
 * Bascule le minuteur : démarre la session proposée, met en pause ou reprend.
 * Utilisé par la vue principale, le mode focus, le mini-mode et Ctrl+S.
 *
 * @param taskId - Tâche à associer si une session de travail démarre
 */
export async function toggleTimer(taskId?: number): Promise<void> {
	if (timerStore.isRunning) {
		pauseTimer();
	} else if (timerStore.remainingSeconds > 0) {
		resumeTimer();
	} else {
		await startSession(timerStore.nextType, taskId);
	}
}

/**
 * Met à jour le titre et la barre de progression de la fenêtre (une fois par seconde)
 */
async function updateWindowProgress(): Promise<void> {
	const { remainingSeconds, totalSeconds } = timerStore;
	if (totalSeconds <= 0 || remainingSeconds === lastTitleSecond) return;
	lastTitleSecond = remainingSeconds;

	const progress = Math.round((1 - remainingSeconds / totalSeconds) * 100);
	try {
		const appWindow = getCurrentWindow();
		await appWindow.setTitle(`${timerStore.displayTime()} - TomatoTask`);
		// @ts-expect-error -- l'API accepte un statut sous forme de chaîne
		await appWindow.setProgressBar({ status: 'normal', progress });
	} catch {
		// Hors Tauri (navigateur, tests) : rien à faire
	}
}

async function resetWindowProgress(): Promise<void> {
	lastTitleSecond = -1;
	try {
		const appWindow = getCurrentWindow();
		await appWindow.setTitle('TomatoTask');
		// @ts-expect-error -- l'API accepte un statut sous forme de chaîne
		await appWindow.setProgressBar({ status: 'none' });
	} catch {
		// Hors Tauri (navigateur, tests) : rien à faire
	}
}

/**
 * Lance l'intervalle du timer
 */
function startInterval(): void {
	if (timerInterval !== null) {
		clearInterval(timerInterval);
	}

	timerInterval = window.setInterval(() => {
		const isComplete = timerStore.tick();
		void updateWindowProgress();
		if (isComplete) {
			void handleSessionComplete();
		}
	}, TICK_MS);
}

/**
 * Met le timer en pause
 */
export function pauseTimer(): void {
	if (timerInterval !== null) {
		clearInterval(timerInterval);
		timerInterval = null;
	}

	timerStore.pause();
}

/**
 * Reprend le timer
 */
export function resumeTimer(): void {
	if (!timerStore.isRunning && timerStore.remainingSeconds > 0) {
		timerStore.resume();
		startInterval();
	}
}

/**
 * Arrête complètement le timer
 *
 * @param interrupted - La session a-t-elle été interrompue? (défaut: true)
 */
export async function stopTimer(interrupted: boolean = true): Promise<void> {
	clearTimers();

	// Si une session est en cours, la marquer comme interrompue
	const sessionId = timerStore.sessionId;

	if (sessionId && interrupted) {
		try {
			await invoke('interrupt_session', { id: sessionId });
		} catch (error) {
			console.error('Failed to interrupt session:', error);
		}
	}

	timerStore.stop();
	await resetWindowProgress();
}

/**
 * Passe la session courante et propose la suivante
 */
export async function skipSession(): Promise<void> {
	const current = timerStore.remainingSeconds > 0 ? timerStore.sessionType : timerStore.nextType;
	await stopTimer();
	timerStore.nextType = current === 'work' ? 'short_break' : 'work';
}

/**
 * Gère la complétion d'une session, quelle que soit la vue affichée
 */
async function handleSessionComplete(): Promise<void> {
	clearTimers();

	const { sessionId, sessionType } = timerStore.getState();

	if (sessionId) {
		try {
			// Marque la session comme complétée dans la DB
			await invoke('complete_session', { id: sessionId });
		} catch (error) {
			console.error('Failed to complete session:', error);
		}
	}

	if (sessionType === 'work') {
		timerStore.completeSession();
	} else {
		timerStore.stop();
	}
	timerStore.nextType = getNextSessionType();
	await resetWindowProgress();

	for (const listener of completeListeners) listener(sessionType);

	// Les compteurs de la tâche ont changé côté base
	if (sessionType === 'work') {
		await taskStore.load().catch((error) => console.error('Failed to reload tasks:', error));
	}
	// Une notification impossible (permission, son bloqué…) ne doit pas bloquer l'enchaînement
	await notifyComplete(sessionType).catch((error) =>
		console.error('Failed to notify session completion:', error)
	);

	const settings = settingsStore.settings;
	const autoStart =
		(sessionType === 'work' && settings.autoStartBreaks) ||
		(sessionType !== 'work' && settings.autoStartPomodoros);

	if (autoStart) {
		autoAdvanceTimeout = window.setTimeout(() => {
			autoAdvanceTimeout = null;
			// L'utilisateur a pu démarrer une session entre-temps
			if (timerStore.remainingSeconds > 0) return;
			void startSession(timerStore.nextType, taskStore.selectedTask?.id);
		}, AUTO_ADVANCE_MS);
	}
}

/**
 * Enregistre un callback pour la complétion de session
 *
 * @param callback - Fonction appelée quand une session se termine
 * @returns Fonction de désinscription
 */
export function onSessionCompleteCallback(callback: SessionCompleteCallback): () => void {
	completeListeners.add(callback);
	return () => completeListeners.delete(callback);
}

/**
 * Obtient le type de la prochaine session
 *
 * @returns Type de la prochaine session suggérée
 */
export function getNextSessionType(): SessionType {
	const state = timerStore.getState();
	const settings = settingsStore.settings;

	if (state.sessionType === 'work') {
		const shouldBeLongBreak =
			state.pomodoroCount % settings.pomodorosUntilLongBreak === 0 && state.pomodoroCount > 0;

		return shouldBeLongBreak ? 'long_break' : 'short_break';
	}

	return 'work';
}
