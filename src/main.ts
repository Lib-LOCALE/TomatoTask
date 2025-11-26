// Point d'entrée de l'application TomatoTask
import { mount } from 'svelte';
import './app.css';
import App from './App.svelte';
import { initializeI18n } from '$lib/services/i18n-service';
import { initKeyboardShortcuts } from '$lib/utils/keyboard';

/**
 * Affiche un écran de chargement pendant l'initialisation
 */
function showLoadingScreen() {
	const appElement = document.getElementById('app')!;
	appElement.innerHTML = `
		<div style="
			display: flex;
			align-items: center;
			justify-content: center;
			height: 100vh;
			background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
			font-family: system-ui, -apple-system, sans-serif;
		">
			<div style="text-align: center; color: white;">
				<div style="font-size: 72px; margin-bottom: 24px; animation: pulse 2s ease-in-out infinite;">
					🍅
				</div>
				<div style="font-size: 24px; font-weight: 600; margin-bottom: 8px;">
					TomatoTask
				</div>
				<div style="font-size: 14px; opacity: 0.9;">
					Loading your productivity companion...
				</div>
			</div>
		</div>
		<style>
			@keyframes pulse {
				0%, 100% { transform: scale(1); }
				50% { transform: scale(1.1); }
			}
		</style>
	`;
}

/**
 * Initialise l'application de manière asynchrone
 *
 * 1. Affiche un écran de chargement
 * 2. Charge les paramètres depuis Tauri (pour la langue sauvegardée)
 * 3. Initialise le système i18n
 * 4. Monte le composant App
 * 5. Initialise les raccourcis clavier globaux
 */
async function bootstrap() {
	// Affiche l'écran de chargement immédiatement
	showLoadingScreen();

	try {
		// Charge les paramètres depuis Tauri pour obtenir la langue et le thème sauvegardés
		// Note: On importe dynamiquement pour éviter les erreurs si Tauri n'est pas disponible
		const { invoke } = await import('@tauri-apps/api/core');
		const settings = await invoke<{ language: string; theme: string }>('get_settings');

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

	// Nettoie l'écran de chargement
	const appElement = document.getElementById('app')!;
	appElement.innerHTML = '';

	// Monte l'application Svelte
	const app = mount(App, {
		target: appElement
	});

	// Initialise les raccourcis clavier globaux
	initKeyboardShortcuts();

	return app;
}

// Lance l'application
bootstrap().catch((error) => {
	console.error('Failed to bootstrap application:', error);

	// Affiche un message d'erreur à l'utilisateur
	const appElement = document.getElementById('app')!;
	appElement.innerHTML = `
		<div style="
			display: flex;
			align-items: center;
			justify-content: center;
			height: 100vh;
			background: #1a1a1a;
			color: white;
			font-family: system-ui, -apple-system, sans-serif;
		">
			<div style="text-align: center; max-width: 500px; padding: 32px;">
				<div style="font-size: 48px; margin-bottom: 24px;">⚠️</div>
				<div style="font-size: 20px; font-weight: 600; margin-bottom: 16px;">
					Failed to start TomatoTask
				</div>
				<div style="font-size: 14px; opacity: 0.7; margin-bottom: 24px;">
					${error.message || 'Unknown error'}
				</div>
				<div style="font-size: 12px; opacity: 0.5;">
					Please check the console for more details
				</div>
			</div>
		</div>
	`;
});
