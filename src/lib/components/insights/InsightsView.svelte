<script lang="ts">
	import { onMount } from 'svelte';
	import { _, locale } from 'svelte-i18n';
	import { getInsights, getFocusStreak, frescoSerials } from '$lib/services/stats-service';
	import type { Insights, FocusStreak } from '$lib/types';
	import Tile from '$lib/components/majolica/Tile.svelte';
	import Chart from './Chart.svelte';

	let period = $state(7);
	let data = $state<Insights | null>(null);
	let streak = $state<FocusStreak | null>(null);
	let loading = $state(true);
	let error = $state(false);
	let request = 0;
	async function load(days: number) {
		const id = ++request;
		loading = true;
		error = false;
		try {
			const [result, series] = await Promise.all([getInsights(days), getFocusStreak()]);
			if (id === request) {
				data = result;
				streak = series;
			}
		} catch (e) {
			if (id === request) {
				error = true;
				console.error('Insights:', e);
			}
		} finally {
			if (id === request) loading = false;
		}
	}
	onMount(() => {
		void load(period);
	});
	const dayName = (day: number) =>
		new Intl.DateTimeFormat($locale || 'en', { weekday: 'long' }).format(
			new Date(2024, 0, 1 + day)
		);
	const dateLabel = (date: string) =>
		new Intl.DateTimeFormat($locale || 'en', { day: 'numeric', month: 'short' }).format(
			new Date(`${date}T12:00:00`)
		);
	const columns = $derived(Math.max(7, ...(data?.daily.map((day) => day.pomodoros) ?? [])));
	const bestDay = $derived(data ? data.by_weekday.indexOf(Math.max(...data.by_weekday)) : 0);
	const serials = $derived(
		data && streak
			? frescoSerials(
					data.daily.map((d) => d.pomodoros),
					streak.total_pomodoros
				)
			: []
	);
</script>

<section class="insights" aria-labelledby="insights-title">
	<header>
		<div>
			<h1 id="insights-title">{$_('insights.title')}</h1>
			<p>{$_('insights.subtitle')}</p>
		</div>
		<div class="period" role="group" aria-label={$_('insights.period')}>
			{#each [7, 30, 90] as days}<button
					type="button"
					aria-pressed={period === days}
					onclick={() => {
						period = days;
						void load(days);
					}}>{$_('insights.days', { values: { count: days } })}</button
				>{/each}
		</div>
	</header>
	{#if loading}<p role="status">{$_('insights.loading')}</p>
	{:else if error}<div role="alert">
			<p>{$_('insights.error')}</p>
			<button class="retry" onclick={() => load(period)}>{$_('insights.retry')}</button>
		</div>
	{:else if data && streak}
		{#if data.total_pomodoros === 0}
			<div class="empty">
				<h2>{$_('insights.emptyTitle')}</h2>
				<p>{$_('insights.empty')}</p>
				<div class="sample"><Tile state="planned" /></div>
			</div>
		{:else}
			<div class="story">
				<p class="lead">
					{$_('insights.completed', { values: { count: data.total_pomodoros, days: period } })}
				</p>
				<p>
					{$_(
						data.total_pomodoros > data.previous_period_pomodoros
							? 'insights.more'
							: data.total_pomodoros < data.previous_period_pomodoros
								? 'insights.less'
								: 'insights.same',
						{ values: { count: Math.abs(data.total_pomodoros - data.previous_period_pomodoros) } }
					)}
				</p>
				<div class="observations">
					<p>
						{$_('insights.bestHour', {
							values: { hour: data.best_hour, end: (data.best_hour ?? 0) + 1 }
						})}
					</p>
					<p>{$_('insights.bestDay', { values: { day: dayName(bestDay) } })}</p>
					<p>
						{$_('insights.completion', {
							values: { rate: Math.round(data.completion_rate * 100) }
						})}
					</p>
					<p>
						{$_('insights.streak', {
							values: { current: streak.current_days, best: streak.best_days }
						})}
					</p>
				</div>
			</div>
			<div class="charts">
				<Chart
					title={$_('insights.daily')}
					unit={$_('insights.minutes')}
					points={data.daily.map((d) => ({
						label: dateLabel(d.date),
						value: d.focus_minutes,
						tiles: d.pomodoros
					}))}
				/>
				<Chart
					title={$_('insights.hours')}
					unit={$_('insights.sessions')}
					points={data.by_hour.map((value, hour) => ({ label: `${hour}h`, value }))}
				/>
				<Chart
					title={$_('insights.weekdays')}
					unit={$_('insights.sessions')}
					points={data.by_weekday.map((value, day) => ({ label: dayName(day).slice(0, 3), value }))}
				/>
			</div>
			<section class="fresco">
				<h2>{$_('insights.fresco')}</h2>
				<p>{$_('insights.frescoDescription')}</p>
				<div class="fresco-wall" style:--columns={columns}>
					{#each data.daily as day, index}
						<span class="day-label">{dateLabel(day.date)} · {day.pomodoros}</span>
						{#each Array(columns) as cell, position}
							{@const serial = serials[index]?.[position]}
							<span
								class="tile-wrap"
								title={serial ? $_('insights.serial', { values: { serial } }) : dateLabel(day.date)}
								tabindex={serial ? 0 : undefined}
								aria-label={serial
									? $_('insights.serial', { values: { serial } })
									: dateLabel(day.date)}
								><Tile
									state={serial ? 'glazed' : 'empty'}
									framed={false}
									position={index * columns + position}
									{serial}
								/></span
							>
						{/each}
					{/each}
				</div>
			</section>
			<div class="rankings">
				<section>
					<h2>{$_('insights.tasks')}</h2>
					<ol>
						{#each data.top_tasks as task}<li>
								<span>{task.title}</span><strong>{task.pomodoros}</strong>
							</li>{:else}<li>{$_('insights.noTasks')}</li>{/each}
					</ol>
				</section>
				<section>
					<h2>{$_('insights.projects')}</h2>
					<ol>
						{#each data.top_projects as project}<li>
								<span>{project.name}</span><strong>{project.pomodoros}</strong>
							</li>{:else}<li>{$_('insights.noProjects')}</li>{/each}
					</ol>
				</section>
			</div>
		{/if}
	{/if}
</section>

<style>
	.insights {
		max-width: 1100px;
		margin: auto;
		padding: 32px clamp(20px, 4vw, 48px) 64px;
	}
	header {
		display: flex;
		justify-content: space-between;
		align-items: end;
		gap: 20px;
		flex-wrap: wrap;
		margin-bottom: 32px;
	}
	h1 {
		font-size: clamp(2rem, 4vw, 3rem);
		line-height: 1.1;
	}
	h2 {
		font-size: 1.5rem;
	}
	header p,
	.fresco > p,
	.empty p {
		color: var(--muted-foreground);
		margin-top: 6px;
	}
	.period {
		display: flex;
		gap: 4px;
	}
	.period button,
	.retry {
		border: 1px solid var(--border);
		border-radius: 4px;
		padding: 7px 12px;
		font-weight: 600;
	}
	.period button[aria-pressed='true'] {
		background: var(--primary);
		color: var(--primary-foreground);
		border-color: var(--primary);
	}
	.story {
		border-top: 1px solid var(--grout);
		border-bottom: 1px solid var(--grout);
		padding: 22px 0;
	}
	.lead {
		font-family: var(--font-heading);
		font-size: clamp(1.4rem, 2.4vw, 2rem);
		line-height: 1.3;
	}
	.story > p + p {
		color: var(--muted-foreground);
		margin-top: 6px;
	}
	.observations {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 12px 28px;
		margin-top: 24px;
	}
	.charts {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 330px), 1fr));
		gap: 32px;
		padding: 32px 0;
	}
	.charts :global(.chart:first-child) {
		grid-column: 1 / -1;
	}
	.fresco {
		border-top: 1px solid var(--grout);
		padding-top: 24px;
	}
	.fresco > p {
		margin-bottom: 20px;
	}
	.fresco-wall {
		display: grid;
		grid-template-columns: 80px repeat(var(--columns), minmax(0, 1fr));
		gap: 3px;
		padding: 3px;
		background: var(--manganese);
	}
	.day-label {
		display: flex;
		align-items: center;
		padding: 4px;
		background: var(--glaze);
		color: var(--foreground);
		font-size: 0.8rem;
	}
	.tile-wrap {
		position: relative;
		display: block;
		min-width: 0;
	}
	.tile-wrap:hover::after,
	.tile-wrap:focus-visible::after {
		content: attr(title);
		position: absolute;
		z-index: 2;
		bottom: 100%;
		left: 0;
		width: max-content;
		max-width: 220px;
		padding: 4px 8px;
		background: var(--popover);
		color: var(--popover-foreground);
		border: 1px solid var(--border);
		font-size: 0.8rem;
	}
	.tile-wrap:focus-visible {
		outline: 2px solid var(--ring);
		outline-offset: 2px;
	}
	.rankings {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: 32px;
		border-top: 1px solid var(--grout);
		margin-top: 32px;
		padding-top: 24px;
	}
	ol {
		list-style: none;
		padding: 0;
		margin-top: 12px;
	}
	li {
		display: flex;
		justify-content: space-between;
		gap: 16px;
		padding: 8px 0;
		border-bottom: 1px solid var(--border);
	}
	.empty {
		padding: 32px 0;
		border-top: 1px solid var(--grout);
	}
	.sample {
		width: 90px;
		margin-top: 24px;
	}
</style>
