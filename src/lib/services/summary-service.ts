// Service de gestion des résumés et statistiques
import { invoke } from '@tauri-apps/api/core';
import type { DailySummary } from '$lib/types';

/**
 * Récupère le résumé quotidien pour une date donnée
 *
 * @param date - Date au format ISO (YYYY-MM-DD)
 * @returns Résumé quotidien
 */
export async function getDailySummary(date: string): Promise<DailySummary> {
	return await invoke<DailySummary>('get_daily_summary', { date });
}

/**
 * Récupère les résumés pour une plage de dates (hebdomadaire)
 *
 * @param startDate - Date de début au format ISO (YYYY-MM-DD)
 * @param endDate - Date de fin au format ISO (YYYY-MM-DD)
 * @returns Liste des résumés quotidiens
 */
export async function getWeeklySummary(startDate: string, endDate: string): Promise<DailySummary[]> {
	return await invoke<DailySummary[]>('get_weekly_summary', { startDate, endDate });
}

/**
 * Récupère le résumé d'aujourd'hui
 *
 * @returns Résumé d'aujourd'hui
 */
export async function getTodaySummary(): Promise<DailySummary> {
	const today = new Date().toISOString().split('T')[0];
	return await getDailySummary(today);
}

/**
 * Récupère le résumé de la semaine en cours (7 derniers jours)
 *
 * @returns Liste des résumés des 7 derniers jours
 */
export async function getThisWeekSummary(): Promise<DailySummary[]> {
	const endDate = new Date();
	const startDate = new Date();
	startDate.setDate(endDate.getDate() - 6); // 7 jours incluant aujourd'hui

	return await getWeeklySummary(
		startDate.toISOString().split('T')[0],
		endDate.toISOString().split('T')[0]
	);
}

/**
 * Agrège les résumés quotidiens en un résumé total
 *
 * @param summaries - Liste des résumés quotidiens
 * @returns Résumé agrégé
 */
export function aggregateSummaries(summaries: DailySummary[]): {
	totalCompletedTasks: number;
	totalCompletedPomodoros: number;
	totalFocusMinutes: number;
} {
	return summaries.reduce(
		(acc, summary) => ({
			totalCompletedTasks: acc.totalCompletedTasks + summary.completedTasksCount,
			totalCompletedPomodoros: acc.totalCompletedPomodoros + summary.completedPomodorosCount,
			totalFocusMinutes: acc.totalFocusMinutes + summary.totalFocusMinutes
		}),
		{
			totalCompletedTasks: 0,
			totalCompletedPomodoros: 0,
			totalFocusMinutes: 0
		}
	);
}

/**
 * Formate les minutes en heures et minutes
 *
 * @param minutes - Nombre de minutes
 * @returns Chaîne formatée "Xh Ymin"
 */
export function formatFocusTime(minutes: number): { hours: number; minutes: number } {
	const hours = Math.floor(minutes / 60);
	const remainingMinutes = minutes % 60;
	return { hours, minutes: remainingMinutes };
}
