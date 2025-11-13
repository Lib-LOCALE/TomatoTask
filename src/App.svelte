<script lang="ts">
	// Application principale TomatoTask
	import { onMount } from 'svelte';
	import PomodoroTimer from '$lib/components/timer/PomodoroTimer.svelte';
	import TaskList from '$lib/components/tasks/TaskList.svelte';
	import TaskModal from '$lib/components/tasks/TaskModal.svelte';
	import { timerStore } from '$lib/stores/timer.svelte';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { startSession, pauseTimer, resumeTimer } from '$lib/services/timer-service';
	import { initializeTasks, deleteTask } from '$lib/services/task-service';
	import { registerShortcut } from '$lib/utils/keyboard';
	import type { Task } from '$lib/types';

	// État de la modal
	let isModalOpen = $state(false);
	let taskToEdit = $state<Task | undefined>(undefined);
	let showDeleteConfirm = $state(false);
	let taskToDelete = $state<Task | null>(null);

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

	onMount(async () => {
		// Charge les tâches au démarrage
		await initializeTasks();

		// Enregistre les raccourcis clavier
		registerShortcut('s', handleStartStopShortcut, true); // Ctrl+S
		registerShortcut('n', handleNewTaskShortcut, true); // Ctrl+N
	});
</script>

<main class="flex h-screen bg-background text-foreground overflow-hidden">
	<!-- Sidebar des tâches (1/3 de l'écran) -->
	<aside class="w-1/3 border-r">
		<TaskList
			onNewTask={handleNewTask}
			onEditTask={handleEditTask}
			onDeleteTask={handleDeleteTask}
			onSelectTask={(task) => console.log('Task selected:', task)}
		/>
	</aside>

	<!-- Zone principale du timer (2/3 de l'écran) -->
	<section class="flex-1">
		<PomodoroTimer autoAdvance={true} />
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
		class="rounded-lg border bg-background p-6 shadow-lg backdrop:bg-black/50"
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
