<script lang="ts">
	// Liste des projets avec sélection pour filtrage
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { projectStore } from '$lib/stores/projects.svelte';
	import type { Project } from '$lib/types';

	// Props
	interface Props {
		onProjectSelect?: (projectId: number | null) => void;
		onNewProject?: () => void;
		onEditProject?: (project: Project) => void;
		onDeleteProject?: (project: Project) => void;
	}

	let { onProjectSelect, onNewProject, onEditProject, onDeleteProject }: Props = $props();

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
	function handleNewProject() {
		onNewProject?.();
	}

	onMount(async () => {
		// Charge les projets au montage
		await projectStore.load();
	});
</script>

<div class="flex flex-col">
	<!-- Header avec bouton "New Project" -->
	<div class="flex items-center justify-between p-4 border-b">
		<h2 class="text-lg font-semibold">{$_('projects.title')}</h2>
		<button
			type="button"
			onclick={handleNewProject}
			class="rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90 font-bold text-lg"
			title={$_('projects.newProject')}
			aria-label={$_('projects.newProject')}
		>
			+
		</button>
	</div>

	<!-- Liste des projets -->
	<div class="flex-1 overflow-y-auto">
		{#if projectStore.isLoading}
			<div class="p-4 text-center text-sm text-muted-foreground">
				{$_('common.loading')}
			</div>
		{:else if projectStore.projects.length === 0}
			<div class="p-4 text-center text-sm text-muted-foreground">
				<p>{$_('projects.noProjects')}</p>
			</div>
		{:else}
			<!-- Option "All Projects" -->
			<button
				type="button"
				onclick={() => handleSelectProject(null)}
				class="w-full px-4 py-2 text-left text-sm hover:bg-muted transition-colors"
				class:bg-muted={selectedProjectId === null}
				class:font-medium={selectedProjectId === null}
			>
				<div class="flex items-center gap-2">
					<div class="h-3 w-3 rounded-full bg-gradient-to-br from-gray-400 to-gray-600"></div>
					<span>{$_('projects.allProjects')}</span>
				</div>
			</button>

			<!-- Projets individuels -->
			{#each projectStore.projects as project (project.id)}
				<div
					class="group relative px-4 py-2 hover:bg-muted transition-colors"
					class:bg-muted={selectedProjectId === project.id}
				>
					<button
						type="button"
						onclick={() => handleSelectProject(project.id)}
						class="w-full text-left text-sm"
						class:font-medium={selectedProjectId === project.id}
					>
						<div class="flex items-center gap-2">
							<div class="h-3 w-3 rounded-full" style="background-color: {project.color}"></div>
							<span class="flex-1 truncate">{project.name}</span>
							<span class="text-xs text-muted-foreground">{project.taskCount}</span>
						</div>
					</button>

					<!-- Actions (visible au hover) -->
					<div class="absolute right-2 top-2 hidden group-hover:flex gap-1">
						<button
							type="button"
							onclick={(e) => {
								e.stopPropagation();
								onEditProject?.(project);
							}}
							class="rounded p-1 hover:bg-background"
							title="Edit project"
						>
							<svg class="h-3 w-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
								/>
							</svg>
						</button>

						<button
							type="button"
							onclick={(e) => {
								e.stopPropagation();
								onDeleteProject?.(project);
							}}
							class="rounded p-1 hover:bg-destructive hover:text-destructive-foreground"
							title="Delete project"
						>
							<svg class="h-3 w-3" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
								/>
							</svg>
						</button>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>
