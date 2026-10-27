<script lang="ts">
	// Formulaire de création/édition de tâche
	import type { Task } from '$lib/types';
	import { _ } from 'svelte-i18n';
	import { untrack } from 'svelte';
	import { localDate } from '$lib/stores/plan.svelte';
	import Tile from '$lib/components/majolica/Tile.svelte';
	import { projectStore } from '$lib/stores/projects.svelte';
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
		planToday: boolean;
	}

	// Initialise avec les valeurs existantes ou par défaut
	let formData = $state<FormData>(
		untrack(() => ({
			title: task?.title || '',
			description: task?.description || '',
			estimatedPomodoros: task?.estimatedPomodoros ?? 1,
			projectId: task?.projectId,
			planToday: task?.plannedFor === localDate()
		}))
	);

	// État du formulaire
	let isSubmitting = $state(false);
	let submitError = $state(false);
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
		submitError = false;
		try {
			await onSubmit(formData);
		} catch (error) {
			console.error('Failed to submit form:', error);
			submitError = true;
		} finally {
			isSubmitting = false;
		}
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4">
	<!-- Titre -->
	<div>
		<label for="task-title" class="text-foreground mb-1 block text-sm font-medium">
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
			<p class="text-destructive mt-1 text-sm">{errors.title}</p>
		{/if}
	</div>

	<!-- Description -->
	<div>
		<label for="task-description" class="text-foreground mb-1 block text-sm font-medium">
			{$_('tasks.description')}
		</label>
		<textarea
			id="task-description"
			bind:value={formData.description}
			placeholder="Optional description..."
			class="border-input bg-background text-foreground ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex min-h-[80px] w-full rounded-md border px-3 py-2 text-sm focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
		></textarea>
	</div>

	<div>
		<label for="task-project" class="text-foreground mb-1 block text-sm font-medium"
			>{$_('projects.title')}</label
		><select
			id="task-project"
			class="border-input bg-background text-foreground w-full rounded-md border px-3 py-2"
			bind:value={formData.projectId}
			><option value={undefined}>{$_('plan.all')}</option
			>{#each projectStore.projects as project}<option value={project.id}>{project.name}</option
				>{/each}</select
		>
	</div>
	<!-- Pomodoros estimés -->
	<div>
		<label for="task-pomodoros" class="text-foreground mb-1 block text-sm font-medium">
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
					class:text-foreground={formData.estimatedPomodoros !== count}
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
				class={errors.estimatedPomodoros ? 'border-destructive w-20' : 'w-20'}
			/>
		</div>
		{#if errors.estimatedPomodoros}
			<p class="text-destructive mt-1 text-sm">{errors.estimatedPomodoros}</p>
		{/if}
	</div>

	<div class="flex gap-px">
		{#each Array(Math.max(0, Math.min(20, Math.floor(formData.estimatedPomodoros || 0)))) as _, i}<span
				class="w-6 border"><Tile position={i} /></span
			>{/each}
	</div>
	<label class="flex items-center gap-2"
		><input type="checkbox" bind:checked={formData.planToday} /> {$_('plan.planToday')}</label
	>
	{#if submitError}<p role="alert" class="text-destructive">{$_('tasks.taskError')}</p>{/if}
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
