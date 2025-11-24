// Service de gestion des notifications desktop
// NOTE: Notifications desktop temporairement désactivées en attendant la configuration correcte du plugin
import type { SessionType } from '$lib/types';

/**
 * Vérifie et demande la permission pour les notifications
 * STUB: Retourne toujours false temporairement
 *
 * @returns true si permission accordée
 */
export async function ensureNotificationPermission(): Promise<boolean> {
	// TODO: Réactiver quand le plugin notification sera configuré
	console.log('Notifications desktop temporairement désactivées');
	return false;
}

/**
 * Envoie une notification desktop
 * STUB: Ne fait rien temporairement
 *
 * @param title - Titre de la notification
 * @param body - Corps de la notification
 */
export async function showNotification(title: string, body: string): Promise<void> {
	// TODO: Réactiver quand le plugin notification sera configuré
	console.log('Notification:', title, body);
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

	// Envoie la notification desktop (désactivée temporairement)
	await notifySessionComplete(sessionType);
}
