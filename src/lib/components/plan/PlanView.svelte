<script lang="ts">
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { ArrowDown, ArrowUp, Plus, Minus, X } from '@lucide/svelte';
	import { planStore } from '$lib/stores/plan.svelte';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { projectStore } from '$lib/stores/projects.svelte';
	import { updateTask } from '$lib/services/task-service';
	import type { Task } from '$lib/types';
	import Tile from '$lib/components/majolica/Tile.svelte';
	let { onStart }: { onStart: (task: Task) => void } = $props();
	let project = $state('');
	let dragged = $state<number | null>(null);
	const stock = $derived(
		taskStore.tasks.filter(
			(t) => !t.isCompleted && !t.plannedFor && (!project || t.projectId === Number(project))
		)
	);
	const plan = $derived(planStore.plan);
	onMount(() => {
		void planStore.load();
	});
	async function estimate(task: Task, delta: number) {
		try {
			await updateTask(
				task.id,
				task.title,
				task.description,
				task.projectId,
				Math.max(0, Math.min(20, task.estimatedPomodoros + delta))
			);
			await planStore.load();
		} catch {
			planStore.error = true;
		}
	}
	async function move(id: number, target: number) {
		if (!plan) return;
		const ids = plan.tasks.map((t) => t.id);
		const from = ids.indexOf(id);
		if (from < 0 || target < 0 || target >= ids.length || from === target) return;
		ids.splice(target, 0, ids.splice(from, 1)[0]);
		try {
			await planStore.reorder(ids);
		} catch {
			/* état affiché par le store */
		}
	}
	async function change(command: 'plan_task' | 'unplan_task', id: number) {
		try {
			await planStore.change(command, id);
		} catch {
			/* état affiché par le store */
		}
	}
</script>

<section class="plan-view">
	<h1>{$_('plan.title')}</h1>
	{#if planStore.error}<p role="alert" class="error">{$_('plan.error')}</p>{/if}
	{#if planStore.loading && !plan}<p role="status">{$_('common.loading')}</p>{:else}
		<div class="summary">
			<div>
				<strong>{plan?.planned_pomodoros ?? 0}</strong>
				{$_('plan.planned')} · <strong>{plan?.completed_pomodoros ?? 0}</strong>
				{$_('plan.completed')}
			</div>
			<div class="tiles" aria-hidden="true">
				{#each Array(Math.max(plan?.planned_pomodoros ?? 0, plan?.completed_pomodoros ?? 0)) as _, i}<span
						><Tile
							state={i < (plan?.completed_pomodoros ?? 0) ? 'glazed' : 'empty'}
							position={i}
						/></span
					>{/each}
			</div>
			{#if (plan?.planned_pomodoros ?? 0) > 12}<p class="capacity">{$_('plan.capacity')}</p>{/if}
		</div>
		<div class="columns">
			<section aria-labelledby="stock-title">
				<div class="heading">
					<h2 id="stock-title">{$_('plan.stock')}</h2>
					<select bind:value={project} aria-label={$_('plan.all')}
						><option value="">{$_('plan.all')}</option>{#each projectStore.projects as p}<option
								value={String(p.id)}>{p.name}</option
							>{/each}</select
					>
				</div>
				{#if stock.length === 0}<p class="empty">{$_('plan.emptyStock')}</p>{:else}<ul>
						{#each stock as task (task.id)}<li>
								<span>{task.title}</span><button
									class="icon"
									aria-label={`${$_('plan.add')} : ${task.title}`}
									title={$_('plan.add')}
									onclick={() => change('plan_task', task.id)}><Plus size={18} /></button
								>
							</li>{/each}
					</ul>{/if}
			</section>
			<section aria-labelledby="day-title">
				<div class="heading"><h2 id="day-title">{$_('plan.day')}</h2></div>
				{#if !plan?.tasks.length}<p class="empty">{$_('plan.emptyDay')}</p>{:else}<ul>
						{#each plan.tasks as task, i (task.id)}<li
								draggable="true"
								ondragstart={() => (dragged = task.id)}
								ondragend={() => (dragged = null)}
								ondragover={(e) => e.preventDefault()}
								ondrop={(e) => {
									e.preventDefault();
									if (dragged !== null) void move(dragged, i);
								}}
							>
								<div class="task-name">
									<span class:done={task.isCompleted}>{task.title}</span
									>{#if task.carriedOver}<small>{$_('plan.carried')}</small>{/if}
								</div>
								<div class="controls">
									<button
										class="icon"
										disabled={i === 0}
										aria-label={`${$_('plan.up')} : ${task.title}`}
										onclick={() => move(task.id, i - 1)}><ArrowUp size={16} /></button
									><button
										class="icon"
										disabled={i === plan.tasks.length - 1}
										aria-label={`${$_('plan.down')} : ${task.title}`}
										onclick={() => move(task.id, i + 1)}><ArrowDown size={16} /></button
									><button
										class="icon"
										disabled={task.estimatedPomodoros <= 0}
										aria-label={`${$_('plan.decrease')} : ${task.title}`}
										onclick={() => estimate(task, -1)}><Minus size={16} /></button
									><span class="estimate">{task.estimatedPomodoros}</span>
									<span class="task-tiles" aria-hidden="true"
										>{#each Array(Math.min(task.estimatedPomodoros, 10)) as _, tile}<span
												><Tile
													state={tile < task.completedPomodoros ? 'glazed' : 'empty'}
													position={tile}
												/></span
											>{/each}</span
									><button
										class="icon"
										disabled={task.estimatedPomodoros >= 20}
										aria-label={`${$_('plan.increase')} : ${task.title}`}
										onclick={() => estimate(task, 1)}><Plus size={16} /></button
									><button
										class="icon"
										aria-label={`${$_('plan.remove')} : ${task.title}`}
										onclick={() => change('unplan_task', task.id)}><X size={16} /></button
									>
								</div>
							</li>{/each}
					</ul>{/if}
				<button
					class="start"
					disabled={!plan?.tasks.some((t) => !t.isCompleted)}
					onclick={() => {
						const first = plan?.tasks.find((t) => !t.isCompleted);
						if (first) onStart(first);
					}}>{$_('plan.start')}</button
				>
			</section>
		</div>{/if}
</section>

<style>
	.plan-view {
		padding: 32px;
		max-width: 1200px;
		margin: auto;
	}
	h1 {
		font-size: 2rem;
	}
	h2 {
		font-size: 1.35rem;
	}
	.summary {
		padding: 20px 0;
		border-bottom: 1px solid var(--grout);
	}
	.summary strong {
		font-size: 1.3rem;
	}
	.tiles {
		display: flex;
		flex-wrap: wrap;
		gap: 1px;
		margin-top: 12px;
	}
	.tiles span {
		width: 30px;
		border: 1px solid var(--grout);
	}
	.capacity,
	.error {
		margin-top: 12px;
		color: var(--tomato);
	}
	.columns {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 32px;
		margin-top: 24px;
	}
	.heading {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 12px;
		min-height: 42px;
	}
	select {
		max-width: 55%;
		border: 1px solid var(--grout);
		background: var(--background);
		padding: 6px;
	}
	ul {
		list-style: none;
		padding: 0;
	}
	li {
		border-bottom: 1px solid var(--grout);
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 8px;
		min-height: 50px;
		padding: 6px 0;
	}
	.task-name {
		min-width: 0;
		overflow-wrap: anywhere;
	}
	small {
		display: block;
		color: var(--muted-foreground);
	}
	.done {
		text-decoration: line-through;
	}
	.controls {
		display: flex;
		align-items: center;
		flex-shrink: 0;
	}
	.icon {
		display: grid;
		place-items: center;
		width: 30px;
		height: 32px;
		border-radius: 3px;
	}
	.icon:hover {
		background: var(--muted);
	}
	button:disabled {
		opacity: 0.35;
	}
	.task-tiles {
		display: flex;
		gap: 1px;
	}
	.task-tiles span {
		width: 12px;
		border: 1px solid var(--grout);
	}
	.estimate {
		min-width: 24px;
		text-align: center;
		font-variant-numeric: tabular-nums;
	}
	.empty {
		color: var(--muted-foreground);
		padding: 20px 0;
	}
	.start {
		margin-top: 24px;
		padding: 9px 16px;
		background: var(--primary);
		color: var(--primary-foreground);
		border-radius: 3px;
	}
	@media (max-width: 900px) {
		.columns {
			grid-template-columns: 1fr;
		}
		.plan-view {
			padding: 24px;
		}
	}
</style>
