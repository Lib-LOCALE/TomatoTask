// Point d'entrée de l'application TomatoTask
import { mount } from 'svelte';
import './app.css';
import App from './App.svelte';
import { initializeI18n } from '$lib/services/i18n-service';
import { initKeyboardShortcuts } from '$lib/utils/keyboard';

/**
 * Initialise l'application de manière asynchrone
 *
 * 1. Charge les paramètres depuis Tauri (pour la langue sauvegardée)
 * 2. Initialise le système i18n
 * 3. Monte le composant App
 * 4. Initialise les raccourcis clavier globaux
 */
async function bootstrap() {
	try {
		// Charge les paramètres depuis Tauri pour obtenir la langue et le thème sauvegardés
		// Note: On importe dynamiquement pour éviter les erreurs si Tauri n'est pas disponible
		const { invoke } = await import('@tauri-apps/api/core');
		const settings = await invoke('get_settings');

		// Initialise i18n avec la langue sauvegardée
		await initializeI18n(settings.language);

		// Applique le thème sauvegardé
		if (settings.theme === 'dark') {
			document.documentElement.classList.add('dark');
		}
	} catch (error) {
		// Si erreur (mode dev sans Tauri), initialise avec détection auto
		console.warn('Failed to load settings from Tauri, using auto-detection:', error);
		await initializeI18n();
	}

	// Monte l'application Svelte
	const app = mount(App, {
		target: document.getElementById('app')!
	});

	// Initialise les raccourcis clavier globaux
	initKeyboardShortcuts();

	return app;
}

// Lance l'application
bootstrap().catch((error) => {
	console.error('Failed to bootstrap application:', error);
});
