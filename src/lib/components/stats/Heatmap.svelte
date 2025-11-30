<script lang="ts">
	import { _ } from 'svelte-i18n';
	import type { DailyFocusTime } from '$lib/types';

	interface Props {
		data: DailyFocusTime[];
		year?: number;
	}

	let { data, year = new Date().getFullYear() }: Props = $props();

	// Génère les données pour la heatmap
	// On veut afficher une grille de 52 semaines x 7 jours

	const today = new Date();
	const startDate = new Date(year, 0, 1); // 1er janvier
	const endDate = new Date(year, 11, 31); // 31 décembre

	// Map des données pour accès rapide par date
	const dataMap = $derived(new Map(data.map((d) => [d.date, d.minutes])));

	// Fonction pour obtenir la couleur en fonction des minutes
	function getColor(minutes: number): string {
		if (minutes === 0) return 'bg-muted';
		if (minutes < 30) return 'bg-green-200 dark:bg-green-900';
		if (minutes < 60) return 'bg-green-400 dark:bg-green-700';
		if (minutes < 120) return 'bg-green-600 dark:bg-green-500';
		return 'bg-green-800 dark:bg-green-300';
	}

	// Génère les semaines
	function getWeeks() {
		const weeks = [];
		let currentDate = new Date(startDate);

		// Ajuste pour commencer le premier jour de la semaine (Dimanche ou Lundi selon locale, ici on assume Dimanche pour simplifier ou Lundi selon ISO)
		// On va faire simple: on itère jour par jour et on groupe par semaine

		let currentWeek = [];

		// Remplir les jours vides avant le 1er janvier si ce n'est pas un dimanche
		const dayOfWeek = currentDate.getDay(); // 0 = Dimanche
		for (let i = 0; i < dayOfWeek; i++) {
			currentWeek.push(null);
		}

		while (currentDate <= endDate) {
			const dateStr = currentDate.toISOString().split('T')[0];
			const minutes = dataMap.get(dateStr) || 0;

			currentWeek.push({
				date: dateStr,
				minutes,
				color: getColor(minutes)
			});

			if (currentWeek.length === 7) {
				weeks.push(currentWeek);
				currentWeek = [];
			}

			currentDate.setDate(currentDate.getDate() + 1);
		}

		// Remplir la dernière semaine
		if (currentWeek.length > 0) {
			while (currentWeek.length < 7) {
				currentWeek.push(null);
			}
			weeks.push(currentWeek);
		}

		return weeks;
	}

	const weeks = $derived(getWeeks());
	import { locale } from 'svelte-i18n';

	// ... (existing code)

	// Génère les mois localisés
	const months = $derived(
		Array.from({ length: 12 }, (_, i) => {
			const date = new Date(year, i, 1);
			return date.toLocaleString($locale || 'en', { month: 'short' });
		})
	);

	// Génère les jours de la semaine localisés (Lun, Mer, Ven)
	const weekDays = $derived(
		[1, 3, 5].map((dayIndex) => {
			// 2024-01-01 was a Monday. We use a known date to get the weekday name.
			// Jan 1 2024 (Monday) -> dayIndex 1
			// Jan 3 2024 (Wednesday) -> dayIndex 3
			// Jan 5 2024 (Friday) -> dayIndex 5
			const date = new Date(2024, 0, dayIndex);
			return date.toLocaleString($locale || 'en', { weekday: 'short' });
		})
	);
</script>

<div class="bg-card text-card-foreground overflow-x-auto rounded-lg border p-4 shadow-sm">
	<h3 class="mb-4 text-lg font-semibold">{$_('stats.yearlyActivity')} ({year})</h3>

	<div class="min-w-[700px]">
		<!-- Mois -->
		<div class="text-muted-foreground mb-2 ml-8 flex text-xs">
			{#each months as month}
				<div class="flex-1">{month}</div>
			{/each}
		</div>

		<div class="flex gap-1">
			<!-- Jours de la semaine -->
			<div class="text-muted-foreground flex h-[100px] flex-col justify-between py-1 pr-2 text-xs">
				{#each weekDays as day}
					<span>{day}</span>
				{/each}
			</div>

			<!-- Grille -->
			<div class="flex gap-1">
				{#each weeks as week}
					<div class="flex flex-col gap-1">
						{#each week as day}
							{#if day}
								<div
									class="h-3 w-3 rounded-sm {day.color} transition-colors"
									title="{day.date}: {day.minutes} min"
								></div>
							{:else}
								<div class="h-3 w-3"></div>
							{/if}
						{/each}
					</div>
				{/each}
			</div>
		</div>

		<!-- Légende -->
		<div class="text-muted-foreground mt-4 flex items-center justify-end gap-2 text-xs">
			<span>{$_('stats.less')}</span>
			<div class="flex gap-1">
				<div class="bg-muted h-3 w-3 rounded-sm"></div>
				<div class="h-3 w-3 rounded-sm bg-green-200 dark:bg-green-900"></div>
				<div class="h-3 w-3 rounded-sm bg-green-400 dark:bg-green-700"></div>
				<div class="h-3 w-3 rounded-sm bg-green-600 dark:bg-green-500"></div>
				<div class="h-3 w-3 rounded-sm bg-green-800 dark:bg-green-300"></div>
			</div>
			<span>{$_('stats.more')}</span>
		</div>
	</div>
</div>
