<script lang="ts">
	// Vue principale des résumés et statistiques
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { invoke } from '@tauri-apps/api/core';
	import SummaryCard from './SummaryCard.svelte';
	import BarChart from '$lib/components/stats/BarChart.svelte';
	import DonutChart from '$lib/components/stats/DonutChart.svelte';
	import Heatmap from '$lib/components/stats/Heatmap.svelte';
	import AnimatedIcon from '$lib/components/ui/AnimatedIcon.svelte';
	import {
		getTodaySummary,
		getThisWeekSummary,
		aggregateSummaries,
		formatFocusTime
	} from '$lib/services/summary-service';
	import { getFocusHistory } from '$lib/services/stats-service';
	import type { DailySummary } from '$lib/types';

	// Types pour les graphiques
	interface DailyFocusTime {
		date: string;
		minutes: number;
	}

	interface ProjectDistribution {
		projectName: string;
		color: string | null;
		minutes: number;
		percentage: number;
	}

	// État local
	let period = $state<'daily' | 'weekly'>('daily');
	let isLoading = $state(true);
	let dailySummary = $state<DailySummary | null>(null);
	let weeklySummaries = $state<DailySummary[]>([]);

	// État pour les graphiques
	let dailyFocus = $state<DailyFocusTime[]>([]);
	let projectDist = $state<ProjectDistribution[]>([]);
	let historyData = $state<DailyFocusTime[]>([]);

	// Données affichées selon la période sélectionnée
	const displayData = $derived(() => {
		if (period === 'daily' && dailySummary) {
			return {
				completedTasks: dailySummary.completedTasksCount,
				completedPomodoros: dailySummary.completedPomodorosCount,
				focusMinutes: dailySummary.totalFocusMinutes
			};
		} else if (period === 'weekly') {
			const aggregated = aggregateSummaries(weeklySummaries);
			return {
				completedTasks: aggregated.totalCompletedTasks,
				completedPomodoros: aggregated.totalCompletedPomodoros,
				focusMinutes: aggregated.totalFocusMinutes
			};
		}
		return { completedTasks: 0, completedPomodoros: 0, focusMinutes: 0 };
	});

	// Formate le temps de focus
	const formattedFocusTime = $derived(() => {
		const { hours, minutes } = formatFocusTime(displayData().focusMinutes);
		if (hours > 0) {
			return $_('summary.hours', { values: { count: hours, minutes } });
		}
		return $_('summary.minutes', { values: { count: minutes } });
	});

	// Données formatées pour les graphiques
	let barChartData = $derived(
		dailyFocus.map((d) => ({
			label: new Date(d.date).toLocaleDateString(undefined, { weekday: 'short' }),
			value: d.minutes
		}))
	);

	let donutChartData = $derived(
		projectDist.map((p) => ({
			label: p.projectName,
			value: p.minutes,
			percentage: p.percentage,
			color: p.color || '#64748b' // Default slate-500
		}))
	);

	/**
	 * Charge les données de résumé
	 */
	async function loadSummaries() {
		isLoading = true;
		try {
			// Charge toutes les données en parallèle
			const [today, week, focus, dist, history] = await Promise.all([
				getTodaySummary(),
				getThisWeekSummary(),
				invoke('get_daily_focus_time') as Promise<DailyFocusTime[]>,
				invoke('get_project_distribution') as Promise<ProjectDistribution[]>,
				getFocusHistory(365)
			]);

			dailySummary = today;
			weeklySummaries = week;
			dailyFocus = focus;
			projectDist = dist;
			historyData = history;
		} catch (error) {
			console.error('Failed to load summaries:', error);
		} finally {
			isLoading = false;
		}
	}

	/**
	 * Change la période affichée
	 */
	function setPeriod(newPeriod: 'daily' | 'weekly') {
		period = newPeriod;
	}

	onMount(() => {
		loadSummaries();

		// Rafraîchit les données toutes les 60 secondes
		const interval = setInterval(loadSummaries, 60000);
		return () => clearInterval(interval);
	});
</script>

<div class="flex flex-col gap-8">
	<!-- En-tête avec sélecteur de période -->
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-3">
			<AnimatedIcon name="stats" size={48} />
			<h2 class="text-2xl font-bold">{$_('summary.title')}</h2>
		</div>

		<!-- Toggle daily/weekly -->
		<div class="flex gap-2">
			<button
				type="button"
				onclick={() => setPeriod('daily')}
				class="rounded-md px-4 py-2 text-sm font-medium transition-colors"
				class:bg-primary={period === 'daily'}
				class:text-primary-foreground={period === 'daily'}
				class:bg-muted={period !== 'daily'}
			>
				{$_('summary.daily')}
			</button>

			<button
				type="button"
				onclick={() => setPeriod('weekly')}
				class="rounded-md px-4 py-2 text-sm font-medium transition-colors"
				class:bg-primary={period === 'weekly'}
				class:text-primary-foreground={period === 'weekly'}
				class:bg-muted={period !== 'weekly'}
			>
				{$_('summary.weekly')}
			</button>
		</div>
	</div>

	<!-- Cartes de statistiques -->
	{#if isLoading}
		<div class="text-muted-foreground py-8 text-center">
			{$_('common.loading')}
		</div>
	{:else if displayData().completedTasks === 0 && displayData().completedPomodoros === 0 && dailyFocus.length === 0}
		<div class="text-muted-foreground py-8 text-center">
			<p>{$_('summary.noData')}</p>
		</div>
	{:else}
		<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
			<!-- Tâches complétées -->
			<SummaryCard
				title={$_('summary.completedTasks')}
				value={displayData().completedTasks}
				icon="✅"
				color="success"
			/>

			<!-- Pomodoros complétés -->
			<SummaryCard
				title={$_('summary.completedPomodoros')}
				value={displayData().completedPomodoros}
				icon="🍅"
				color="primary"
			/>

			<!-- Temps de focus -->
			<SummaryCard
				title={$_('summary.focusTime')}
				value={formattedFocusTime()}
				icon="⏱️"
				color="warning"
			/>
		</div>

		<!-- Graphiques -->
		<div class="grid gap-6 md:grid-cols-2">
			<BarChart title={$_('summary.focusActivity')} data={barChartData} color="bg-orange-500" />

			<DonutChart title={$_('summary.projectDistribution')} data={donutChartData} />
		</div>

		<!-- Heatmap -->
		<div class="mt-4">
			<Heatmap data={historyData} />
		</div>
	{/if}
</div>
