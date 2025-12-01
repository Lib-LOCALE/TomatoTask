<script lang="ts">
	// Liste des projets avec sélection pour filtrage
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { ChevronDown, ChevronRight, Plus, Inbox, Edit2, Trash2 } from '@lucide/svelte';
	import { projectStore } from '$lib/stores/projects.svelte';
	import { updateTask } from '$lib/services/task-service';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import type { Project } from '$lib/types';
	import { slide } from 'svelte/transition';
	import { flip } from 'svelte/animate';

	// Props
	interface Props {
		onProjectSelect?: (projectId: number | null) => void;
		onNewProject?: () => void;
		onEditProject?: (project: Project) => void;
		onDeleteProject?: (project: Project) => void;
	}

	let { onProjectSelect, onNewProject, onEditProject, onDeleteProject }: Props = $props();

	// État local
	let isExpanded = $state(true);
	let hoveredProjectId = $state<number | null>(null);
	let dragOverProjectId = $state<number | null | 'inbox'>(null);

	// Projet sélectionné
	const selectedProjectId = $derived(projectStore.selectedProjectId);

	/**
	 * Gère la sélection d'un projet
	 */
	function handleSelectProject(projectId: number | null) {
		projectStore.selectProject(projectId);
		onProjectSelect?.(projectId);
	}

	/**
	 * Gère la création d'un nouveau projet
	 */
	function handleNewProject(e: MouseEvent) {
		e.stopPropagation();
		onNewProject?.();
	}

	/**
	 * Gère le survol d'un élément draggable
	 */
	function handleDragOver(e: DragEvent, projectId: number | null) {
		e.preventDefault();
		if (projectId === null) {
			dragOverProjectId = 'inbox';
		} else {
			dragOverProjectId = projectId;
		}
	}

	/**
	 * Gère la sortie d'un élément draggable
	 */
	function handleDragLeave(e: DragEvent) {
		dragOverProjectId = null;
	}

	/**
	 * Gère le drop d'une tâche
	 */
	async function handleDrop(e: DragEvent, projectId: number | null) {
		e.preventDefault();
		dragOverProjectId = null;

		const taskIdStr = e.dataTransfer?.getData('text/plain');
		if (!taskIdStr) return;

		const taskId = parseInt(taskIdStr);
		if (isNaN(taskId)) return;

		// Trouve la tâche
		const task = taskStore.tasks.find((t) => t.id === taskId);
		if (!task) return;

		// Si le projet est différent, met à jour
		if (task.projectId !== (projectId === null ? undefined : projectId)) {
			try {
				// Note: updateTask signature: id, title, description, projectId, estimatedPomodoros
				// We need to pass all fields.
				await updateTask(
					task.id,
					task.title,
					task.description,
					projectId === null ? undefined : projectId,
					task.estimatedPomodoros
				);

				// Refresh projects to update counts
				await projectStore.load();
			} catch (error) {
				console.error('Failed to move task to project:', error);
			}
		}
	}

	onMount(async () => {
		// Charge les projets au montage
		await projectStore.load();
	});
</script>

<div class="flex h-full flex-col">
	<!-- Section Header (Collapsible) -->
	<div
		role="button"
		tabindex="0"
		class="hover:bg-muted/50 group flex w-full cursor-pointer items-center justify-between p-4 transition-colors"
		onclick={() => (isExpanded = !isExpanded)}
		onkeydown={(e) => e.key === 'Enter' && (isExpanded = !isExpanded)}
	>
		<div
			class="text-muted-foreground group-hover:text-foreground flex items-center gap-2 text-sm font-semibold transition-colors"
		>
			{#if isExpanded}
				<ChevronDown class="h-4 w-4" />
			{:else}
				<ChevronRight class="h-4 w-4" />
			{/if}
			{$_('projects.title')}
		</div>

		<button
			type="button"
			onclick={(e) => {
				e.stopPropagation();
				handleNewProject(e);
			}}
			class="hover:bg-background rounded-md p-1 opacity-0 transition-all group-hover:opacity-100"
			title={$_('projects.newProject')}
		>
			<Plus class="h-4 w-4" />
		</button>
	</div>

	{#if isExpanded}
		<div class="flex-1 space-y-1 overflow-y-auto px-2 pb-2">
			<!-- All Tasks / Inbox -->
			<div
				role="button"
				tabindex="0"
				onclick={() => handleSelectProject(null)}
				onkeydown={(e) => e.key === 'Enter' && handleSelectProject(null)}
				ondragover={(e) => handleDragOver(e, null)}
				ondragleave={handleDragLeave}
				ondrop={(e) => handleDrop(e, null)}
				class="group flex w-full cursor-pointer items-center gap-3 rounded-md border border-transparent px-3 py-2 text-left text-sm transition-all"
				class:bg-primary={selectedProjectId === null && dragOverProjectId !== 'inbox'}
				class:text-primary-foreground={selectedProjectId === null && dragOverProjectId !== 'inbox'}
				class:hover:bg-muted={selectedProjectId !== null && dragOverProjectId !== 'inbox'}
				class:bg-accent={dragOverProjectId === 'inbox'}
				class:border-primary={dragOverProjectId === 'inbox'}
			>
				<Inbox class="h-4 w-4" />
				<span class="font-medium">{$_('projects.allProjects')}</span>
				{#if projectStore.projects.length > 0}
					<span class="ml-auto text-xs opacity-70">
						{projectStore.projects.reduce((acc, p) => acc + (p.taskCount || 0), 0)}
					</span>
				{/if}
			</div>

			<div class="bg-border/50 mx-2 my-2 h-px"></div>

			<!-- Project List -->
			{#if projectStore.isLoading}
				<div class="text-muted-foreground p-4 text-center text-xs">
					{$_('common.loading')}
				</div>
			{:else if projectStore.projects.length === 0}
				<div class="px-4 py-8 text-center">
					<p class="text-muted-foreground mb-3 text-xs">{$_('projects.noProjects')}</p>
					<button
						type="button"
						onclick={(e) => handleNewProject(e as unknown as MouseEvent)}
						class="text-primary text-xs hover:underline"
					>
						{$_('projects.form.createTitle')}
					</button>
				</div>
			{:else}
				{#each projectStore.projects as project, index (project.id)}
					<div
						class="group relative"
						transition:slide={{ duration: 200, axis: 'y' }}
						animate:flip={{ duration: 300 }}
						onmouseenter={() => (hoveredProjectId = project.id)}
						onmouseleave={() => (hoveredProjectId = null)}
						role="group"
						draggable="true"
						ondragstart={(e) => {
							if (e.dataTransfer) {
								e.dataTransfer.setData(
									'application/json',
									JSON.stringify({ type: 'project', id: project.id })
								);
								e.dataTransfer.effectAllowed = 'move';
							}
						}}
						ondragover={(e) => {
							// Allow dropping projects on other projects for reordering
							// But also allow dropping tasks on projects (handled by handleDragOver)
							// We need to distinguish or handle both.
							// For project reordering, we check the data type if possible, but data is protected.
							// We can just allow it and check in drop.
							handleDragOver(e, project.id);
						}}
						ondragleave={handleDragLeave}
						ondrop={async (e) => {
							e.preventDefault();
							dragOverProjectId = null;

							// Check if it's a project being dragged
							const projectDataStr = e.dataTransfer?.getData('application/json');
							if (projectDataStr) {
								try {
									const data = JSON.parse(projectDataStr);
									if (data.type === 'project' && data.id !== project.id) {
										// Reorder projects
										const currentProjects = [...projectStore.projects];
										const draggedIndex = currentProjects.findIndex((p) => p.id === data.id);
										const targetIndex = index;

										if (draggedIndex !== -1) {
											const [draggedProject] = currentProjects.splice(draggedIndex, 1);
											currentProjects.splice(targetIndex, 0, draggedProject);

											const newProjectIds = currentProjects.map((p) => p.id);
											await projectStore.reorder(newProjectIds);
										}
										return; // Stop here if it was a project reorder
									}
								} catch (e) {
									// Not a JSON or not a project, continue to task drop logic
								}
							}

							// Fallback to task drop logic (assigning task to project)
							handleDrop(e, project.id);
						}}
					>
						<div
							role="button"
							tabindex="0"
							onclick={() => handleSelectProject(project.id)}
							onkeydown={(e) => e.key === 'Enter' && handleSelectProject(project.id)}
							class="flex w-full cursor-pointer items-center gap-3 rounded-md border border-transparent px-3 py-2 text-left text-sm transition-all"
							class:bg-muted={selectedProjectId === project.id && dragOverProjectId !== project.id}
							class:hover:bg-muted={selectedProjectId !== project.id &&
								dragOverProjectId !== project.id}
							class:bg-accent={dragOverProjectId === project.id}
							class:border-primary={dragOverProjectId === project.id}
						>
							<!-- Color Indicator -->
							<div
								class="h-3 w-3 rounded-full border border-black/10 shadow-sm dark:border-white/10"
								style="background-color: {project.color}"
							></div>

							<span class="flex-1 truncate font-medium opacity-90">{project.name}</span>

							<!-- Task Count Badge -->
							{#if (project.taskCount || 0) > 0}
								<span
									class="bg-background/50 text-muted-foreground rounded-full px-1.5 py-0.5 text-xs"
								>
									{project.taskCount}
								</span>
							{/if}
						</div>

						<!-- Actions Menu (visible on hover) -->
						{#if hoveredProjectId === project.id}
							<div
								class="bg-muted/80 absolute top-1/2 right-2 flex -translate-y-1/2 gap-1 rounded-md p-0.5 shadow-sm backdrop-blur-sm"
							>
								<button
									type="button"
									onclick={(e) => {
										e.stopPropagation();
										onEditProject?.(project);
									}}
									class="hover:bg-background text-muted-foreground hover:text-foreground rounded-sm p-1 transition-colors"
									title="Edit"
								>
									<Edit2 class="h-3 w-3" />
								</button>
								<button
									type="button"
									onclick={(e) => {
										e.stopPropagation();
										onDeleteProject?.(project);
									}}
									class="hover:bg-destructive hover:text-destructive-foreground text-muted-foreground rounded-sm p-1 transition-colors"
									title="Delete"
								>
									<Trash2 class="h-3 w-3" />
								</button>
							</div>
						{/if}
					</div>
				{/each}
			{/if}
		</div>
	{/if}
</div>
