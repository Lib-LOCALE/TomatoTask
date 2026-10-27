import { describe, expect, it } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { installTauriMock } from '../src/lib/dev/tauri-mock';

describe('backend navigateur', () => {
	it('crée une tâche et enregistre une session terminée', async () => {
		installTauriMock(true);
		const task = await invoke<{ id: number; completedPomodoros: number }>('create_task', {
			input: { title: 'Écrire', estimatedPomodoros: 2 }
		});
		const session = await invoke<{ id: number }>('create_session', {
			taskId: task.id,
			durationMinutes: 25,
			sessionType: 'work'
		});
		await invoke('complete_session', { id: session.id });
		expect(
			(await invoke<{ completedPomodoros: number }>('get_task', { id: task.id })).completedPomodoros
		).toBe(1);
	});
});

it('planifie, reporte et réordonne les tâches du jour', async () => {
	installTauriMock(true);
	const a = await invoke<{ id: number }>('create_task', {
		input: { title: 'A', estimatedPomodoros: 2 }
	});
	const b = await invoke<{ id: number }>('create_task', {
		input: { title: 'B', estimatedPomodoros: 3 }
	});
	const today = new Date().toLocaleDateString('en-CA');
	const yesterday = new Date();
	yesterday.setDate(yesterday.getDate() - 1);
	await invoke('plan_task', { id: a.id, date: yesterday.toLocaleDateString('en-CA') });
	await invoke('plan_task', { id: b.id, date: today });
	await invoke('reorder_plan', { date: today, taskIds: [b.id, a.id] });
	const plan = await invoke<{
		tasks: { id: number; carriedOver: boolean }[];
		planned_pomodoros: number;
	}>('get_day_plan', { date: today });
	expect(plan.tasks.map((t) => [t.id, t.carriedOver])).toEqual([
		[b.id, false],
		[a.id, true]
	]);
	expect(plan.planned_pomodoros).toBe(5);
	await expect(invoke('reorder_plan', { date: today, taskIds: [b.id, b.id] })).rejects.toThrow();
});
