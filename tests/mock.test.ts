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
