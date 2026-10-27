import { beforeEach, describe, expect, it, vi } from 'vitest';
import { installTauriMock } from '../src/lib/dev/tauri-mock';
import { taskStore } from '../src/lib/stores/tasks.svelte';
import { projectStore } from '../src/lib/stores/projects.svelte';
import { settingsStore } from '../src/lib/stores/settings.svelte';
import { timerStore } from '../src/lib/stores/timer.svelte';
import { toastStore } from '../src/lib/stores/toast.svelte';
import {
	getNextSessionType,
	pauseTimer,
	resumeTimer,
	startSession,
	stopTimer
} from '../src/lib/services/timer-service';

describe('stores et minuteur', () => {
	beforeEach(() => {
		installTauriMock(true);
		taskStore.clearFilters();
		timerStore.stop();
		timerStore.resetPomodoroCount();
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
});
