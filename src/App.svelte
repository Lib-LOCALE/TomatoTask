<script lang="ts">
	// Application principale TomatoTask
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
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
	import OnboardingFlow from '$lib/components/onboarding/OnboardingFlow.svelte';
	import { timerStore } from '$lib/stores/timer.svelte';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { projectStore } from '$lib/stores/projects.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { startSession, pauseTimer, resumeTimer } from '$lib/services/timer-service';
	import { initializeTasks, deleteTask, selectTask } from '$lib/services/task-service';
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

	// État de l'onboarding
	let showOnboarding = $state(false);

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

		// Auto-select first task of the project if available
		const tasks = taskStore.filteredTasks;
		if (tasks.length > 0) {
			selectTask(tasks[0]);
		} else {
			selectTask(null);
		}
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

	/**
	 * Gère la complétion de l'onboarding
	 */
	function handleOnboardingComplete() {
		showOnboarding = false;
	}

	/**
	 * Gère la création d'une nouvelle tâche
	 * Sélectionne automatiquement la tâche dans le timer
	 */
	function handleTaskCreated(task: Task) {
		selectTask(task);
	}

	onMount(async () => {
		// Vérifie si l'onboarding a déjà été complété
		const onboardingCompleted = localStorage.getItem('tomatotask_onboarding_completed');
		if (!onboardingCompleted) {
			showOnboarding = true;
		}

		// Charge les paramètres, tâches et projets au démarrage
		await settingsStore.load();
		await initializeTasks();
		await projectStore.load();

		// Auto-select first task if none selected
		if (!taskStore.selectedTask && taskStore.tasks.length > 0) {
			selectTask(taskStore.tasks[0]);
		}

		// Enregistre les raccourcis clavier
		registerShortcut('s', handleStartStopShortcut, true); // Ctrl+S
		registerShortcut('n', handleNewTaskShortcut, true); // Ctrl+N
		registerShortcut('l', handleLanguageShortcut, true); // Ctrl+L
		registerShortcut('/', handleShortcutsHelpShortcut, true); // Ctrl+/
	});
</script>

<main class="bg-background text-foreground flex h-screen overflow-hidden">
	<!-- Sidebar des projets et tâches (1/3 de l'écran) -->
	<aside class="flex w-1/3 flex-col border-r">
		<!-- Liste des projets (1/3 du sidebar) -->
		{#if settingsStore.settings.enableProjects}
			<div class="h-1/3 border-b">
				<ProjectList
					onProjectSelect={handleProjectSelect}
					onNewProject={handleNewProject}
					onEditProject={handleEditProject}
					onDeleteProject={handleDeleteProject}
				/>
			</div>
		{/if}

		<!-- Liste des tâches (2/3 du sidebar) -->
		<div class="flex-1">
			<TaskList
				onNewTask={handleNewTask}
				onEditTask={handleEditTask}
				onDeleteTask={handleDeleteTask}
				onSelectTask={(task) => selectTask(task)}
			/>
		</div>
	</aside>

	<!-- Zone principale du timer et statistiques (2/3 de l'écran) -->
	<section class="relative flex-1 overflow-y-auto">
		<!-- Boutons flottants (floating top-right) -->
		<div class="absolute top-4 right-4 z-10 flex gap-2">
			<!-- Bouton paramètres -->
			<button
				type="button"
				onclick={() => (showSettings = true)}
				class="hover:bg-muted rounded-md p-2 transition-colors"
				title="Settings"
			>
				<svg class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
					></path>
					<path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
					></path>
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
	onTaskCreated={handleTaskCreated}
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
		class="border-border bg-background fixed top-1/2 left-1/2 z-50 m-0 max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg border p-6 shadow-2xl"
		style="backdrop-filter: blur(4px);"
	>
		<h2 class="mb-4 text-lg font-semibold">{$_('dialogs.deleteTaskTitle')}</h2>
		<p class="text-muted-foreground mb-6">
			{$_('dialogs.deleteTaskConfirm', { values: { name: taskToDelete.title } })}
		</p>

		<div class="flex justify-end gap-2">
			<button
				type="button"
				onclick={() => {
					showDeleteConfirm = false;
					taskToDelete = null;
				}}
				class="hover:bg-muted rounded-md border px-4 py-2"
			>
				{$_('common.cancel')}
			</button>

			<button
				type="button"
				onclick={confirmDelete}
				class="bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded-md px-4 py-2"
			>
				{$_('common.delete')}
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
		class="border-border bg-background fixed top-1/2 left-1/2 z-50 m-0 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg border p-6 shadow-2xl"
		style="backdrop-filter: blur(4px);"
	>
		<div class="w-full max-w-md">
			<!-- Header -->
			<div class="mb-4 flex items-center justify-between border-b pb-4">
				<h2 class="text-lg font-semibold">{$_('dialogs.languageTitle')}</h2>

				<button
					type="button"
					onclick={() => (showLanguageSelector = false)}
					class="hover:bg-muted rounded-md p-1"
				>
					<svg
						class="h-5 w-5"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						viewBox="0 0 24 24"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
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
					class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-4 py-2"
				>
					{$_('dialogs.done')}
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
		class="border-border bg-background fixed top-1/2 left-1/2 z-50 m-0 max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg border p-6 shadow-2xl"
		style="backdrop-filter: blur(4px);"
	>
		<h2 class="mb-4 text-lg font-semibold">{$_('dialogs.deleteProjectTitle')}</h2>
		<p class="text-muted-foreground mb-6">
			{$_('dialogs.deleteProjectConfirm', { values: { name: projectToDelete.name } })}
		</p>

		<div class="flex justify-end gap-2">
			<button
				type="button"
				onclick={() => {
					showProjectDeleteConfirm = false;
					projectToDelete = null;
				}}
				class="hover:bg-muted rounded-md border px-4 py-2"
			>
				{$_('common.cancel')}
			</button>

			<button
				type="button"
				onclick={confirmProjectDelete}
				class="bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded-md px-4 py-2"
			>
				{$_('common.delete')}
			</button>
		</div>
	</dialog>
{/if}

<!-- Onboarding Flow (affiché uniquement à la première ouverture) -->
{#if showOnboarding}
	<OnboardingFlow onComplete={handleOnboardingComplete} />
{/if}
