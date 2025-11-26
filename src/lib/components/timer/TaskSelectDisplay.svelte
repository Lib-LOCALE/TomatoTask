<script lang="ts">
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { selectTask } from '$lib/services/task-service';
	import { _ } from 'svelte-i18n';
	import type { Task } from '$lib/types';
	import Button from '$lib/components/ui/button/button.svelte';
	import { CheckCircle2, X } from '@lucide/svelte';

	interface Props {
		selectedTaskId?: number;
		onTaskSelected?: (task: Task | null) => void;
	}

	let { selectedTaskId, onTaskSelected }: Props = $props();

	// Tâches actives uniquement
	const activeTasks = $derived(taskStore.tasks.filter((t) => !t.isCompleted));

	// Tâche sélectionnée
	const selectedTask = $derived(
		selectedTaskId ? taskStore.tasks.find((t) => t.id === selectedTaskId) : null
	);

	// État pour afficher le sélecteur
	let isSelecting = $state(false);

	function handleSelect(task: Task) {
		selectTask(task);
		onTaskSelected?.(task);
		isSelecting = false;
	}

	function handleClear() {
		selectTask(null);
		onTaskSelected?.(null);
	}
</script>

<div class="w-full">
	{#if !isSelecting && selectedTask}
		<!-- Affichage de la tâche sélectionnée -->
		<div class="bg-card relative overflow-hidden rounded-lg border p-4 shadow-sm transition-all">
			<div class="flex items-start justify-between gap-4">
				<div class="flex-1">
					<div class="text-muted-foreground mb-1 flex items-center gap-2 text-xs font-medium">
						<span class="tracking-wider uppercase">{$_('tasks.currentTask')}</span>
					</div>
					<h3 class="text-foreground text-lg leading-tight font-semibold">
						{selectedTask.title}
					</h3>
					{#if selectedTask.description}
						<p class="text-muted-foreground mt-1 line-clamp-2 text-sm">
							{selectedTask.description}
						</p>
					{/if}

					<!-- Progress -->
					{#if selectedTask.estimatedPomodoros > 0}
						<div class="text-muted-foreground mt-3 flex items-center gap-2 text-xs">
							<div class="bg-muted h-1.5 w-24 overflow-hidden rounded-full">
								<div
									class="bg-primary h-full transition-all"
									style:width="{(selectedTask.completedPomodoros /
										selectedTask.estimatedPomodoros) *
										100}%"
								></div>
							</div>
							<span
								>{selectedTask.completedPomodoros} / {selectedTask.estimatedPomodoros} pomodoros</span
							>
						</div>
					{/if}
				</div>

				<div class="flex flex-col gap-2">
					<Button variant="outline" size="sm" onclick={() => (isSelecting = true)}>
						{$_('common.change')}
					</Button>
					<Button
						variant="ghost"
						size="icon"
						class="h-8 w-8"
						onclick={handleClear}
						title="Clear selection"
					>
						<X class="h-4 w-4" />
					</Button>
				</div>
			</div>

			<!-- Background decoration -->
			<div class="bg-primary/5 absolute -top-4 -right-4 h-24 w-24 rounded-full blur-2xl"></div>
		</div>
	{:else}
		<!-- Mode sélection -->
		<div class="space-y-3">
			{#if !isSelecting}
				<button
					onclick={() => (isSelecting = true)}
					class="border-muted-foreground/25 bg-muted/50 text-muted-foreground hover:border-primary/50 hover:bg-muted hover:text-foreground flex w-full items-center justify-center gap-2 rounded-lg border border-dashed p-8 transition-all"
				>
					<CheckCircle2 class="h-5 w-5" />
					<span class="font-medium">{$_('tasks.selectTask')}</span>
				</button>
			{:else}
				<div class="bg-card rounded-lg border shadow-sm">
					<div class="flex items-center justify-between border-b px-4 py-3">
						<h3 class="font-medium">{$_('tasks.selectTask')}</h3>
						<Button variant="ghost" size="sm" onclick={() => (isSelecting = false)}>
							{$_('common.cancel')}
						</Button>
					</div>

					<div class="max-h-60 overflow-y-auto p-2">
						{#if activeTasks.length === 0}
							<div class="text-muted-foreground p-4 text-center text-sm">
								{$_('tasks.noActiveTasks')}
							</div>
						{:else}
							<div class="space-y-1">
								{#each activeTasks as task (task.id)}
									<button
										onclick={() => handleSelect(task)}
										class="hover:bg-accent hover:text-accent-foreground flex w-full items-center justify-between rounded-md px-3 py-2 text-left text-sm transition-colors"
										class:bg-accent={task.id === selectedTaskId}
									>
										<span class="font-medium">{task.title}</span>
										{#if task.estimatedPomodoros > 0}
											<span class="text-muted-foreground text-xs">
												{task.completedPomodoros}/{task.estimatedPomodoros} 🍅
											</span>
										{/if}
									</button>
								{/each}
							</div>
						{/if}
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>
