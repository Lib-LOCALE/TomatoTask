// Fonctions de validation pour les entrées utilisateur

/**
 * Valide un titre de tâche
 *
 * @param title - Titre à valider
 * @returns true si valide, message d'erreur sinon
 */
export function validateTaskTitle(title: string): true | string {
	const trimmed = title.trim();

	if (trimmed.length === 0) {
		return 'Task title cannot be empty';
	}

	if (trimmed.length > 200) {
		return 'Task title must be 200 characters or less';
	}

	return true;
}

/**
 * Valide une durée de timer en minutes
 *
 * @param duration - Durée en minutes
 * @param min - Minimum autorisé (défaut: 1)
 * @param max - Maximum autorisé (défaut: 180)
 * @returns true si valide, message d'erreur sinon
 */
export function validateTimerDuration(
	duration: number,
	min: number = 1,
	max: number = 180
): true | string {
	if (!Number.isInteger(duration)) {
		return 'Duration must be a whole number';
	}

	if (duration < min) {
		return `Duration must be at least ${min} minute${min > 1 ? 's' : ''}`;
	}

	if (duration > max) {
		return `Duration must be ${max} minutes or less`;
	}

	return true;
}

/**
 * Valide un nombre de Pomodoros estimés
 *
 * @param count - Nombre de Pomodoros
 * @returns true si valide, message d'erreur sinon
 */
export function validatePomodoroCount(count: number): true | string {
	if (!Number.isInteger(count)) {
		return 'Pomodoro count must be a whole number';
	}

	if (count < 0) {
		return 'Pomodoro count cannot be negative';
	}

	if (count > 20) {
		return 'Pomodoro count must be 20 or less';
	}

	return true;
}

/**
 * Valide un nom de projet
 *
 * @param name - Nom du projet
 * @returns true si valide, message d'erreur sinon
 */
export function validateProjectName(name: string): true | string {
	const trimmed = name.trim();

	if (trimmed.length === 0) {
		return 'Project name cannot be empty';
	}

	if (trimmed.length > 100) {
		return 'Project name must be 100 characters or less';
	}

	return true;
}

/**
 * Valide un code couleur hexadécimal
 *
 * @param color - Code couleur (format #RRGGBB)
 * @returns true si valide, message d'erreur sinon
 */
export function validateColorHex(color: string): true | string {
	const hexColorRegex = /^#[0-9A-Fa-f]{6}$/;

	if (!hexColorRegex.test(color)) {
		return 'Color must be in hex format (#RRGGBB)';
	}

	return true;
}

/**
 * Valide une date au format ISO (YYYY-MM-DD)
 *
 * @param dateStr - Date au format string
 * @returns true si valide, message d'erreur sinon
 */
export function validateISODate(dateStr: string): true | string {
	const isoDateRegex = /^\d{4}-\d{2}-\d{2}$/;

	if (!isoDateRegex.test(dateStr)) {
		return 'Date must be in ISO format (YYYY-MM-DD)';
	}

	const date = new Date(dateStr);
	if (isNaN(date.getTime())) {
		return 'Invalid date';
	}

	return true;
}

/**
 * Nettoie et normalise une chaîne de caractères
 *
 * @param str - Chaîne à nettoyer
 * @returns Chaîne nettoyée (trim + espaces multiples réduits)
 */
export function sanitizeString(str: string): string {
	return str.trim().replace(/\s+/g, ' ');
}
