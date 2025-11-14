// Service d'internationalisation avec svelte-i18n
import { register, init, locale, getLocaleFromNavigator } from 'svelte-i18n';
import type { Language } from '$lib/types';

// Enregistre tous les fichiers de traduction
register('en', () => import('$lib/i18n/en.json'));
register('fr', () => import('$lib/i18n/fr.json'));
register('es', () => import('$lib/i18n/es.json'));
register('it', () => import('$lib/i18n/it.json'));
register('de', () => import('$lib/i18n/de.json'));

/**
 * Langues supportées par l'application
 */
export const SUPPORTED_LANGUAGES: Language[] = ['en', 'fr', 'es', 'it', 'de'];

/**
 * Détecte la langue préférée de l'utilisateur
 *
 * Ordre de priorité:
 * 1. Paramètres sauvegardés (via Tauri settings)
 * 2. Langue du navigateur
 * 3. Anglais par défaut
 *
 * @param savedLanguage - Langue sauvegardée (optionnel)
 * @returns Langue à utiliser
 */
export function detectUserLanguage(savedLanguage?: string): Language {
	// Si une langue est sauvegardée, l'utiliser
	if (savedLanguage && SUPPORTED_LANGUAGES.includes(savedLanguage as Language)) {
		return savedLanguage as Language;
	}

	// Sinon, détecter depuis le navigateur
	const browserLang = getLocaleFromNavigator();

	if (browserLang) {
		// Extrait le code langue (ex: 'fr-FR' -> 'fr')
		const langCode = browserLang.split('-')[0];

		if (SUPPORTED_LANGUAGES.includes(langCode as Language)) {
			return langCode as Language;
		}
	}

	// Par défaut: anglais
	return 'en';
}

/**
 * Initialise le système i18n
 *
 * @param initialLanguage - Langue initiale (optionnel, détection auto sinon)
 */
export async function initializeI18n(initialLanguage?: string): Promise<void> {
	const userLanguage = detectUserLanguage(initialLanguage);

	await init({
		fallbackLocale: 'en',
		initialLocale: userLanguage
	});
}

/**
 * Change la langue de l'application
 *
 * @param newLanguage - Nouvelle langue
 */
export function changeLanguage(newLanguage: Language): void {
	locale.set(newLanguage);
}

/**
 * Obtient les informations de locale pour une langue
 *
 * @param lang - Code langue
 * @returns Nom complet de la langue
 */
export function getLanguageName(lang: Language): string {
	const names: Record<Language, string> = {
		en: 'English',
		fr: 'Français',
		es: 'Español',
		it: 'Italiano',
		de: 'Deutsch'
	};

	return names[lang];
}

/**
 * Obtient le code langue court pour affichage
 *
 * @param lang - Code langue
 * @returns Code court (EN, FR, etc.)
 */
export function getLanguageCode(lang: Language): string {
	return lang.toUpperCase();
}

/**
 * Obtient le drapeau emoji pour une langue
 *
 * @param lang - Code langue
 * @returns Emoji drapeau
 */
export function getLanguageFlag(lang: Language): string {
	const flags: Record<Language, string> = {
		en: '🇬🇧', // Drapeau Royaume-Uni
		fr: '🇫🇷', // Drapeau France
		es: '🇪🇸', // Drapeau Espagne
		it: '🇮🇹', // Drapeau Italie
		de: '🇩🇪'  // Drapeau Allemagne
	};

	return flags[lang];
}

/**
 * Obtient le chemin vers l'image du drapeau pour une langue
 *
 * @param lang - Code langue
 * @returns Chemin vers l'image du drapeau
 */
export function getLanguageFlagPath(lang: Language): string {
	return `/flag-${lang}.png`;
}
