<script lang="ts">
	// Sélecteur de tâche pour associer au timer
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { selectTask } from '$lib/services/task-service';
	import { _ } from 'svelte-i18n';
	import type { Task } from '$lib/types';

	// Props
	interface Props {
		selectedTaskId?: number;
		onTaskSelected?: (task: Task | null) => void;
	}

	let { selectedTaskId, onTaskSelected }: Props = $props();

	/**
	 * Gère la sélection d'une tâche
	 */
	function handleSelect(event: Event) {
		const select = event.target as HTMLSelectElement;
		const taskId = select.value ? parseInt(select.value) : null;

		if (taskId === null) {
			selectTask(null);
			onTaskSelected?.(null);
		} else {
			const task = taskStore.tasks.find((t) => t.id === taskId);
			if (task) {
				selectTask(task);
				onTaskSelected?.(task);
			}
		}
	}

	// Tâches actives uniquement
	const activeTasks = $derived(taskStore.tasks.filter((t) => !t.isCompleted));

	// Derived value for the select element to ensure it's always a string
	const selectValue = $derived(selectedTaskId?.toString() ?? '');
</script>

<div class="w-full">
	<label for="task-select" class="mb-2 block text-sm font-medium">
		{$_('tasks.title')}
	</label>

	<select
		id="task-select"
		onchange={handleSelect}
		value={selectValue}
		class="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus:ring-ring flex h-10 w-full rounded-md border px-3 py-2 text-sm focus:ring-2 focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
	>
		<option value="">No task (free session)</option>

		{#each activeTasks as task (task.id)}
			<option value={task.id}>
				{task.title}
				{#if task.estimatedPomodoros > 0}
					({task.completedPomodoros}/{task.estimatedPomodoros} 🍅)
				{/if}
			</option>
		{/each}
	</select>

	{#if activeTasks.length === 0}
		<p class="text-muted-foreground mt-2 text-xs">
			No active tasks. Create one to track your work!
		</p>
	{/if}
</div>
