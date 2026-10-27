<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { planStore } from '$lib/stores/plan.svelte';
	import { timerStore } from '$lib/stores/timer.svelte';
	import type { PomodoroSession } from '$lib/types';
	import Tile from './Tile.svelte';
	import { frescoTiles } from './patterns';
	interface Streak {
		current_days: number;
		best_days: number;
		total_pomodoros: number;
		today_pomodoros: number;
	}
	let { onStreak = (_days: number) => {} }: { onStreak?: (days: number) => void } = $props();
	let completed = $state(0);
	let total = $state(0);
	let previousCompleted = $state<number | null>(null);
	let animatedIndex = $state(-1);
	let error = $state(false);
	let week = $state<{ label: string; count: number }[]>([]);
	const planned = $derived(planStore.plan?.planned_pomodoros ?? 0);
	const painting = $derived(timerStore.sessionType === 'work' && timerStore.remainingSeconds > 0);
	const tiles = $derived(frescoTiles(planned, completed, total, painting));
	async function refresh() {
		await planStore.load();
		const date = new Date();
		const day = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`;
		const yesterday = new Date(date);
		yesterday.setDate(yesterday.getDate() - 1);
		const tomorrow = new Date(date);
		tomorrow.setDate(tomorrow.getDate() + 1);
		const utcDay = (d: Date) => d.toISOString().slice(0, 10);
		try {
			const [sessions, streak] = await Promise.all([
				invoke<PomodoroSession[]>('get_sessions_by_date_range', {
					startDate: utcDay(yesterday),
					endDate: utcDay(tomorrow)
				}),
				invoke<Streak>('get_focus_streak')
			]);
			const count = sessions.filter(
				(s) =>
					s.sessionType === 'work' &&
					s.completedAt &&
					!s.interrupted &&
					new Date(
						s.completedAt.includes('T') ? s.completedAt : s.completedAt.replace(' ', 'T') + 'Z'
					).toLocaleDateString('en-CA') === day
			).length;
			if (previousCompleted !== null && count > previousCompleted) animatedIndex = count - 1;
			previousCompleted = count;
			completed = count;
			total = streak.total_pomodoros;
			onStreak(streak.current_days);
			error = false;
		} catch (e) {
			console.error('Fresco:', e);
			error = true;
		}
	}
	async function loadWeek() {
		const now = new Date();
		const dates = Array.from({ length: 7 }, (_, index) => {
			const date = new Date(now);
			date.setDate(date.getDate() - 6 + index);
			return date;
		});
		const dateKey = (date: Date) =>
			`${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`;
		const start = new Date(dates[0]);
		start.setDate(start.getDate() - 1);
		const end = new Date(now);
		end.setDate(end.getDate() + 1);
		try {
			const sessions = await invoke<PomodoroSession[]>('get_sessions_by_date_range', {
				startDate: start.toISOString().slice(0, 10),
				endDate: end.toISOString().slice(0, 10)
			});
			week = dates.map((date) => ({
				label: new Intl.DateTimeFormat(undefined, { weekday: 'short', day: 'numeric' }).format(
					date
				),
				count: sessions.filter(
					(s) =>
						s.sessionType === 'work' &&
						s.completedAt &&
						!s.interrupted &&
						dateKey(
							new Date(
								s.completedAt.includes('T') ? s.completedAt : s.completedAt.replace(' ', 'T') + 'Z'
							)
						) === dateKey(date)
				).length
			}));
		} catch (e) {
			console.error('Fresco week:', e);
			error = true;
		}
	}

	onMount(() => {
		void refresh();
		void loadWeek();
		const interval = window.setInterval(() => {
			void refresh();
			void loadWeek();
		}, 60_000);
		return () => window.clearInterval(interval);
	});
	let lastCount = 0;
	$effect(() => {
		const count = timerStore.pomodoroCount;
		if (count !== lastCount) {
			lastCount = count;
			void refresh();
			void loadWeek();
		}
	});
</script>

<section class="fresco" aria-labelledby="fresco-title">
	<div class="fresco-heading">
		<div>
			<h2 id="fresco-title">{$_('majolica.fresco')}</h2>
			<p>{$_('majolica.frescoDescription')}</p>
		</div>
		<strong
			>{$_('majolica.count', {
				values: { done: completed, total: Math.max(planned, completed) }
			})}</strong
		>
	</div>
	{#if error}<p role="status">{$_('majolica.loadError')}</p>
	{:else}
		{#if planned === 0 && completed === 0 && !painting}<p class="empty">
				{$_('majolica.empty')}
			</p>{/if}
		<div
			class="grid"
			aria-label={$_('majolica.count', {
				values: { done: completed, total: Math.max(planned, completed) }
			})}
		>
			{#each tiles as tile, index}<Tile
					{...tile}
					position={index}
					progress={timerStore.progress() / 100}
					animate={index === animatedIndex}
				/>{/each}
		</div>{/if}
	<section class="week" aria-labelledby="week-title">
		<div class="week-heading">
			<h3 id="week-title">{$_('majolica.week')}</h3>
			<strong>{week.reduce((sum, day) => sum + day.count, 0)}</strong>
		</div>
		{#if week.length}
			<div class="week-labels">
				{#each week as day}<span title={day.label}>{day.label}</span>{/each}
			</div>
			<div
				class="week-wall"
				role="img"
				aria-label={week.map((day) => `${day.label}: ${day.count}`).join(', ')}
			>
				{#each Array(Math.max(3, ...week.map((day) => day.count))) as _, row}
					{#each week as day, column}
						<Tile
							state={row < day.count ? 'glazed' : 'empty'}
							position={row * 7 + column}
							framed={false}
						/>
					{/each}
				{/each}
			</div>
		{/if}
	</section>
</section>

<style>
	.fresco {
		padding: 32px 24px;
		border-left: 1px solid var(--grout);
		height: 100%;
	}
	.fresco-heading {
		display: flex;
		align-items: start;
		justify-content: space-between;
		gap: 12px;
		margin-bottom: 24px;
	}
	h2 {
		font-size: 1.6rem;
		line-height: 1.1;
	}
	p {
		color: var(--muted-foreground);
		font-size: 0.875rem;
		margin-top: 8px;
	}
	strong {
		white-space: nowrap;
		font-size: 0.85rem;
		color: var(--copper);
	}
	.grid {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 3px;
		padding: 3px;
		background: var(--manganese);
	}
	.empty {
		padding: 24px 0;
	}
	.week {
		margin-top: 28px;
	}
	.week-heading {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		margin-bottom: 12px;
	}
	.week-heading h3 {
		font-family: var(--font-heading);
		font-size: 1.3rem;
	}
	.week-labels {
		display: grid;
		grid-template-columns: repeat(7, minmax(0, 1fr));
		gap: 3px;
		margin-bottom: 6px;
	}
	.week-labels span {
		font-size: 0.7rem;
		text-align: center;
		overflow: hidden;
		white-space: nowrap;
	}
	.week-wall {
		display: grid;
		grid-template-columns: repeat(7, minmax(0, 1fr));
		gap: 3px;
		padding: 3px;
		background: var(--manganese);
	}

	@media (max-width: 1099px) {
		.fresco {
			border-left: 0;
			border-top: 1px solid var(--grout);
			height: auto;
		}
		.grid {
			grid-template-columns: repeat(6, minmax(0, 1fr));
		}
	}
</style>
