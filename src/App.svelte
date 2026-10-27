<script lang="ts">
	// Application principale TomatoTask
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import PomodoroTimer from '$lib/components/timer/PomodoroTimer.svelte';
	import TaskList from '$lib/components/tasks/TaskList.svelte';
	import TaskModal from '$lib/components/tasks/TaskModal.svelte';
	import ProjectList from '$lib/components/projects/ProjectList.svelte';
	import ProjectModal from '$lib/components/projects/ProjectModal.svelte';
	import LanguageSelector from '$lib/components/settings/LanguageSelector.svelte';
	import ShortcutsHelp from '$lib/components/keyboard/ShortcutsHelp.svelte';
	import ThemeToggle from '$lib/components/settings/ThemeToggle.svelte';
	import SettingsPanel from '$lib/components/settings/SettingsPanel.svelte';
	import OnboardingFlow from '$lib/components/onboarding/OnboardingFlow.svelte';
	import ToastContainer from '$lib/components/effects/ToastContainer.svelte';
	import DayFresco from '$lib/components/majolica/DayFresco.svelte';
	import PlanView from '$lib/components/plan/PlanView.svelte';
	import InsightsView from '$lib/components/insights/InsightsView.svelte';
	import FocusMode from '$lib/components/focus/FocusMode.svelte';
	import { timerStore } from '$lib/stores/timer.svelte';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { projectStore } from '$lib/stores/projects.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { startSession, pauseTimer, resumeTimer } from '$lib/services/timer-service';
	import { initializeTasks, deleteTask, selectTask } from '$lib/services/task-service';
	import { registerShortcut } from '$lib/utils/keyboard';
	import type { Task, Project } from '$lib/types';

	import { WindowService } from '$lib/services/window-service';
	import MiniTimer from '$lib/components/timer/MiniTimer.svelte';
	import { Minimize2, Maximize2, Settings, Menu, X } from '@lucide/svelte';
	import { fade, fly } from 'svelte/transition';

	// État des modals
	let isModalOpen = $state(false);
	let taskToEdit = $state<Task | undefined>(undefined);
	let showDeleteConfirm = $state(false);
	let taskToDelete = $state<Task | null>(null);
	let showLanguageSelector = $state(false);
	let showShortcutsHelp = $state(false);
	let showSettings = $state(false);
	let isMiniMode = $state(false);

	// Vues principales et mode focus immersif
	const views = ['today', 'plan', 'insights'] as const;
	let view = $state<(typeof views)[number]>('today');
	let focusOpen = $state(false);
	let drawerOpen = $state(false);
	let streak = $state(0);

	// État des projets
	let isProjectModalOpen = $state(false);
	let projectToEdit = $state<Project | null>(null);
	let showProjectDeleteConfirm = $state(false);
	let projectToDelete = $state<Project | null>(null);

	// État de l'onboarding
	let showOnboarding = $state(false);

	/**
	 * Bascule entre le mode normal et le mode mini
	 */
	async function toggleMiniMode() {
		isMiniMode = await WindowService.toggleMiniMode(isMiniMode);
	}

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
		registerShortcut('n', handleNewTaskShortcut, true);
		registerShortcut('s', handleStartStopShortcut, true);
		registerShortcut('l', handleLanguageShortcut, true); // Ctrl+L
		registerShortcut('/', handleShortcutsHelpShortcut, true); // Ctrl+/
	});

	// Applique le thème quand il change
	$effect(() => {
		const theme = settingsStore.settings.theme;
		const preference = window.matchMedia('(prefers-color-scheme: dark)');
		const apply = () =>
			document.documentElement.classList.toggle(
				'dark',
				theme === 'dark' || (theme === 'system' && preference.matches)
			);
		apply();
		preference.addEventListener('change', apply);
		return () => preference.removeEventListener('change', apply);
	});
	function nativeModal(node: HTMLDialogElement) {
		node.showModal();
	}
</script>

{#if isMiniMode}
	<MiniTimer onExpand={toggleMiniMode} />
{:else if focusOpen}
	<FocusMode onClose={() => (focusOpen = false)} />
{:else}
	<main class="app-shell" class:focused={timerStore.isRunning && timerStore.sessionType === 'work'}>
		<aside class="rail" class:open={drawerOpen}>
			<div class="brand">
				<span class="logo"
					><svg aria-hidden="true" viewBox="0 0 32 32"
						><path
							d="M16 9 C6 5 3 13 4 20 C5 29 27 29 28 20 C29 13 26 5 16 9Z"
							fill="#f06b55"
						/><path
							d="M16 9 L16 3 M16 9 L9 5 M16 9 L23 5 M16 9 L12 13 M16 9 L20 13"
							fill="none"
							stroke="#a6d8ae"
							stroke-width="2"
						/></svg
					>TomatoTask</span
				><button
					class="close-drawer"
					onclick={() => (drawerOpen = false)}
					aria-label={$_('common.close')}><X size={20} /></button
				>
			</div>
			<p class="streak">{$_('majolica.streak', { values: { count: streak } })}</p>
			{#if settingsStore.settings.enableProjects}<div class="projects">
					<ProjectList
						onProjectSelect={handleProjectSelect}
						onNewProject={handleNewProject}
						onEditProject={handleEditProject}
						onDeleteProject={handleDeleteProject}
					/>
				</div>{/if}
			<div class="tasks">
				<TaskList
					onNewTask={handleNewTask}
					onEditTask={handleEditTask}
					onDeleteTask={handleDeleteTask}
					onSelectTask={(task) => selectTask(task)}
				/>
			</div>
		</aside>
		{#if drawerOpen}<button
				class="drawer-backdrop"
				aria-label={$_('common.close')}
				onclick={() => (drawerOpen = false)}
			></button>{/if}
		<section class="workspace">
			<header class="app-header">
				<button
					class="open-drawer"
					onclick={() => (drawerOpen = true)}
					aria-label={$_('majolica.menu')}><Menu size={20} /></button
				>
				<div class="header-actions">
					<button
						onclick={toggleMiniMode}
						title={$_('majolica.mini')}
						aria-label={$_('majolica.mini')}><Minimize2 size={20} /></button
					><button
						onclick={() => (showSettings = true)}
						title={$_('settings.title')}
						aria-label={$_('settings.title')}><Settings size={20} /></button
					><ThemeToggle variant="button" />
				</div>
			</header>
			<div class="workspace-scroll">
				{#if view === 'today'}
					<div class="main-grid">
						<PomodoroTimer autoAdvance={true} />
						<div class="fresco-panel"><DayFresco onStreak={(days) => (streak = days)} /></div>
					</div>
				{:else if view === 'plan'}
					<PlanView />
				{:else}
					<InsightsView />
				{/if}
			</div>
		</section>
	</main>
{/if}

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
		aria-labelledby="delete-task-title"
		oncancel={() => {
			showDeleteConfirm = false;
			taskToDelete = null;
		}}
		onclick={(e) => {
			if ((e.target as HTMLElement).tagName === 'DIALOG') {
				showDeleteConfirm = false;
				taskToDelete = null;
			}
		}}
		class="border-border bg-background fixed top-1/2 left-1/2 z-50 m-0 max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg border p-6 shadow-2xl"
		transition:fade={{ duration: 200 }}
		use:nativeModal
	>
		<div in:fly={{ y: 20, duration: 300 }}>
			<h2 id="delete-task-title" class="mb-4 text-lg font-semibold">
				{$_('dialogs.deleteTaskTitle')}
			</h2>
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
		</div>
	</dialog>
{/if}

<!-- Sélecteur de langue (accessible avec Ctrl+L) -->
{#if showLanguageSelector}
	<dialog
		aria-labelledby="language-dialog-title"
		oncancel={() => (showLanguageSelector = false)}
		onclick={(e) => {
			if ((e.target as HTMLElement).tagName === 'DIALOG') {
				showLanguageSelector = false;
			}
		}}
		class="border-border bg-background fixed top-1/2 left-1/2 z-50 m-0 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg border p-6 shadow-2xl"
		transition:fade={{ duration: 200 }}
		use:nativeModal
	>
		<div class="w-full max-w-md" in:fly={{ y: 20, duration: 300 }}>
			<!-- Header -->
			<div class="mb-4 flex items-center justify-between border-b pb-4">
				<h2 id="language-dialog-title" class="text-lg font-semibold">
					{$_('dialogs.languageTitle')}
				</h2>

				<button
					type="button"
					onclick={() => (showLanguageSelector = false)}
					class="hover:bg-muted rounded-md p-1"
					aria-label={$_('common.close')}
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
		aria-labelledby="delete-project-title"
		oncancel={() => {
			showProjectDeleteConfirm = false;
			projectToDelete = null;
		}}
		onclick={(e) => {
			if ((e.target as HTMLElement).tagName === 'DIALOG') {
				showProjectDeleteConfirm = false;
				projectToDelete = null;
			}
		}}
		class="border-border bg-background fixed top-1/2 left-1/2 z-50 m-0 max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg border p-6 shadow-2xl"
		transition:fade={{ duration: 200 }}
		use:nativeModal
	>
		<div in:fly={{ y: 20, duration: 300 }}>
			<h2 id="delete-project-title" class="mb-4 text-lg font-semibold">
				{$_('dialogs.deleteProjectTitle')}
			</h2>
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
		</div>
	</dialog>
{/if}

<!-- Onboarding Flow (affiché uniquement à la première ouverture) -->
{#if showOnboarding}
	<OnboardingFlow onComplete={handleOnboardingComplete} />
{/if}

<!-- Toast notifications -->
<ToastContainer />

<style>
	.app-shell {
		display: flex;
		height: 100vh;
		overflow: hidden;
		background: var(--background);
		color: var(--foreground);
	}
	.rail {
		width: 280px;
		flex: none;
		min-height: 0;
		display: flex;
		flex-direction: column;
		background: var(--sidebar);
		color: var(--sidebar-foreground);
		overflow: hidden;
		transition:
			opacity 0.3s,
			filter 0.3s;
	}
	.brand {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 24px 20px 4px;
	}
	.logo {
		display: flex;
		align-items: center;
		gap: 9px;
		font-family: var(--font-heading);
		font-size: 1.3rem;
		font-weight: 700;
	}
	.logo svg {
		width: 30px;
		height: 30px;
	}
	.streak {
		padding: 4px 20px 18px;
		color: #ffe5a4;
		font-weight: 700;
		font-size: 0.9rem;
	}
	.projects {
		flex: 0 1 28%;
		min-height: 120px;
		overflow: auto;
		border-bottom: 1px solid #8298c0;
	}
	.tasks {
		flex: 1 1 0;
		min-height: 0;
		overflow: hidden;
	}
	.rail :global(.bg-card),
	.rail :global(.bg-background) {
		background: #294a84;
		color: white;
	}
	.rail :global(.text-muted-foreground) {
		color: #e0e8f8;
	}
	.rail :global(.bg-muted) {
		background: #385b9a;
		color: white;
	}
	.rail :global(h2) {
		font-size: 1.1rem;
	}
	.rail :global(button) {
		font-size: 0.8rem;
	}
	.rail :global(.px-6) {
		padding-left: 12px;
		padding-right: 12px;
	}
	.rail :global(.p-6) {
		padding: 12px;
	}
	.rail :global(.group > .flex > div:last-child) {
		position: absolute;
		right: 8px;
		top: 8px;
		background: #294a84;
	}
	.rail :global(.border-muted-foreground) {
		border-color: #dce9ff;
	}
	.rail :global(.group:focus-within > .flex > div:last-child) {
		opacity: 1;
	}
	.rail :global(.group h3) {
		font-family: var(--font-body);
		font-size: 0.95rem;
		line-height: 1.3;
	}
	.rail :global(.gap-4) {
		gap: 8px;
	}
	.workspace {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
	}
	.app-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 20px 0;
	}
	.view-tabs {
		display: flex;
		gap: 4px;
	}
	.view-tabs button {
		padding: 8px 14px;
		border-radius: var(--radius);
		font-weight: 600;
		color: var(--muted-foreground);
	}
	.view-tabs button:hover {
		color: var(--foreground);
	}
	.view-tabs button[aria-current='page'] {
		background: var(--primary);
		color: var(--primary-foreground);
	}
	.header-actions {
		display: flex;
		gap: 4px;
		margin-left: auto;
	}
	.header-actions button,
	.open-drawer,
	.close-drawer {
		padding: 8px;
		border-radius: 4px;
	}
	.header-actions :global(button) {
		color: var(--foreground);
	}
	.header-actions button:hover {
		background: var(--muted);
	}
	.workspace-scroll {
		overflow: auto;
		flex: 1;
		min-height: 0;
	}
	.main-grid {
		display: grid;
		grid-template-columns: minmax(0, 1fr) 320px;
		min-height: calc(100vh - 52px);
	}
	.statistics {
		padding: 32px;
		border-top: 1px solid var(--grout);
	}
	.focused .rail,
	.focused .fresco-panel,
	.focused .app-header,
	.focused .statistics {
		opacity: 0.85;
		filter: saturate(0.65);
	}
	.focused .rail:hover,
	.focused .fresco-panel:hover,
	.focused .app-header:hover,
	.focused .statistics:hover,
	.focused .rail:focus-within,
	.focused .app-header:focus-within {
		opacity: 1;
		filter: none;
	}
	.open-drawer,
	.close-drawer,
	.drawer-backdrop {
		display: none;
	}
	@media (max-width: 1099px) {
		.main-grid {
			grid-template-columns: 1fr;
		}
	}
	@media (max-width: 759px) {
		.rail {
			position: fixed;
			z-index: 30;
			inset: 0 auto 0 0;
			transform: translateX(-100%);
			visibility: hidden;
			transition: transform 0.25s;
		}
		.rail.open {
			visibility: visible;
			transform: translateX(0);
		}
		.open-drawer,
		.close-drawer {
			display: block;
		}
		.drawer-backdrop {
			display: block;
			position: fixed;
			z-index: 20;
			inset: 0;
			background: rgba(0, 0, 0, 0.45);
		}
	}
</style>
