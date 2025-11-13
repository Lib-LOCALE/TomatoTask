// Service de gestion du timer Pomodoro
import { invoke } from '@tauri-apps/api/core';
import { timerStore } from '$lib/stores/timer.svelte';
import { settingsStore } from '$lib/stores/settings.svelte';
import type { SessionType, PomodoroSession } from '$lib/types';

/**
 * Intervalle du timer (mis à jour chaque seconde)
 */
let timerInterval: number | null = null;

/**
 * Callback appelé quand une session se termine
 */
type SessionCompleteCallback = (sessionType: SessionType) => void;
let onSessionComplete: SessionCompleteCallback | null = null;

/**
 * Démarre une nouvelle session Pomodoro
 *
 * @param sessionType - Type de session (work, short_break, long_break)
 * @param taskId - ID de la tâche associée (optionnel, uniquement pour work)
 */
export async function startSession(
	sessionType: SessionType,
	taskId?: number
): Promise<void> {
	// Arrête toute session en cours
	stopTimer();

	// Détermine la durée selon le type et les paramètres
	const settings = settingsStore.settings;
	let duration: number;

	switch (sessionType) {
		case 'work':
			duration = settings.workDuration;
			break;
		case 'short_break':
			duration = settings.shortBreakDuration;
			break;
		case 'long_break':
			duration = settings.longBreakDuration;
			break;
	}

	try {
		// Crée la session dans la base de données
		const session = await invoke<PomodoroSession>('create_session', {
			taskId: sessionType === 'work' ? taskId : undefined,
			durationMinutes: duration,
			sessionType
		});

		// Démarre le timer dans le store
		timerStore.startSession(duration, sessionType, taskId, session.id);

		// Lance l'intervalle qui décrémente chaque seconde
		startInterval();
	} catch (error) {
		console.error('Failed to start session:', error);
		throw error;
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

		if (isComplete) {
			handleSessionComplete();
		}
	}, 1000);
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
	if (!timerStore.isRunning) {
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
	if (timerInterval !== null) {
		clearInterval(timerInterval);
		timerInterval = null;
	}

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
}

/**
 * Gère la complétion d'une session
 */
async function handleSessionComplete(): Promise<void> {
	// Arrête l'intervalle
	if (timerInterval !== null) {
		clearInterval(timerInterval);
		timerInterval = null;
	}

	const state = timerStore.getState();
	const sessionId = state.sessionId;

	if (sessionId) {
		try {
			// Marque la session comme complétée dans la DB
			await invoke('complete_session', { id: sessionId });

			// Si c'était une session de travail, incrémente le compteur
			if (state.sessionType === 'work') {
				timerStore.completeSession();
			}

			// Appelle le callback si défini
			if (onSessionComplete) {
				onSessionComplete(state.sessionType);
			}
		} catch (error) {
			console.error('Failed to complete session:', error);
		}
	}

	timerStore.stop();
}

/**
 * Démarre automatiquement la prochaine session
 *
 * Logique:
 * - Après work → short_break (ou long_break tous les N Pomodoros)
 * - Après break → work (si auto-start activé)
 */
export async function startNextSession(taskId?: number): Promise<void> {
	const state = timerStore.getState();
	const settings = settingsStore.settings;

	let nextType: SessionType;

	if (state.sessionType === 'work') {
		// Après work: break court ou long?
		const shouldBeLongBreak =
			state.pomodoroCount % settings.pomosorosUntilLongBreak === 0 &&
			state.pomodoroCount > 0;

		nextType = shouldBeLongBreak ? 'long_break' : 'short_break';
	} else {
		// Après break: retour au work
		nextType = 'work';
	}

	await startSession(nextType, taskId);
}

/**
 * Enregistre un callback pour la complétion de session
 *
 * @param callback - Fonction appelée quand une session se termine
 */
export function onSessionCompleteCallback(callback: SessionCompleteCallback): void {
	onSessionComplete = callback;
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
			state.pomodoroCount % settings.pomosorosUntilLongBreak === 0 &&
			state.pomodoroCount > 0;

		return shouldBeLongBreak ? 'long_break' : 'short_break';
	}

	return 'work';
}
