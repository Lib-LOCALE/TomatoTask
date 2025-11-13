// Store réactif pour les paramètres de l'application (Svelte 5 Runes)
import { invoke } from '@tauri-apps/api/core';
import type { Settings } from '$lib/types';

/**
 * Paramètres par défaut (fallback si Tauri échoue)
 */
const DEFAULT_SETTINGS: Settings = {
	workDuration: 25,
	shortBreakDuration: 5,
	longBreakDuration: 15,
	pomosorosUntilLongBreak: 4,
	language: 'en',
	theme: 'light',
	notificationSound: 'default',
	autoStartBreaks: false,
	autoStartPomodoros: false
};

/**
 * Store des paramètres de l'application
 *
 * Charge les paramètres depuis Tauri et permet de les modifier
 */
class SettingsStore {
	// État mutable
	settings = $state<Settings>({ ...DEFAULT_SETTINGS });
	isLoading = $state(true);
	error = $state<string | null>(null);

	/**
	 * Charge les paramètres depuis Tauri
	 */
	async load(): Promise<void> {
		this.isLoading = true;
		this.error = null;

		try {
			const settings = await invoke<Settings>('get_settings');
			this.settings = settings;
		} catch (err) {
			console.error('Failed to load settings:', err);
			this.error = 'Failed to load settings';
			// Garde les paramètres par défaut
		} finally {
			this.isLoading = false;
		}
	}

	/**
	 * Sauvegarde les paramètres dans Tauri
	 */
	async save(newSettings: Settings): Promise<void> {
		try {
			const updated = await invoke<Settings>('update_settings', { settings: newSettings });
			this.settings = updated;
			this.error = null;
		} catch (err) {
			console.error('Failed to save settings:', err);
			this.error = 'Failed to save settings';
			throw err;
		}
	}

	/**
	 * Met à jour un paramètre spécifique
	 */
	async updateSetting<K extends keyof Settings>(
		key: K,
		value: Settings[K]
	): Promise<void> {
		const newSettings = { ...this.settings, [key]: value };
		await this.save(newSettings);
	}

	/**
	 * Réinitialise aux paramètres par défaut
	 */
	async reset(): Promise<void> {
		await this.save(DEFAULT_SETTINGS);
	}
}

// Instance singleton exportée
export const settingsStore = new SettingsStore();
