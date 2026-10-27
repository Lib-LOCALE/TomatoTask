<script lang="ts">
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { projectStore } from '$lib/stores/projects.svelte';
	import { _ } from 'svelte-i18n';
	import { Plus } from '@lucide/svelte';
	import TaskCard from './TaskCard.svelte';
	import type { Task } from '$lib/types';
	import { reorderTasks } from '$lib/services/task-service';
	function drop(event: DragEvent, target: number) {
		event.preventDefault();
		const id = Number(event.dataTransfer?.getData('text/plain'));
		const ids = taskStore.filteredTasks.map((t) => t.id);
		const from = ids.indexOf(id);
		const to = ids.indexOf(target);
		if (from < 0 || to < 0 || from === to) return;
		ids.splice(to, 0, ids.splice(from, 1)[0]);
		void reorderTasks(ids);
	}
	interface Props {
		onEditTask?: (task: Task) => void;
		onDeleteTask?: (task: Task) => void;
		onSelectTask?: (task: Task) => void;
		onNewTask?: () => void;
	}
	let { onEditTask, onDeleteTask, onSelectTask, onNewTask }: Props = $props();
</script>

<div class="list">
	<div class="heading">
		<h2>{projectStore.selectedProjectId ? $_('tasks.projectTasks') : $_('tasks.title')}</h2>
		<button onclick={onNewTask} aria-label={$_('tasks.newTask')} title={$_('tasks.newTask')}
			><Plus size={18} /></button
		>
	</div>
	<div class="filters">
		{#each ['all', 'active', 'completed'] as filter}<button
				class:active={taskStore.filterCompleted === filter}
				onclick={() => taskStore.setCompletionFilter(filter as 'all' | 'active' | 'completed')}
				>{$_(`tasks.filter${filter[0].toUpperCase()}${filter.slice(1)}`)}</button
			>{/each}
	</div>
	<div class="rows">
		{#if taskStore.isLoading}<p>{$_('common.loading')}</p>{:else if taskStore.error}<p role="alert">
				{$_('tasks.taskError')}
			</p>{:else if !taskStore.filteredTasks.length}<p>
				{$_('tasks.noTasks')}
			</p>{:else}{#each taskStore.filteredTasks as task (task.id)}<div
					draggable="true"
					role="listitem"
					ondragstart={(event) => event.dataTransfer?.setData('text/plain', String(task.id))}
					ondragover={(event) => event.preventDefault()}
					ondrop={(event) => drop(event, task.id)}
				>
					<TaskCard {task} onEdit={onEditTask} onDelete={onDeleteTask} onSelect={onSelectTask} />
				</div>{/each}{/if}
	</div>
</div>

<style>
	.list {
		display: flex;
		flex-direction: column;
		height: 100%;
		min-height: 0;
		padding: 12px 16px;
	}
	.heading {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.heading h2 {
		font-size: 1.15rem;
	}
	.heading button {
		padding: 5px;
	}
	.filters {
		display: flex;
		gap: 4px;
		margin: 10px 0 4px;
	}
	.filters button {
		font-size: 0.75rem;
		padding: 4px 6px;
		border: 1px solid transparent;
	}
	.filters .active {
		border-color: currentColor;
	}
	.rows {
		overflow: auto;
		min-height: 0;
	}
	.rows p {
		padding: 12px 0;
		font-size: 0.85rem;
	}
</style>
