// Types TypeScript pour les paramètres et résumés

/**
 * Thème de l'application
 */
export type Theme = 'light' | 'dark';

/**
 * Langues supportées
 */
export type Language = 'en' | 'fr' | 'es' | 'it' | 'de';

/**
 * Paramètres de configuration de l'application
 */
export interface Settings {
	workDuration: number;
	shortBreakDuration: number;
	longBreakDuration: number;
	pomosorosUntilLongBreak: number;
	language: Language;
	theme: Theme;
	notificationSound: string;
	autoStartBreaks: boolean;
	autoStartPomodoros: boolean;
}

/**
 * Résumé quotidien de productivité
 */
export interface DailySummary {
	date: string;
	completedTasksCount: number;
	completedPomodorosCount: number;
	totalFocusMinutes: number;
}
