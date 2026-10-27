<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { taskStore } from '$lib/stores/tasks.svelte';
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
	const planned = $derived(
		taskStore.tasks.filter((t) => !t.isCompleted).reduce((sum, t) => sum + t.estimatedPomodoros, 0)
	);
	const painting = $derived(timerStore.sessionType === 'work' && timerStore.remainingSeconds > 0);
	const tiles = $derived(frescoTiles(planned, completed, total, painting));
	async function refresh() {
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
		const interval = window.setInterval(refresh, 60_000);
		return () => window.clearInterval(interval);
	});
	let lastCount = 0;
	$effect(() => {
		const count = timerStore.pomodoroCount;
		if (count !== lastCount) {
			lastCount = count;
			void refresh();
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
	{:else if tiles.length === 0}<p class="empty">{$_('majolica.empty')}</p>
	{:else}<div
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
	<details
		ontoggle={(event) => {
			if (event.currentTarget.open) void loadWeek();
		}}
	>
		<summary>{$_('majolica.week')}</summary>
		{#each week as day}<div class="week-row">
				<span>{day.label}</span>
				<div>
					{#each Array(day.count) as _, index}<span class="week-tile"
							><Tile state="glazed" position={index} /></span
						>{/each}
				</div>
				<strong>{day.count}</strong>
			</div>{/each}
	</details>
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
		background: var(--grout);
		box-shadow: inset 0 2px 4px rgba(24, 20, 18, 0.25);
	}
	.empty {
		padding: 24px 0;
	}
	details {
		margin-top: 24px;
		padding-top: 16px;
		border-top: 1px solid var(--grout);
	}
	summary {
		cursor: pointer;
		font-weight: 700;
	}
	.week-row {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-top: 10px;
		font-size: 0.85rem;
	}
	.week-row > span:first-child {
		width: 55px;
	}
	.week-row > div {
		display: flex;
		flex: 1;
		flex-wrap: wrap;
		gap: 1px;
	}
	.week-tile {
		display: block;
		width: 22px;
		border: 1px solid var(--grout);
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
