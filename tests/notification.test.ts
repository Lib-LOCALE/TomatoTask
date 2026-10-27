import { beforeEach, describe, expect, it, vi } from 'vitest';
import { installTauriMock } from '../src/lib/dev/tauri-mock';
import { initializeI18n } from '../src/lib/services/i18n-service';
import { settingsStore } from '../src/lib/stores/settings.svelte';
import { timerStore } from '../src/lib/stores/timer.svelte';
import { notifySessionComplete } from '../src/lib/services/notification-service';

const notification = vi.hoisted(() => vi.fn());
vi.mock('@tauri-apps/plugin-notification', () => ({
	isPermissionGranted: vi.fn().mockResolvedValue(true),
	requestPermission: vi.fn().mockResolvedValue('granted'),
	sendNotification: notification
}));

describe('notifications de session', () => {
	beforeEach(async () => {
		installTauriMock(true);
		await settingsStore.load();
		await initializeI18n('fr');
		timerStore.sessionType = 'work';
		timerStore.pomodoroCount = 4;
		notification.mockClear();
	});
	it('indique le numéro du carreau et la durée de la prochaine pause longue', async () => {
		await notifySessionComplete('work');
		expect(notification).toHaveBeenCalledWith(
			expect.objectContaining({
				body: 'Carreau n° 0 émaillé — pause de 15 min'
			})
		);
	});
	it('annonce la fin de pause dans la langue active', async () => {
		await notifySessionComplete('short_break');
		expect(notification).toHaveBeenCalledWith(
			expect.objectContaining({
				body: 'Pause terminée — prêt à vous concentrer ?'
			})
		);
	});
});
