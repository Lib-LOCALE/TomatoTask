<script lang="ts">
	// Composant d'affichage du timer (temps restant et progression)
	import { timerStore } from '$lib/stores/timer.svelte';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import AnimatedIcon from '$lib/components/ui/AnimatedIcon.svelte';
	import { _ } from 'svelte-i18n';

	// Props
	interface Props {
		showProgress?: boolean;
	}

	let { showProgress = true }: Props = $props();

	// Tâche courante liée au timer
	const currentTask = $derived(() => {
		if (!timerStore.taskId) return null;
		return taskStore.tasks.find((t) => t.id === timerStore.taskId);
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
	<div class="text-sm font-medium tracking-wide uppercase opacity-60">
		{$_(`timer.${timerStore.sessionType}`)}
	</div>

	<!-- Tâche courante si liée au timer -->
	{#if currentTask()}
		{@const task = currentTask()}
		{#if task}
			<div class="bg-muted mb-2 max-w-md rounded-lg px-4 py-3 text-center">
				<div class="text-muted-foreground mb-1 text-xs">{$_('timer.workingOn')}</div>
				<div class="text-base font-semibold">{task.title}</div>
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
			<svg class="h-64 w-64 -rotate-90 transform" viewBox="0 0 100 100">
				<!-- Cercle de fond -->
				<circle
					cx="50"
					cy="50"
					r="45"
					fill="none"
					stroke="currentColor"
					stroke-width="8"
					class="opacity-10"
				></circle>

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
				></circle>
			</svg>
		{/if}

		<!-- Temps au centre -->
		<div class="absolute inset-0 flex flex-col items-center justify-center">
			<!-- Tomate animée quand le timer est actif -->
			{#if timerStore.isActive}
				<div class="mb-2">
					{#if timerStore.sessionType === 'work'}
						<AnimatedIcon name="tomato-focused" size={64} />
					{:else}
						<AnimatedIcon name="tomato-relaxing" size={64} />
					{/if}
				</div>
			{/if}

			<div
				class={`font-mono text-6xl font-bold tabular-nums ${sessionColors[timerStore.sessionType]}`}
			>
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
