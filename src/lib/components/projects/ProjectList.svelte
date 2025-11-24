<script lang="ts">
	// Liste des projets avec sélection pour filtrage
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { projectStore } from '$lib/stores/projects.svelte';
	import { updateTask } from '$lib/services/task-service';
	import { taskStore } from '$lib/stores/tasks.svelte';

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

<div class="flex flex-col h-full">
	<!-- Section Header (Collapsible) -->
	<button
		class="flex items-center justify-between p-4 w-full hover:bg-muted/50 transition-colors group"
		onclick={() => (isExpanded = !isExpanded)}
	>
		<div class="flex items-center gap-2 font-semibold text-sm text-muted-foreground group-hover:text-foreground transition-colors">
			{#if isExpanded}
				<ChevronDown class="h-4 w-4" />
			{:else}
				<ChevronRight class="h-4 w-4" />
			{/if}
			{$_('projects.title')}
		</div>
		
		<button
			type="button"
			onclick={handleNewProject}
			class="opacity-0 group-hover:opacity-100 p-1 hover:bg-background rounded-md transition-all"
			title={$_('projects.newProject')}
		>
			<Plus class="h-4 w-4" />
		</button>
	</button>

	{#if isExpanded}
		<div class="flex-1 overflow-y-auto px-2 pb-2 space-y-1">
			<!-- All Tasks / Inbox -->
			<div
				role="button"
				tabindex="0"
				onclick={() => handleSelectProject(null)}
				onkeydown={(e) => e.key === 'Enter' && handleSelectProject(null)}
				ondragover={(e) => handleDragOver(e, null)}
				ondragleave={handleDragLeave}
				ondrop={(e) => handleDrop(e, null)}
				class="w-full px-3 py-2 text-left text-sm rounded-md transition-all flex items-center gap-3 group cursor-pointer border border-transparent"
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
						{projectStore.projects.reduce((acc, p) => acc + p.taskCount, 0)}
					</span>
				{/if}
			</div>

			<div class="h-px bg-border/50 my-2 mx-2"></div>

			<!-- Project List -->
			{#if projectStore.isLoading}
				<div class="p-4 text-center text-xs text-muted-foreground">
					{$_('common.loading')}
				</div>
			{:else if projectStore.projects.length === 0}
				<div class="text-center py-8 px-4">
					<p class="text-xs text-muted-foreground mb-3">{$_('projects.noProjects')}</p>
					<button
						type="button"
						onclick={(e) => handleNewProject(e as unknown as MouseEvent)}
						class="text-xs text-primary hover:underline"
					>
						{$_('projects.form.createTitle')}
					</button>
				</div>
			{:else}
				{#each projectStore.projects as project (project.id)}
					<div
						class="group relative"
						onmouseenter={() => (hoveredProjectId = project.id)}
						onmouseleave={() => (hoveredProjectId = null)}
						role="group"
					>
						<div
							role="button"
							tabindex="0"
							onclick={() => handleSelectProject(project.id)}
							onkeydown={(e) => e.key === 'Enter' && handleSelectProject(project.id)}
							ondragover={(e) => handleDragOver(e, project.id)}
							ondragleave={handleDragLeave}
							ondrop={(e) => handleDrop(e, project.id)}
							class="w-full px-3 py-2 text-left text-sm rounded-md transition-all flex items-center gap-3 cursor-pointer border border-transparent"
							class:bg-muted={selectedProjectId === project.id && dragOverProjectId !== project.id}
							class:hover:bg-muted={selectedProjectId !== project.id && dragOverProjectId !== project.id}
							class:bg-accent={dragOverProjectId === project.id}
							class:border-primary={dragOverProjectId === project.id}
						>
							<!-- Color Indicator -->
							<div 
								class="h-3 w-3 rounded-full border border-black/10 dark:border-white/10 shadow-sm" 
								style="background-color: {project.color}"
							></div>
							
							<span class="flex-1 truncate font-medium opacity-90">{project.name}</span>
							
							<!-- Task Count Badge -->
							{#if project.taskCount > 0}
								<span class="text-xs bg-background/50 px-1.5 py-0.5 rounded-full text-muted-foreground">
									{project.taskCount}
								</span>
							{/if}
						</div>

						<!-- Actions Menu (visible on hover) -->
						{#if hoveredProjectId === project.id}
							<div class="absolute right-2 top-1/2 -translate-y-1/2 flex gap-1 bg-muted/80 backdrop-blur-sm rounded-md p-0.5 shadow-sm">
								<button
									type="button"
									onclick={(e) => {
										e.stopPropagation();
										onEditProject?.(project);
									}}
									class="p-1 hover:bg-background rounded-sm text-muted-foreground hover:text-foreground transition-colors"
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
									class="p-1 hover:bg-destructive hover:text-destructive-foreground rounded-sm text-muted-foreground transition-colors"
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
