// Application du thème clair / sombre au document
import type { Theme } from '$lib/types';

const STORAGE_KEY = 'tomatotask_theme';

function systemPrefersDark(): boolean {
	return window.matchMedia('(prefers-color-scheme: dark)').matches;
}

/**
 * Indique si le thème donné s'affiche en sombre
 */
export function isDarkTheme(theme: Theme): boolean {
	return theme === 'dark' || (theme === 'system' && systemPrefersDark());
}

/**
 * Applique le thème au document et le mémorise localement
 *
 * La copie locale permet d'afficher le bon thème dès le démarrage, avant même
 * que les paramètres soient lus depuis la base de données.
 */
export function applyTheme(theme: Theme): void {
	const dark = isDarkTheme(theme);
	const root = document.documentElement;
	root.classList.toggle('dark', dark);
	// Les contrôles natifs (listes déroulantes, barres de défilement) suivent le thème
	root.style.colorScheme = dark ? 'dark' : 'light';
	try {
		localStorage.setItem(STORAGE_KEY, theme);
	} catch {
		// Stockage indisponible : le thème reste appliqué pour la session
	}
}

/**
 * Thème mémorisé localement lors de la dernière utilisation
 */
export function getStoredTheme(): Theme | null {
	try {
		const theme = localStorage.getItem(STORAGE_KEY);
		return theme === 'light' || theme === 'dark' || theme === 'system' ? theme : null;
	} catch {
		return null;
	}
}
