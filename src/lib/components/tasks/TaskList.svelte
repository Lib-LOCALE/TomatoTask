<script lang="ts">
	// Composant liste de tâches avec filtres
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { _ } from 'svelte-i18n';
	import TaskCard from './TaskCard.svelte';
	import type { Task } from '$lib/types';
	import Button from '$lib/components/ui/button/button.svelte';

	// Props
	interface Props {
		onEditTask?: (task: Task) => void;
		onDeleteTask?: (task: Task) => void;
		onSelectTask?: (task: Task) => void;
		onNewTask?: () => void;
	}

	let { onEditTask, onDeleteTask, onSelectTask, onNewTask }: Props = $props();
</script>

<div class="flex h-full flex-col">
	<!-- Header avec filtres -->
	<div class="flex items-center justify-between border-b px-6 py-4">
		<div class="flex items-center gap-2">
			<h2 class="text-xl font-semibold">{$_('tasks.title')}</h2>

			<!-- Compteurs -->
			<span class="rounded-full bg-muted px-2 py-1 text-xs font-medium">
				{taskStore.activeCount()} active
			</span>
		</div>

		<!-- Bouton nouvelle tâche -->
		<Button onclick={onNewTask}>
			{$_('tasks.newTask')}
		</Button>
	</div>

	<!-- Filtres -->
	<div class="flex gap-2 border-b px-6 py-3">
		<Button
			variant={taskStore.filterCompleted === 'all' ? 'default' : 'outline'}
			size="sm"
			onclick={() => taskStore.setCompletionFilter('all')}
		>
			All
		</Button>

		<Button
			variant={taskStore.filterCompleted === 'active' ? 'default' : 'outline'}
			size="sm"
			onclick={() => taskStore.setCompletionFilter('active')}
		>
			Active
		</Button>

		<Button
			variant={taskStore.filterCompleted === 'completed' ? 'default' : 'outline'}
			size="sm"
			onclick={() => taskStore.setCompletionFilter('completed')}
		>
			Completed
		</Button>
	</div>

	<!-- Liste des tâches -->
	<div class="flex-1 overflow-y-auto p-6">
		{#if taskStore.isLoading}
			<!-- État de chargement -->
			<div class="flex items-center justify-center py-12">
				<div class="text-muted-foreground">
					{$_('common.loading')}
				</div>
			</div>
		{:else if taskStore.error}
			<!-- État d'erreur -->
			<div class="flex items-center justify-center py-12">
				<div class="text-destructive">
					{taskStore.error}
				</div>
			</div>
		{:else if taskStore.filteredTasks().length === 0}
			<!-- État vide -->
			<div class="flex flex-col items-center justify-center py-12 text-center">
				<svg
					class="mb-4 h-16 w-16 text-muted-foreground opacity-50"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
					/>
				</svg>

				<p class="text-muted-foreground">
					{$_('tasks.noTasks')}
				</p>

				<Button class="mt-4" onclick={onNewTask}>
					{$_('tasks.newTask')}
				</Button>
			</div>
		{:else}
			<!-- Liste des tâches -->
			<div class="space-y-3">
				{#each taskStore.filteredTasks() as task (task.id)}
					<TaskCard
						{task}
						onEdit={onEditTask}
						onDelete={onDeleteTask}
						onSelect={onSelectTask}
					/>
				{/each}
			</div>
		{/if}
	</div>
</div>
