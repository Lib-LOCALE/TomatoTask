<script lang="ts">
	// Formulaire pour créer/éditer un projet
	import { _ } from 'svelte-i18n';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import type { Project } from '$lib/types';

	// Props
	interface Props {
		project?: Project | null;
		onSave: (name: string, color: string) => Promise<void>;
		onCancel: () => void;
	}

	let { project = null, onSave, onCancel }: Props = $props();

	// État du formulaire
	let name = $state(project?.name || '');
	let color = $state(project?.color || '#3b82f6');
	let isSaving = $state(false);
	let error = $state<string | null>(null);

	// Couleurs prédéfinies
	const predefinedColors = [
		'#3b82f6', // blue
		'#10b981', // green
		'#f59e0b', // amber
		'#ef4444', // red
		'#8b5cf6', // violet
		'#ec4899', // pink
		'#06b6d4', // cyan
		'#f97316' // orange
	];

	/**
	 * Gère la soumission du formulaire
	 */
	async function handleSubmit(event: Event) {
		event.preventDefault();
		error = null;

		// Validation
		if (!name.trim()) {
			error = $_('projects.errors.nameRequired');
			return;
		}

		if (name.length > 50) {
			error = $_('projects.errors.nameTooLong');
			return;
		}

		isSaving = true;
		try {
			await onSave(name.trim(), color);
		} catch (e) {
			console.error('Failed to save project:', e);
			error = $_('projects.errors.saveFailed');
		} finally {
			isSaving = false;
		}
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4">
	<!-- Nom du projet -->
	<div>
		<label for="project-name" class="block text-sm font-medium mb-1">
			{$_('projects.form.name')}
		</label>
		<Input
			id="project-name"
			type="text"
			bind:value={name}
			placeholder={$_('projects.form.namePlaceholder')}
			maxlength={50}
			required
		/>
	</div>

	<!-- Couleur du projet -->
	<div>
		<label class="block text-sm font-medium mb-2">
			{$_('projects.form.color')}
		</label>
		<div class="flex flex-wrap gap-2">
			{#each predefinedColors as presetColor}
				<button
					type="button"
					onclick={() => (color = presetColor)}
					class="h-8 w-8 rounded-md border-2 transition-all hover:scale-110"
					class:ring-2={color === presetColor}
					class:ring-offset-2={color === presetColor}
					class:ring-primary={color === presetColor}
					style="background-color: {presetColor}"
					title={presetColor}
				/>
			{/each}

			<!-- Sélecteur de couleur personnalisé -->
			<div class="relative">
				<input
					type="color"
					bind:value={color}
					class="h-8 w-8 rounded-md border-2 cursor-pointer"
					title={$_('projects.form.customColor')}
				/>
			</div>
		</div>

		<!-- Aperçu de la couleur sélectionnée -->
		<div class="mt-2 flex items-center gap-2 text-sm text-muted-foreground">
			<div class="h-4 w-4 rounded-full" style="background-color: {color}"></div>
			<span>{color}</span>
		</div>
	</div>

	<!-- Message d'erreur -->
	{#if error}
		<div class="rounded-md bg-destructive/10 border border-destructive/20 p-3 text-sm text-destructive">
			{error}
		</div>
	{/if}

	<!-- Boutons d'action -->
	<div class="flex justify-end gap-2 pt-2">
		<Button type="button" variant="outline" onclick={onCancel} disabled={isSaving}>
			{$_('common.cancel')}
		</Button>

		<Button type="submit" disabled={isSaving}>
			{#if isSaving}
				{$_('common.loading')}
			{:else if project}
				{$_('common.save')}
			{:else}
				{$_('projects.form.create')}
			{/if}
		</Button>
	</div>
</form>
