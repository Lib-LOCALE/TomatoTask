<script lang="ts">
	// Application principale TomatoTask
	import { onMount } from 'svelte';
	import PomodoroTimer from '$lib/components/timer/PomodoroTimer.svelte';
	import TaskList from '$lib/components/tasks/TaskList.svelte';
	import TaskModal from '$lib/components/tasks/TaskModal.svelte';
	import ProjectList from '$lib/components/projects/ProjectList.svelte';
	import ProjectModal from '$lib/components/projects/ProjectModal.svelte';
	import SummaryView from '$lib/components/summary/SummaryView.svelte';
	import LanguageSelector from '$lib/components/settings/LanguageSelector.svelte';
	import ShortcutsHelp from '$lib/components/keyboard/ShortcutsHelp.svelte';
	import ThemeToggle from '$lib/components/settings/ThemeToggle.svelte';
	import SettingsPanel from '$lib/components/settings/SettingsPanel.svelte';
	import { timerStore } from '$lib/stores/timer.svelte';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { projectStore } from '$lib/stores/projects.svelte';
	import { startSession, pauseTimer, resumeTimer } from '$lib/services/timer-service';
	import { initializeTasks, deleteTask } from '$lib/services/task-service';
	import { registerShortcut } from '$lib/utils/keyboard';
	import type { Task, Project } from '$lib/types';

	// État des modals
	let isModalOpen = $state(false);
	let taskToEdit = $state<Task | undefined>(undefined);
	let showDeleteConfirm = $state(false);
	let taskToDelete = $state<Task | null>(null);
	let showLanguageSelector = $state(false);
	let showShortcutsHelp = $state(false);
	let showSettings = $state(false);

	// État des projets
	let isProjectModalOpen = $state(false);
	let projectToEdit = $state<Project | null>(null);
	let showProjectDeleteConfirm = $state(false);
	let projectToDelete = $state<Project | null>(null);

	/**
	 * Gère le raccourci Ctrl+S (Start/Stop timer)
	 */
	function handleStartStopShortcut() {
		const state = timerStore.getState();

		if (state.remainingSeconds === 0) {
			// Timer non démarré: démarre une session de travail
			startSession('work');
		} else if (state.isRunning) {
			// Timer en cours: met en pause
			pauseTimer();
		} else {
			// Timer en pause: reprend
			resumeTimer();
		}
	}

	/**
	 * Gère le raccourci Ctrl+N (Nouvelle tâche)
	 */
	function handleNewTaskShortcut() {
		taskToEdit = undefined;
		isModalOpen = true;
	}

	/**
	 * Gère le raccourci Ctrl+L (Sélecteur de langue)
	 */
	function handleLanguageShortcut() {
		showLanguageSelector = !showLanguageSelector;
	}

	/**
	 * Gère le raccourci Ctrl+/ (Aide raccourcis clavier)
	 */
	function handleShortcutsHelpShortcut() {
		showShortcutsHelp = !showShortcutsHelp;
	}

	/**
	 * Ouvre la modal de création de tâche
	 */
	function handleNewTask() {
		taskToEdit = undefined;
		isModalOpen = true;
	}

	/**
	 * Ouvre la modal d'édition de tâche
	 */
	function handleEditTask(task: Task) {
		taskToEdit = task;
		isModalOpen = true;
	}

	/**
	 * Demande confirmation pour supprimer une tâche
	 */
	function handleDeleteTask(task: Task) {
		taskToDelete = task;
		showDeleteConfirm = true;
	}

	/**
	 * Confirme la suppression de la tâche
	 */
	async function confirmDelete() {
		if (taskToDelete) {
			try {
				await deleteTask(taskToDelete.id);
				showDeleteConfirm = false;
				taskToDelete = null;
			} catch (error) {
				console.error('Failed to delete task:', error);
			}
		}
	}

	/**
	 * Gère la sélection d'un projet pour filtrer les tâches
	 */
	function handleProjectSelect(projectId: number | null) {
		taskStore.setProjectFilter(projectId);
	}

	/**
	 * Ouvre la modal de création de projet
	 */
	function handleNewProject() {
		projectToEdit = null;
		isProjectModalOpen = true;
	}

	/**
	 * Ouvre la modal d'édition de projet
	 */
	function handleEditProject(project: Project) {
		projectToEdit = project;
		isProjectModalOpen = true;
	}

	/**
	 * Demande confirmation pour supprimer un projet
	 */
	function handleDeleteProject(project: Project) {
		projectToDelete = project;
		showProjectDeleteConfirm = true;
	}

	/**
	 * Confirme la suppression du projet
	 */
	async function confirmProjectDelete() {
		if (projectToDelete) {
			try {
				await projectStore.delete(projectToDelete.id);
				showProjectDeleteConfirm = false;
				projectToDelete = null;
			} catch (error) {
				console.error('Failed to delete project:', error);
			}
		}
	}

	onMount(async () => {
		// Charge les tâches et projets au démarrage
		await initializeTasks();
		await projectStore.load();

		// Enregistre les raccourcis clavier
		registerShortcut('s', handleStartStopShortcut, true); // Ctrl+S
		registerShortcut('n', handleNewTaskShortcut, true); // Ctrl+N
		registerShortcut('l', handleLanguageShortcut, true); // Ctrl+L
		registerShortcut('/', handleShortcutsHelpShortcut, true); // Ctrl+/
	});
</script>

<main class="flex h-screen bg-background text-foreground overflow-hidden">
	<!-- Sidebar des projets et tâches (1/3 de l'écran) -->
	<aside class="w-1/3 border-r flex flex-col">
		<!-- Liste des projets (1/3 du sidebar) -->
		<div class="h-1/3 border-b">
			<ProjectList
				onProjectSelect={handleProjectSelect}
				onNewProject={handleNewProject}
				onEditProject={handleEditProject}
				onDeleteProject={handleDeleteProject}
			/>
		</div>

		<!-- Liste des tâches (2/3 du sidebar) -->
		<div class="flex-1">
			<TaskList
				onNewTask={handleNewTask}
				onEditTask={handleEditTask}
				onDeleteTask={handleDeleteTask}
				onSelectTask={(task) => console.log('Task selected:', task)}
			/>
		</div>
	</aside>

	<!-- Zone principale du timer et statistiques (2/3 de l'écran) -->
	<section class="flex-1 overflow-y-auto relative">
		<!-- Boutons flottants (floating top-right) -->
		<div class="absolute top-4 right-4 z-10 flex gap-2">
			<!-- Bouton paramètres -->
			<button
				type="button"
				onclick={() => (showSettings = true)}
				class="rounded-md p-2 hover:bg-muted transition-colors"
				title="Settings"
			>
				<svg
					class="h-5 w-5"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
					/>
					<path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
				</svg>
			</button>

			<!-- Toggle thème -->
			<ThemeToggle variant="button" />
		</div>

		<div class="flex flex-col gap-8">
			<!-- Timer Pomodoro -->
			<div>
				<PomodoroTimer autoAdvance={true} />
			</div>

			<!-- Statistiques et résumés -->
			<div class="px-8 pb-8">
				<SummaryView />
			</div>
		</div>
	</section>
</main>

<!-- Modal de création/édition de tâche -->
<TaskModal
	bind:isOpen={isModalOpen}
	task={taskToEdit}
	onClose={() => {
		isModalOpen = false;
		taskToEdit = undefined;
	}}
/>

<!-- Confirmation de suppression (simple dialog natif) -->
{#if showDeleteConfirm && taskToDelete}
	<dialog
		open
		onclick={(e) => {
			if ((e.target as HTMLElement).tagName === 'DIALOG') {
				showDeleteConfirm = false;
				taskToDelete = null;
			}
		}}
		class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 rounded-lg border border-border bg-background p-6 shadow-2xl m-0 z-50 max-w-md"
		style="backdrop-filter: blur(4px);"
	>
		<h2 class="mb-4 text-lg font-semibold">Delete Task</h2>
		<p class="mb-6 text-muted-foreground">
			Are you sure you want to delete "{taskToDelete.title}"? This action cannot be undone.
		</p>

		<div class="flex justify-end gap-2">
			<button
				type="button"
				onclick={() => {
					showDeleteConfirm = false;
					taskToDelete = null;
				}}
				class="rounded-md border px-4 py-2 hover:bg-muted"
			>
				Cancel
			</button>

			<button
				type="button"
				onclick={confirmDelete}
				class="rounded-md bg-destructive px-4 py-2 text-destructive-foreground hover:bg-destructive/90"
			>
				Delete
			</button>
		</div>
	</dialog>
{/if}

<!-- Sélecteur de langue (accessible avec Ctrl+L) -->
{#if showLanguageSelector}
	<dialog
		open
		onclick={(e) => {
			if ((e.target as HTMLElement).tagName === 'DIALOG') {
				showLanguageSelector = false;
			}
		}}
		class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 rounded-lg border border-border bg-background p-6 shadow-2xl m-0 z-50 max-w-md w-full"
		style="backdrop-filter: blur(4px);"
	>
		<div class="w-full max-w-md">
			<!-- Header -->
			<div class="flex items-center justify-between border-b pb-4 mb-4">
				<h2 class="text-lg font-semibold">Language / Langue</h2>

				<button
					type="button"
					onclick={() => (showLanguageSelector = false)}
					class="rounded-md p-1 hover:bg-muted"
				>
					<svg
						class="h-5 w-5"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						viewBox="0 0 24 24"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Language selector -->
			<LanguageSelector variant="buttons" />

			<!-- Footer -->
			<div class="mt-6 flex justify-end">
				<button
					type="button"
					onclick={() => (showLanguageSelector = false)}
					class="rounded-md bg-primary px-4 py-2 text-primary-foreground hover:bg-primary/90"
				>
					Done
				</button>
			</div>
		</div>
	</dialog>
{/if}

<!-- Aide raccourcis clavier (accessible avec Ctrl+/) -->
<ShortcutsHelp bind:isOpen={showShortcutsHelp} onClose={() => (showShortcutsHelp = false)} />

<!-- Panneau de paramètres (accessible via bouton gear) -->
<SettingsPanel bind:isOpen={showSettings} onClose={() => (showSettings = false)} />

<!-- Modal de création/édition de projet -->
<ProjectModal
	bind:isOpen={isProjectModalOpen}
	project={projectToEdit}
	onClose={() => {
		isProjectModalOpen = false;
		projectToEdit = null;
	}}
/>

<!-- Confirmation de suppression de projet -->
{#if showProjectDeleteConfirm && projectToDelete}
	<dialog
		open
		onclick={(e) => {
			if ((e.target as HTMLElement).tagName === 'DIALOG') {
				showProjectDeleteConfirm = false;
				projectToDelete = null;
			}
		}}
		class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 rounded-lg border border-border bg-background p-6 shadow-2xl m-0 z-50 max-w-md"
		style="backdrop-filter: blur(4px);"
	>
		<h2 class="mb-4 text-lg font-semibold">Delete Project</h2>
		<p class="mb-6 text-muted-foreground">
			Are you sure you want to delete "{projectToDelete.name}"? All tasks will remain but will no
			longer be associated with this project.
		</p>

		<div class="flex justify-end gap-2">
			<button
				type="button"
				onclick={() => {
					showProjectDeleteConfirm = false;
					projectToDelete = null;
				}}
				class="rounded-md border px-4 py-2 hover:bg-muted"
			>
				Cancel
			</button>

			<button
				type="button"
				onclick={confirmProjectDelete}
				class="rounded-md bg-destructive px-4 py-2 text-destructive-foreground hover:bg-destructive/90"
			>
				Delete
			</button>
		</div>
	</dialog>
{/if}
