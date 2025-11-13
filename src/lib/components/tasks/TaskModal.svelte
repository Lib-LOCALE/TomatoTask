<script lang="ts">
	// Modal pour créer ou éditer une tâche
	import { onMount } from 'svelte';
	import type { Task } from '$lib/types';
	import { createTask, updateTask } from '$lib/services/task-service';
	import { _ } from 'svelte-i18n';
	import TaskForm from './TaskForm.svelte';

	// Props
	interface Props {
		isOpen: boolean;
		task?: Task; // Si fourni, mode édition
		onClose: () => void;
	}

	let { isOpen = $bindable(), task, onClose }: Props = $props();

	let dialogElement: HTMLDialogElement;

	/**
	 * Gère la soumission du formulaire
	 */
	async function handleSubmit(data: {
		title: string;
		description: string;
		estimatedPomodoros: number;
		projectId?: number;
	}) {
		try {
			if (task) {
				// Mode édition
				await updateTask(
					task.id,
					data.title,
					data.description,
					data.projectId,
					data.estimatedPomodoros
				);
			} else {
				// Mode création
				await createTask(
					data.title,
					data.description,
					data.projectId,
					data.estimatedPomodoros
				);
			}

			// Ferme le modal
			isOpen = false;
			onClose();
		} catch (error) {
			console.error('Failed to save task:', error);
			throw error;
		}
	}

	/**
	 * Gère la fermeture du modal
	 */
	function handleClose() {
		isOpen = false;
		onClose();
	}

	/**
	 * Gère le clic sur le backdrop
	 */
	function handleBackdropClick(event: MouseEvent) {
		if (event.target === dialogElement) {
			handleClose();
		}
	}

	// Ouvre/ferme le dialog natif selon l'état
	$effect(() => {
		if (!dialogElement) return;

		if (isOpen) {
			dialogElement.showModal();
		} else {
			dialogElement.close();
		}
	});

	onMount(() => {
		// Gère la touche Escape
		dialogElement.addEventListener('cancel', (e) => {
			e.preventDefault();
			handleClose();
		});
	});
</script>

<!-- Dialog natif HTML -->
<dialog
	bind:this={dialogElement}
	onclick={handleBackdropClick}
	class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 rounded-lg border border-border bg-background p-0 shadow-2xl m-0 z-50"
>
	<div class="w-full max-w-md">
		<!-- Header -->
		<div class="flex items-center justify-between border-b px-6 py-4">
			<h2 class="text-lg font-semibold">
				{task ? $_('tasks.editTask') : $_('tasks.newTask')}
			</h2>

			<button
				type="button"
				onclick={handleClose}
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

		<!-- Contenu -->
		<div class="px-6 py-4">
			<TaskForm {task} onSubmit={handleSubmit} onCancel={handleClose} />
		</div>
	</div>
</dialog>

<style>
	dialog {
		position: fixed;
		margin: 0;
	}

	dialog::backdrop {
		background-color: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
	}
</style>
