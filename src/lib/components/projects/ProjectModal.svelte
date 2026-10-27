<script lang="ts">
	// Modal pour créer/éditer un projet
	import { _ } from 'svelte-i18n';
	import { projectStore } from '$lib/stores/projects.svelte';
	import ProjectForm from './ProjectForm.svelte';
	import type { Project } from '$lib/types';

	// Props
	interface Props {
		isOpen: boolean;
		project?: Project | null;
		onClose: () => void;
	}

	let { isOpen = $bindable(), project = null, onClose }: Props = $props();

	/**
	 * Gère la sauvegarde du projet
	 */
	async function handleSave(name: string, color: string) {
		try {
			if (project) {
				// Mode édition
				await projectStore.update(project.id, name, color);
			} else {
				// Mode création
				await projectStore.create(name, color);
			}
			handleClose();
		} catch (error) {
			// L'erreur sera gérée par le formulaire
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

	let dialogElement: HTMLDialogElement;
	$effect(() => {
		if (!dialogElement) return;
		if (isOpen && !dialogElement.open) dialogElement.showModal();
		else if (!isOpen && dialogElement.open) dialogElement.close();
	});
</script>

<dialog
	bind:this={dialogElement}
	oncancel={(event) => {
		event.preventDefault();
		handleClose();
	}}
	onclick={(event) => {
		if (event.target === dialogElement) handleClose();
	}}
	class="border-border bg-background fixed top-1/2 left-1/2 z-50 m-0 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg border p-0 shadow-2xl"
>
	<div class="w-full" role="none">
		<!-- Header -->
		<div class="flex items-center justify-between border-b px-6 py-4">
			<h2 class="text-foreground text-lg font-semibold">
				{project ? $_('projects.form.editTitle') : $_('projects.form.createTitle')}
			</h2>

			<button
				type="button"
				onclick={handleClose}
				class="hover:bg-muted rounded-md p-1"
				title={$_('common.close')}
				aria-label={$_('common.close')}
			>
				<svg class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
				</svg>
			</button>
		</div>

		<!-- Contenu: Formulaire -->
		<div class="px-6 py-4">
			{#if isOpen}<ProjectForm {project} onSave={handleSave} onCancel={handleClose} />{/if}
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
	}
</style>
