// Service de gestion des statistiques
import { invoke } from '@tauri-apps/api/core';
import type { DailyFocusTime, ProjectDistribution } from '$lib/types';

/**
 * Récupère le temps de focus quotidien pour les 7 derniers jours
 */
export async function getDailyFocusTime(): Promise<DailyFocusTime[]> {
	return await invoke<DailyFocusTime[]>('get_daily_focus_time');
}

/**
 * Récupère la distribution du temps par projet
 */
export async function getProjectDistribution(): Promise<ProjectDistribution[]> {
	return await invoke<ProjectDistribution[]>('get_project_distribution');
}

/**
 * Récupère l'historique de focus sur une période donnée
 *
 * @param days - Nombre de jours à récupérer (ex: 365 pour un an)
 */
export async function getFocusHistory(days: number): Promise<DailyFocusTime[]> {
	return await invoke<DailyFocusTime[]>('get_focus_history', { days });
}

export async function getInsights(days: number): Promise<import('$lib/types').Insights> {
	return invoke('get_insights', { days });
}

export async function getFocusStreak(): Promise<import('$lib/types').FocusStreak> {
	return invoke('get_focus_streak');
}

/** Numéros des carreaux d'une période, dans l'ordre chronologique. */
export function frescoSerials(counts: number[], lifetimeTotal: number): number[][] {
	let serial = lifetimeTotal - counts.reduce((sum, count) => sum + count, 0);
	return counts.map((count) => Array.from({ length: count }, () => ++serial));
}
