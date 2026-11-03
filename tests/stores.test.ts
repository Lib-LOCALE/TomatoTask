import { beforeEach, describe, expect, it, vi } from 'vitest';
import { installTauriMock } from '../src/lib/dev/tauri-mock';
import { taskStore } from '../src/lib/stores/tasks.svelte';
import { projectStore } from '../src/lib/stores/projects.svelte';
import { settingsStore } from '../src/lib/stores/settings.svelte';
import { timerStore } from '../src/lib/stores/timer.svelte';
import { toastStore } from '../src/lib/stores/toast.svelte';
import {
	getNextSessionType,
	onSessionCompleteCallback,
	pauseTimer,
	resumeTimer,
	skipSession,
	startSession,
	stopTimer,
	toggleTimer
} from '../src/lib/services/timer-service';

describe('stores et minuteur', () => {
	beforeEach(() => {
		installTauriMock(true);
		taskStore.clearFilters();
		timerStore.stop();
		timerStore.resetPomodoroCount();
		timerStore.nextType = 'work';
	});
	it('filtre les tâches par projet et par état', async () => {
		await taskStore.load();
		const a = await taskStore.create({ title: 'A', projectId: 1, estimatedPomodoros: 1 });
		await taskStore.create({ title: 'B', projectId: 2, estimatedPomodoros: 1 });
		await taskStore.toggleCompletion(a.id);
		taskStore.setProjectFilter(1);
		taskStore.setCompletionFilter('completed');
		expect(taskStore.filteredTasks.map((t) => t.title)).toEqual(['A']);
	});
	it('gère projets et paramètres', async () => {
		await projectStore.load();
		const p = await projectStore.create('Projet', '#123456');
		await projectStore.update(p.id, 'Modifié', '#abcdef');
		expect(projectStore.getById(p.id)?.name).toBe('Modifié');
		await settingsStore.load();
		await settingsStore.updateSetting('theme', 'dark');
		expect(settingsStore.settings.theme).toBe('dark');
		await projectStore.delete(p.id);
		expect(projectStore.projects).toHaveLength(0);
	});
	it('expire les messages', () => {
		vi.useFakeTimers();
		toastStore.clear();
		toastStore.success('Bravo', 100);
		expect(toastStore.toasts).toHaveLength(1);
		vi.advanceTimersByTime(400);
		expect(toastStore.toasts).toHaveLength(0);
		vi.useRealTimers();
	});
	it('met en pause et choisit la pause longue après quatre pomodoros', async () => {
		await settingsStore.load();
		await startSession('work');
		pauseTimer();
		expect(timerStore.isRunning).toBe(false);
		timerStore.pomodoroCount = 4;
		expect(getNextSessionType()).toBe('long_break');
		timerStore.pomodoroCount = 1;
		expect(getNextSessionType()).toBe('short_break');
		timerStore.sessionType = 'short_break';
		expect(getNextSessionType()).toBe('work');
		await stopTimer();
	});
	it('la reprise après une pause relance le décompte', async () => {
		vi.useFakeTimers();
		await settingsStore.load();
		await startSession('work');
		await vi.advanceTimersByTimeAsync(1000);
		pauseTimer();
		const remaining = timerStore.remainingSeconds;
		await vi.advanceTimersByTimeAsync(2000);
		expect(timerStore.remainingSeconds).toBe(remaining);
		resumeTimer();
		await vi.advanceTimersByTimeAsync(1000);
		expect(timerStore.remainingSeconds).toBe(remaining - 1);
		await stopTimer();
		vi.useRealTimers();
	});
	it('termine une session de travail et incrémente les compteurs', async () => {
		vi.useFakeTimers();
		await settingsStore.load();
		await settingsStore.updateSetting('workDuration', 1);
		const task = await taskStore.create({ title: 'Focus', estimatedPomodoros: 1 });
		await startSession('work', task.id);
		await vi.advanceTimersByTimeAsync(60000);
		expect(timerStore.pomodoroCount).toBe(1);
		await taskStore.load();
		expect(taskStore.tasks.find((item) => item.id === task.id)?.completedPomodoros).toBe(1);
		expect(getNextSessionType()).toBe('short_break');
		await stopTimer();
		vi.useRealTimers();
	});
	it("suit l'horloge réelle même si les rafraîchissements sont retardés", async () => {
		vi.useFakeTimers();
		await settingsStore.load();
		await startSession('work');
		const total = timerStore.totalSeconds;
		// Fenêtre masquée ou veille : l'horloge avance sans que l'intervalle ne tourne
		vi.setSystemTime(Date.now() + 90_000);
		await vi.advanceTimersByTimeAsync(250);
		expect(timerStore.remainingSeconds).toBe(total - 90);
		await stopTimer();
		vi.useRealTimers();
	});
	it('démarre la session proposée avec la tâche sélectionnée, puis bascule pause/reprise', async () => {
		await settingsStore.load();
		timerStore.nextType = 'short_break';
		await toggleTimer(42);
		expect(timerStore.sessionType).toBe('short_break');
		expect(timerStore.taskId).toBeUndefined();
		await toggleTimer();
		expect(timerStore.isRunning).toBe(false);
		await toggleTimer();
		expect(timerStore.isRunning).toBe(true);
		await stopTimer();
		timerStore.nextType = 'work';
		await toggleTimer(42);
		expect(timerStore.sessionType).toBe('work');
		expect(timerStore.taskId).toBe(42);
		await stopTimer();
	});
	it('passer une session de travail propose une pause', async () => {
		await settingsStore.load();
		await startSession('work');
		await skipSession();
		expect(timerStore.remainingSeconds).toBe(0);
		expect(timerStore.nextType).toBe('short_break');
		await skipSession();
		expect(timerStore.nextType).toBe('work');
	});
	it('propose la pause suivante et prévient les vues abonnées à la fin', async () => {
		vi.useFakeTimers();
		await settingsStore.load();
		await settingsStore.updateSetting('workDuration', 1);
		const listener = vi.fn();
		const unsubscribe = onSessionCompleteCallback(listener);
		await startSession('work');
		await vi.advanceTimersByTimeAsync(60_000);
		expect(listener).toHaveBeenCalledWith('work');
		expect(timerStore.nextType).toBe('short_break');
		unsubscribe();
		await startSession('short_break');
		await vi.advanceTimersByTimeAsync(5 * 60_000);
		expect(listener).toHaveBeenCalledTimes(1);
		expect(timerStore.nextType).toBe('work');
		vi.useRealTimers();
	});
	it("n'enchaîne pas automatiquement si une autre session a été lancée entre-temps", async () => {
		vi.useFakeTimers();
		await settingsStore.load();
		await settingsStore.updateSetting('workDuration', 1);
		await settingsStore.updateSetting('autoStartBreaks', true);
		await startSession('work');
		await vi.advanceTimersByTimeAsync(60_000);
		await startSession('long_break');
		await vi.advanceTimersByTimeAsync(3_000);
		expect(timerStore.sessionType).toBe('long_break');
		await settingsStore.updateSetting('autoStartBreaks', false);
		await stopTimer();
		vi.useRealTimers();
	});
});
