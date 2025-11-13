<script lang="ts">
	// Composant d'affichage du timer (temps restant et progression)
	import { timerStore } from '$lib/stores/timer.svelte';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { _ } from 'svelte-i18n';

	// Props
	interface Props {
		showProgress?: boolean;
	}

	let { showProgress = true }: Props = $props();

	// Tâche courante liée au timer
	const currentTask = $derived(() => {
		if (!timerStore.taskId) return null;
		return taskStore.tasks.find(t => t.id === timerStore.taskId);
	});

	// Couleur selon le type de session
	const sessionColors = {
		work: 'text-red-600 dark:text-red-400',
		short_break: 'text-green-600 dark:text-green-400',
		long_break: 'text-blue-600 dark:text-blue-400'
	};

	const progressColors = {
		work: 'stroke-red-600',
		short_break: 'stroke-green-600',
		long_break: 'stroke-blue-600'
	};
</script>

<div class="flex flex-col items-center gap-4">
	<!-- Type de session -->
	<div class="text-sm font-medium uppercase tracking-wide opacity-60">
		{$_(`timer.${timerStore.sessionType}`)}
	</div>

	<!-- Tâche courante si liée au timer -->
	{#if currentTask()}
		{@const task = currentTask()}
		{#if task}
			<div class="mb-2 rounded-lg bg-muted px-4 py-3 text-center max-w-md">
				<div class="text-xs text-muted-foreground mb-1">{$_('timer.workingOn')}</div>
				<div class="font-semibold text-base">{task.title}</div>
				{#if task.estimatedPomodoros > 0}
					<div class="mt-2 text-xs opacity-70">
						{task.completedPomodoros}/{task.estimatedPomodoros} 🍅
					</div>
				{/if}
			</div>
		{/if}
	{/if}

	<!-- Affichage circulaire du temps -->
	<div class="relative">
		{#if showProgress}
			<!-- Cercle de progression SVG -->
			<svg class="w-64 h-64 transform -rotate-90" viewBox="0 0 100 100">
				<!-- Cercle de fond -->
				<circle
					cx="50"
					cy="50"
					r="45"
					fill="none"
					stroke="currentColor"
					stroke-width="8"
					class="opacity-10"
				/>

				<!-- Cercle de progression -->
				<circle
					cx="50"
					cy="50"
					r="45"
					fill="none"
					stroke="currentColor"
					stroke-width="8"
					stroke-linecap="round"
					class={progressColors[timerStore.sessionType]}
					stroke-dasharray="283"
					stroke-dashoffset={283 * (1 - timerStore.progress() / 100)}
					style="transition: stroke-dashoffset 1s linear"
				/>
			</svg>
		{/if}

		<!-- Temps au centre -->
		<div
			class="absolute inset-0 flex flex-col items-center justify-center"
		>
			<div class={`text-6xl font-mono font-bold tabular-nums ${sessionColors[timerStore.sessionType]}`}>
				{timerStore.displayTime()}
			</div>

			<!-- Compteur de Pomodoros -->
			{#if timerStore.pomodoroCount > 0}
				<div class="mt-2 text-sm opacity-60">
					{$_('timer.pomodoroCount', { values: { count: timerStore.pomodoroCount } })}
				</div>
			{/if}
		</div>
	</div>
</div>
