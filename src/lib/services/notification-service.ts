// Service de gestion des notifications desktop
import {
	isPermissionGranted,
	requestPermission,
	sendNotification
} from '@tauri-apps/plugin-notification';
import type { SessionType } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';
import { get } from 'svelte/store';
import { _, locale } from 'svelte-i18n';
import { settingsStore } from '$lib/stores/settings.svelte';
import { getNextSessionType } from '$lib/services/timer-service';

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
				sound: undefined
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
	const translate = get(_);
	let body: string;
	if (sessionType === 'work') {
		const next = getNextSessionType();
		const minutes =
			next === 'long_break'
				? settingsStore.settings.longBreakDuration
				: settingsStore.settings.shortBreakDuration;
		try {
			const streak = await invoke<{ total_pomodoros: number }>('get_focus_streak');
			body = translate('chrome.notification.tile', {
				values: {
					serial: new Intl.NumberFormat(get(locale) || 'en').format(streak.total_pomodoros),
					minutes
				}
			});
		} catch {
			body = translate('chrome.notification.work', { values: { minutes } });
		}
	} else {
		body = translate('chrome.notification.break');
	}
	await showNotification('TomatoTask', body);
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
	const soundName = settingsStore.settings.notificationSound;
	if (soundName !== 'none') playNotificationSound('squashed_tomato');

	// Envoie la notification desktop
	await notifySessionComplete(sessionType);
}
