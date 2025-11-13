<script lang="ts">
	// Composant carte de tâche avec checkbox et actions
	import type { Task } from '$lib/types';
	import { toggleTaskCompletion } from '$lib/services/task-service';
	import { _ } from 'svelte-i18n';
	import { calculateProgress } from '$lib/utils/time-formatter';
	import Button from '$lib/components/ui/button/button.svelte';

	// Props
	interface Props {
		task: Task;
		onEdit?: (task: Task) => void;
		onDelete?: (task: Task) => void;
		onSelect?: (task: Task) => void;
	}

	let { task, onEdit, onDelete, onSelect }: Props = $props();

	// État local
	let isToggling = $state(false);

	/**
	 * Gère le toggle de la checkbox
	 */
	async function handleToggle() {
		if (isToggling) return;

		isToggling = true;
		try {
			await toggleTaskCompletion(task.id);
		} catch (error) {
			console.error('Failed to toggle task:', error);
		} finally {
			isToggling = false;
		}
	}

	// Calcule la progression des Pomodoros
	const progress = $derived(() => {
		return calculateProgress(task.completedPomodoros, task.estimatedPomodoros);
	});
</script>

<div
	class="group relative rounded-lg border bg-card p-4 transition-all hover:shadow-md"
	class:opacity-60={task.isCompleted}
>
	<!-- Header: Checkbox + Titre -->
	<div class="flex items-start gap-3">
		<!-- Checkbox -->
		<button
			type="button"
			onclick={handleToggle}
			disabled={isToggling}
			class="mt-1 flex h-5 w-5 shrink-0 items-center justify-center rounded border-2 transition-colors"
			class:border-primary={task.isCompleted}
			class:bg-primary={task.isCompleted}
			class:border-muted-foreground={!task.isCompleted}
		>
			{#if task.isCompleted}
				<svg
					class="h-3 w-3 text-primary-foreground"
					fill="none"
					stroke="currentColor"
					stroke-width="3"
					viewBox="0 0 24 24"
				>
					<polyline points="20 6 9 17 4 12" />
				</svg>
			{/if}
		</button>

		<!-- Titre et Description -->
		<button
			type="button"
			onclick={() => onSelect?.(task)}
			class="flex-1 text-left"
		>
			<h3
				class="font-medium"
				class:line-through={task.isCompleted}
			>
				{task.title}
			</h3>

			{#if task.description}
				<p class="mt-1 text-sm text-muted-foreground">
					{task.description}
				</p>
			{/if}
		</button>

		<!-- Actions (visible au hover) -->
		<div class="flex gap-1 opacity-0 transition-opacity group-hover:opacity-100">
			<Button
				variant="ghost"
				size="sm"
				onclick={() => onEdit?.(task)}
			>
				{$_('common.edit')}
			</Button>

			<Button
				variant="ghost"
				size="sm"
				onclick={() => onDelete?.(task)}
			>
				{$_('common.delete')}
			</Button>
		</div>
	</div>

	<!-- Footer: Pomodoros Progress -->
	{#if task.estimatedPomodoros > 0}
		<div class="mt-3 space-y-2">
			<!-- Barre de progression -->
			<div class="h-2 w-full overflow-hidden rounded-full bg-muted">
				<div
					class="h-full bg-primary transition-all"
					style:width="{progress()}%"
				></div>
			</div>

			<!-- Texte de progression -->
			<div class="flex items-center justify-between text-xs text-muted-foreground">
				<span>
					{$_('tasks.completedPomodoros', {
						values: {
							completed: task.completedPomodoros,
							estimated: task.estimatedPomodoros
						}
					})}
				</span>

				{#if task.completedPomodoros >= task.estimatedPomodoros}
					<span class="font-medium text-green-600 dark:text-green-400">
						✓ {$_('tasks.completed')}
					</span>
				{/if}
			</div>
		</div>
	{/if}
</div>
