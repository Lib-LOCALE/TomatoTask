// Utilitaires de formatage du temps et des dates

/**
 * Formate des secondes en format MM:SS
 *
 * @param seconds - Nombre de secondes
 * @returns Chaîne formatée (ex: "25:00", "05:30")
 */
export function formatSecondsToMMSS(seconds: number): string {
	const mins = Math.floor(seconds / 60);
	const secs = seconds % 60;
	return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}

/**
 * Formate des minutes en format lisible
 *
 * @param minutes - Nombre de minutes
 * @returns Chaîne formatée (ex: "25 min", "1h 30min")
 */
export function formatMinutes(minutes: number): string {
	if (minutes < 60) {
		return `${minutes} min`;
	}

	const hours = Math.floor(minutes / 60);
	const remainingMinutes = minutes % 60;

	if (remainingMinutes === 0) {
		return `${hours}h`;
	}

	return `${hours}h ${remainingMinutes}min`;
}

/**
 * Formate une date ISO en format localisé
 *
 * @param isoDate - Date au format ISO (YYYY-MM-DD ou ISO 8601 complet)
 * @param locale - Locale à utiliser (défaut: 'en')
 * @param options - Options de formatage Intl.DateTimeFormat
 * @returns Date formatée selon la locale
 */
export function formatDate(
	isoDate: string,
	locale: string = 'en',
	options: Intl.DateTimeFormatOptions = {
		year: 'numeric',
		month: 'short',
		day: 'numeric'
	}
): string {
	const date = new Date(isoDate);
	return new Intl.DateTimeFormat(locale, options).format(date);
}

/**
 * Formate une date/heure ISO complète
 *
 * @param isoDateTime - Date/heure au format ISO 8601
 * @param locale - Locale à utiliser (défaut: 'en')
 * @returns Date/heure formatée (ex: "Nov 13, 2025, 10:30 AM")
 */
export function formatDateTime(isoDateTime: string, locale: string = 'en'): string {
	const date = new Date(isoDateTime);
	return new Intl.DateTimeFormat(locale, {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
		hour: '2-digit',
		minute: '2-digit'
	}).format(date);
}

/**
 * Formate une durée relative (ex: "2 hours ago", "in 5 minutes")
 *
 * @param isoDateTime - Date/heure au format ISO 8601
 * @param locale - Locale à utiliser (défaut: 'en')
 * @returns Durée relative formatée
 */
export function formatRelativeTime(isoDateTime: string, locale: string = 'en'): string {
	const date = new Date(isoDateTime);
	const now = new Date();
	const diffMs = date.getTime() - now.getTime();
	const diffSecs = Math.floor(diffMs / 1000);
	const diffMins = Math.floor(diffSecs / 60);
	const diffHours = Math.floor(diffMins / 60);
	const diffDays = Math.floor(diffHours / 24);

	const rtf = new Intl.RelativeTimeFormat(locale, { numeric: 'auto' });

	if (Math.abs(diffDays) > 0) {
		return rtf.format(diffDays, 'day');
	}
	if (Math.abs(diffHours) > 0) {
		return rtf.format(diffHours, 'hour');
	}
	if (Math.abs(diffMins) > 0) {
		return rtf.format(diffMins, 'minute');
	}
	return rtf.format(diffSecs, 'second');
}

/**
 * Obtient la date d'aujourd'hui au format ISO (YYYY-MM-DD)
 *
 * @returns Date d'aujourd'hui
 */
export function getTodayISO(): string {
	const today = new Date();
	return today.toISOString().split('T')[0];
}

/**
 * Obtient une date ISO décalée de N jours
 *
 * @param daysOffset - Nombre de jours (positif = futur, négatif = passé)
 * @returns Date au format ISO (YYYY-MM-DD)
 */
export function getDateWithOffset(daysOffset: number): string {
	const date = new Date();
	date.setDate(date.getDate() + daysOffset);
	return date.toISOString().split('T')[0];
}

/**
 * Obtient le début et la fin de la semaine courante
 *
 * @returns Objet avec startDate et endDate au format ISO
 */
export function getCurrentWeekRange(): { startDate: string; endDate: string } {
	const today = new Date();
	const dayOfWeek = today.getDay(); // 0 (dimanche) à 6 (samedi)

	// Lundi de la semaine courante
	const monday = new Date(today);
	const daysToMonday = dayOfWeek === 0 ? -6 : 1 - dayOfWeek;
	monday.setDate(today.getDate() + daysToMonday);

	// Dimanche de la semaine courante
	const sunday = new Date(monday);
	sunday.setDate(monday.getDate() + 6);

	return {
		startDate: monday.toISOString().split('T')[0],
		endDate: sunday.toISOString().split('T')[0]
	};
}

/**
 * Calcule le pourcentage de progression
 *
 * @param completed - Nombre complété
 * @param total - Total
 * @returns Pourcentage (0-100)
 */
export function calculateProgress(completed: number, total: number): number {
	if (total === 0) return 0;
	return Math.round((completed / total) * 100);
}
