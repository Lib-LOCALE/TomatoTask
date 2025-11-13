<script lang="ts">
	// Formulaire de création/édition de tâche
	import type { Task } from '$lib/types';
	import { _ } from 'svelte-i18n';
	import { validateTaskTitle, validatePomodoroCount } from '$lib/utils/validators';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';

	// Props
	interface Props {
		task?: Task; // Si fourni, mode édition
		onSubmit: (data: FormData) => Promise<void>;
		onCancel: () => void;
	}

	let { task, onSubmit, onCancel }: Props = $props();

	// Données du formulaire
	interface FormData {
		title: string;
		description: string;
		estimatedPomodoros: number;
		projectId?: number;
	}

	// Initialise avec les valeurs existantes ou par défaut
	let formData = $state<FormData>({
		title: task?.title || '',
		description: task?.description || '',
		estimatedPomodoros: task?.estimatedPomodoros || 1,
		projectId: task?.projectId
	});

	// État du formulaire
	let isSubmitting = $state(false);
	let errors = $state<Partial<Record<keyof FormData, string>>>({});

	/**
	 * Valide le formulaire
	 */
	function validateForm(): boolean {
		const newErrors: Partial<Record<keyof FormData, string>> = {};

		// Valide le titre
		const titleValidation = validateTaskTitle(formData.title);
		if (titleValidation !== true) {
			newErrors.title = titleValidation;
		}

		// Valide les Pomodoros
		const pomodorosValidation = validatePomodoroCount(formData.estimatedPomodoros);
		if (pomodorosValidation !== true) {
			newErrors.estimatedPomodoros = pomodorosValidation;
		}

		errors = newErrors;
		return Object.keys(newErrors).length === 0;
	}

	/**
	 * Soumet le formulaire
	 */
	async function handleSubmit(event: Event) {
		event.preventDefault();

		if (!validateForm()) {
			return;
		}

		isSubmitting = true;
		try {
			await onSubmit(formData);
		} catch (error) {
			console.error('Failed to submit form:', error);
		} finally {
			isSubmitting = false;
		}
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4">
	<!-- Titre -->
	<div>
		<label for="task-title" class="mb-1 block text-sm font-medium">
			{$_('tasks.taskTitle')} *
		</label>
		<Input
			id="task-title"
			type="text"
			bind:value={formData.title}
			placeholder="Write documentation"
			class={errors.title ? 'border-destructive' : ''}
			required
			autofocus
		/>
		{#if errors.title}
			<p class="mt-1 text-sm text-destructive">{errors.title}</p>
		{/if}
	</div>

	<!-- Description -->
	<div>
		<label for="task-description" class="mb-1 block text-sm font-medium">
			{$_('tasks.description')}
		</label>
		<textarea
			id="task-description"
			bind:value={formData.description}
			placeholder="Optional description..."
			class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
		></textarea>
	</div>

	<!-- Pomodoros estimés -->
	<div>
		<label for="task-pomodoros" class="mb-1 block text-sm font-medium">
			{$_('tasks.estimatedPomodoros')}
		</label>
		<div class="flex items-center gap-2">
			<!-- Boutons rapides -->
			{#each [1, 2, 3, 5] as count}
				<button
					type="button"
					onclick={() => (formData.estimatedPomodoros = count)}
					class="h-10 w-10 rounded-md border transition-colors"
					class:border-primary={formData.estimatedPomodoros === count}
					class:bg-primary={formData.estimatedPomodoros === count}
					class:text-primary-foreground={formData.estimatedPomodoros === count}
				>
					{count}
				</button>
			{/each}

			<!-- Input personnalisé -->
			<Input
				id="task-pomodoros"
				type="number"
				bind:value={formData.estimatedPomodoros}
				min="0"
				max="20"
				class="w-20"
				class:border-destructive={errors.estimatedPomodoros}
			/>
		</div>
		{#if errors.estimatedPomodoros}
			<p class="mt-1 text-sm text-destructive">{errors.estimatedPomodoros}</p>
		{/if}
	</div>

	<!-- Actions -->
	<div class="flex justify-end gap-2 pt-4">
		<Button type="button" variant="outline" onclick={onCancel}>
			{$_('common.cancel')}
		</Button>

		<Button type="submit" disabled={isSubmitting}>
			{isSubmitting ? $_('common.loading') : $_('common.save')}
		</Button>
	</div>
</form>
