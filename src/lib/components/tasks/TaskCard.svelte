<script lang="ts">
	// Composant carte de tâche avec checkbox et actions
	import type { Task } from '$lib/types';
	import { toggleTaskCompletion } from '$lib/services/task-service';
	import { _ } from 'svelte-i18n';
	import { calculateProgress } from '$lib/utils/time-formatter';
	import { toastStore } from '$lib/stores/toast.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Confetti from '$lib/components/effects/Confetti.svelte';

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
	let showConfetti = $state(false);
	let justCompleted = $state(false);

	/**
	 * Gère le toggle de la checkbox
	 */
	async function handleToggle() {
		if (isToggling) return;

		const wasCompleted = task.isCompleted;
		isToggling = true;

		try {
			await toggleTaskCompletion(task.id);

			// Si la tâche vient d'être complétée
			if (!wasCompleted && task.isCompleted) {
				justCompleted = true;
				showConfetti = true;
				toastStore.celebration($_('tasks.taskCompleted'));

				// Reset confetti après 3s
				setTimeout(() => {
					showConfetti = false;
				}, 3000);

				// Reset animation après 1s
				setTimeout(() => {
					justCompleted = false;
				}, 1000);
			}
		} catch (error) {
			console.error('Failed to toggle task:', error);
			toastStore.error($_('tasks.taskError'));
		} finally {
			isToggling = false;
		}
	}

	// Calcule la progression des Pomodoros
	const progress = $derived(calculateProgress(task.completedPomodoros, task.estimatedPomodoros));
</script>

{#if showConfetti}
	<Confetti duration={3000} particleCount={50} />
{/if}

<div
	class="group bg-card relative rounded-lg border p-4 transition-all hover:shadow-md hover-lift scale-in"
	class:opacity-60={task.isCompleted}
	class:pop={justCompleted}
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
					class="text-primary-foreground h-3 w-3"
					fill="none"
					stroke="currentColor"
					stroke-width="3"
					viewBox="0 0 24 24"
				>
					<polyline points="20 6 9 17 4 12"></polyline>
				</svg>
			{/if}
		</button>

		<!-- Titre et Description -->
		<button type="button" onclick={() => onSelect?.(task)} class="flex-1 text-left">
			<h3 class="font-medium" class:line-through={task.isCompleted}>
				{task.title}
			</h3>

			{#if task.description}
				<p class="text-muted-foreground mt-1 text-sm">
					{task.description}
				</p>
			{/if}
		</button>

		<!-- Actions (visible au hover) -->
		<div class="flex gap-1 opacity-0 transition-opacity group-hover:opacity-100">
			<Button variant="ghost" size="sm" class="btn-press hover-bounce" onclick={() => onEdit?.(task)}>
				{$_('common.edit')}
			</Button>

			<Button
				variant="ghost"
				size="sm"
				class="btn-press hover-bounce"
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
			<div class="bg-muted h-2 w-full overflow-hidden rounded-full">
				<div
					class="bg-primary progress-liquid h-full transition-all duration-500"
					style:width="{progress}%"
				></div>
			</div>

			<!-- Texte de progression -->
			<div class="text-muted-foreground flex items-center justify-between text-xs">
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
