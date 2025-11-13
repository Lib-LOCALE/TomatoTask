<script lang="ts">
	// Vue principale des résumés et statistiques
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import SummaryCard from './SummaryCard.svelte';
	import {
		getTodaySummary,
		getThisWeekSummary,
		aggregateSummaries,
		formatFocusTime
	} from '$lib/services/summary-service';
	import type { DailySummary } from '$lib/types';

	// État local
	let period = $state<'daily' | 'weekly'>('daily');
	let isLoading = $state(true);
	let dailySummary = $state<DailySummary | null>(null);
	let weeklySummaries = $state<DailySummary[]>([]);

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

	/**
	 * Charge les données de résumé
	 */
	async function loadSummaries() {
		isLoading = true;
		try {
			// Charge les deux types de données
			const [today, week] = await Promise.all([
				getTodaySummary(),
				getThisWeekSummary()
			]);

			dailySummary = today;
			weeklySummaries = week;
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

<div class="flex flex-col gap-4">
	<!-- En-tête avec sélecteur de période -->
	<div class="flex items-center justify-between">
		<h2 class="text-2xl font-bold">{$_('summary.title')}</h2>

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
		<div class="text-center py-8 text-muted-foreground">
			{$_('common.loading')}
		</div>
	{:else if displayData().completedTasks === 0 && displayData().completedPomodoros === 0}
		<div class="text-center py-8 text-muted-foreground">
			<p>{$_('summary.noData')}</p>
		</div>
	{:else}
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
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
	{/if}

	<!-- Graphique hebdomadaire (placeholder pour futures améliorations) -->
	{#if period === 'weekly' && weeklySummaries.length > 0}
		<div class="mt-4 rounded-lg border bg-card p-4">
			<h3 class="mb-4 text-lg font-semibold">Weekly Breakdown</h3>
			<div class="space-y-2">
				{#each weeklySummaries as daySummary}
					<div class="flex items-center justify-between text-sm">
						<span class="font-medium">{daySummary.date}</span>
						<div class="flex gap-4 text-muted-foreground">
							<span>✅ {daySummary.completedTasksCount}</span>
							<span>🍅 {daySummary.completedPomodorosCount}</span>
							<span>⏱️ {daySummary.totalFocusMinutes}min</span>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>
