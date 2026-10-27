<script lang="ts">
	import type { Task } from '$lib/types';
	import { toggleTaskCompletion } from '$lib/services/task-service';
	import { planStore, localDate } from '$lib/stores/plan.svelte';
	import { _ } from 'svelte-i18n';
	import { Pencil, CalendarPlus, Trash2 } from '@lucide/svelte';
	import Tile from '$lib/components/majolica/Tile.svelte';
	interface Props {
		task: Task;
		onEdit?: (task: Task) => void;
		onDelete?: (task: Task) => void;
		onSelect?: (task: Task) => void;
	}
	let { task, onEdit, onDelete, onSelect }: Props = $props();
	let error = $state(false);
	async function toggle() {
		try {
			await toggleTaskCompletion(task.id);
			error = false;
		} catch {
			error = true;
		}
	}
	async function plan() {
		try {
			await planStore.change('plan_task', task.id);
			error = false;
		} catch {
			error = true;
		}
	}
</script>

<div class="task-row group">
	<button
		class="check"
		aria-label={$_(task.isCompleted ? 'majolica.markActive' : 'majolica.markComplete', {
			values: { title: task.title }
		})}
		onclick={toggle}>{task.isCompleted ? '✓' : ''}</button
	>
	<div class="body">
		<button class="title" class:done={task.isCompleted} onclick={() => onSelect?.(task)}
			>{task.title}</button
		>
		<div class="meta">
			<span class="mini-tiles" aria-hidden="true"
				>{#each Array(Math.min(10, Math.max(task.estimatedPomodoros, task.completedPomodoros))) as _, i}<span
						><Tile state={i < task.completedPomodoros ? 'glazed' : 'empty'} position={i} /></span
					>{/each}</span
			><span>{task.completedPomodoros}/{task.estimatedPomodoros}</span
			>{#if task.plannedFor === localDate()}<span class="badge">{$_('plan.today')}</span>{/if}
		</div>
		{#if error}<span role="alert">{$_('tasks.taskError')}</span>{/if}
	</div>
	<div class="actions">
		<button aria-label={`${$_('common.edit')} : ${task.title}`} onclick={() => onEdit?.(task)}
			><Pencil size={15} /></button
		>{#if !task.isCompleted && task.plannedFor !== localDate()}<button
				aria-label={`${$_('plan.add')} : ${task.title}`}
				onclick={plan}><CalendarPlus size={15} /></button
			>{/if}<button
			aria-label={`${$_('common.delete')} : ${task.title}`}
			onclick={() => onDelete?.(task)}><Trash2 size={15} /></button
		>
	</div>
</div>

<style>
	.task-row {
		display: flex;
		gap: 8px;
		align-items: flex-start;
		padding: 9px 4px;
		border-bottom: 1px solid #ffffff40;
	}
	.check {
		flex: none;
		width: 18px;
		height: 18px;
		border: 1px solid currentColor;
		border-radius: 2px;
		margin-top: 3px;
		line-height: 1;
	}
	.body {
		flex: 1;
		min-width: 0;
	}
	.title {
		display: block;
		text-align: left;
		overflow-wrap: anywhere;
		font-weight: 600;
	}
	.done {
		text-decoration: line-through;
	}
	.meta {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 0.75rem;
		margin-top: 4px;
	}
	.mini-tiles {
		display: flex;
		gap: 1px;
	}
	.mini-tiles span {
		width: 10px;
		border: 1px solid #ffffff80;
	}
	.badge {
		border: 1px solid #ffffff80;
		padding: 0 3px;
	}
	.actions {
		display: flex;
		opacity: 0;
	}
	.group:hover .actions,
	.group:focus-within .actions {
		opacity: 1;
	}
	.actions button {
		padding: 4px;
	}
	@media (hover: none) {
		.actions {
			opacity: 1;
		}
	}
</style>
