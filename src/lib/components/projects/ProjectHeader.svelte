<script lang="ts">
	import { taskStore } from '$lib/stores/tasks.svelte';
	import type { Project } from '$lib/types';
	import { _ } from 'svelte-i18n';

	interface Props {
		project: Project;
	}

	let { project }: Props = $props();

	// Calculate stats from taskStore
	const projectTasks = $derived(taskStore.tasks.filter((t) => t.projectId === project.id));
	const totalTasks = $derived(projectTasks.length);
	const completedTasks = $derived(projectTasks.filter((t) => t.isCompleted).length);
	const progress = $derived(totalTasks > 0 ? (completedTasks / totalTasks) * 100 : 0);
</script>

<div class="bg-card text-card-foreground mb-6 rounded-lg border p-6 shadow-sm">
	<div class="flex items-start justify-between">
		<div class="flex items-center gap-4">
			<!-- Large Color Indicator -->
			<div
				class="flex h-12 w-12 items-center justify-center rounded-xl border text-2xl shadow-sm"
				style="background-color: {project.color}20; color: {project.color}; border-color: {project.color}40"
			>
				<div class="h-6 w-6 rounded-full" style="background-color: {project.color}"></div>
			</div>

			<div>
				<h1 class="text-2xl font-bold tracking-tight">{project.name}</h1>
				<div class="text-muted-foreground mt-1 flex items-center gap-2 text-sm">
					<span>{completedTasks} / {totalTasks} tasks completed</span>
					<span>•</span>
					<span>{Math.round(progress)}% done</span>
				</div>
			</div>
		</div>
	</div>

	<!-- Progress Bar -->
	<div class="bg-secondary mt-6 h-2 w-full overflow-hidden rounded-full">
		<div
			class="h-full rounded-full transition-all duration-500 ease-out"
			style="width: {progress}%; background-color: {project.color}"
		></div>
	</div>
</div>
