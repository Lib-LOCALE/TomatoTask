// Service de gestion des notifications desktop
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import type { SessionType } from '$lib/types';

/**
 * Vérifie et demande la permission pour les notifications
 *
 * @returns true si permission accordée
 */
export async function ensureNotificationPermission(): Promise<boolean> {
	try {
		let permission = await isPermissionGranted();

		if (!permission) {
			const permissionResponse = await requestPermission();
			permission = permissionResponse === 'granted';
		}

		return permission;
	} catch (error) {
		console.error('Failed to check/request notification permission:', error);
		return false;
	}
}

/**
 * Envoie une notification desktop
 *
 * @param title - Titre de la notification
 * @param body - Corps de la notification
 */
export async function showNotification(title: string, body: string): Promise<void> {
	try {
		const permission = await ensureNotificationPermission();

		if (permission) {
			sendNotification({
				title,
				body,
				sound: 'default'
			});
		} else {
			console.warn('Notification permission denied');
		}
	} catch (error) {
		console.error('Failed to send notification:', error);
	}
}

/**
 * Envoie une notification de complétion de session
 *
 * @param sessionType - Type de session terminée
 */
export async function notifySessionComplete(sessionType: SessionType): Promise<void> {
	let title: string;
	let body: string;

	switch (sessionType) {
		case 'work':
			title = '🍅 Work Session Complete!';
			body = 'Great job! Time for a well-deserved break.';
			break;
		case 'short_break':
			title = '☕ Short Break Over';
			body = 'Break time is over. Ready to focus again?';
			break;
		case 'long_break':
			title = '🎉 Long Break Over';
			body = 'Refreshed and ready? Let\'s get back to work!';
			break;
	}

	await showNotification(title, body);
}

/**
 * Joue un son de notification
 *
 * Utilise l'API Audio HTML5 pour jouer un son local
 *
 * @param soundName - Nom du fichier son (défaut: 'notification')
 */
export function playNotificationSound(soundName: string = 'notification'): void {
	try {
		const audio = new Audio(`/sounds/${soundName}.mp3`);
		audio.volume = 0.5; // 50% volume
		audio.play().catch((error) => {
			console.error('Failed to play notification sound:', error);
		});
	} catch (error) {
		console.error('Failed to create audio element:', error);
	}
}

/**
 * Notification complète: son + desktop notification
 *
 * @param sessionType - Type de session terminée
 */
export async function notifyComplete(sessionType: SessionType): Promise<void> {
	// Joue le son approprié
	const soundName = sessionType === 'work' ? 'squashed_tomato' : 'notification';
	playNotificationSound(soundName);

	// Envoie la notification desktop
	await notifySessionComplete(sessionType);
}
