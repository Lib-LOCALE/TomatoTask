<script lang="ts">
	// Composant principal du timer Pomodoro
	import { onMount } from 'svelte';
	import { timerStore } from '$lib/stores/timer.svelte';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { onSessionCompleteCallback, startNextSession } from '$lib/services/timer-service';
	import { notifyComplete } from '$lib/services/notification-service';
	import TimerDisplay from './TimerDisplay.svelte';
	import TimerControls from './TimerControls.svelte';
	import TaskSelectDisplay from './TaskSelectDisplay.svelte';
	import SoundControl from '$lib/components/sounds/SoundControl.svelte';
	import type { SessionType, Task } from '$lib/types';
	import { _ } from 'svelte-i18n';

	// Props
	interface Props {
		autoAdvance?: boolean;
	}

	let { autoAdvance = false }: Props = $props();

	// État local pour la tâche sélectionnée
	let selectedTaskId = $state<number | undefined>(undefined);

	// Synchronise selectedTaskId avec taskStore.selectedTask
	$effect(() => {
		selectedTaskId = taskStore.selectedTask?.id;
	});

	// État pour le feedback visuel de complétion
	let showCompletionFeedback = $state(false);
	let completedSessionType = $state<SessionType>('work');

	/**
	 * Gère la complétion d'une session
	 *
	 * 1. Affiche un feedback visuel de célébration
	 * 2. Envoie une notification
	 * 3. Démarre automatiquement la prochaine session si configuré
	 */
	async function handleSessionComplete(sessionType: SessionType) {
		// Affiche le feedback de complétion
		completedSessionType = sessionType;
		showCompletionFeedback = true;

		// Cache le feedback après 4 secondes
		setTimeout(() => {
			showCompletionFeedback = false;
		}, 4000);
		// Notification desktop + son
		await notifyComplete(sessionType);

		// Auto-advance si configuré
		if (autoAdvance) {
			const settings = settingsStore.settings;

			// Vérifie les paramètres d'auto-start
			const shouldAutoStart =
				(sessionType === 'work' && settings.autoStartBreaks) ||
				(sessionType !== 'work' && settings.autoStartPomodoros);

			if (shouldAutoStart) {
				// Attend 3 secondes avant de démarrer la prochaine session
				setTimeout(async () => {
					try {
						await startNextSession(selectedTaskId);
					} catch (error) {
						console.error('Failed to auto-start next session:', error);
					}
				}, 3000);
			}
		}
	}

	onMount(() => {
		// Charge les paramètres au montage
		settingsStore.load();

		// Enregistre le callback de complétion
		onSessionCompleteCallback(handleSessionComplete);
	});
</script>

<div class="flex min-h-screen flex-col items-center justify-center p-8">
	<!-- Conteneur principal -->
	<div class="w-full max-w-2xl">
		<!-- Feedback de complétion animé -->
		{#if showCompletionFeedback}
			<div
				class="mb-4 animate-bounce rounded-lg border-2 p-4 text-center shadow-lg"
				class:border-green-500={completedSessionType === 'work'}
				class:bg-green-50={completedSessionType === 'work'}
				class:dark:bg-green-950={completedSessionType === 'work'}
				class:border-blue-500={completedSessionType !== 'work'}
				class:bg-blue-50={completedSessionType !== 'work'}
				class:dark:bg-blue-950={completedSessionType !== 'work'}
			>
				<div class="mb-2 text-2xl">
					{#if completedSessionType === 'work'}
						🎉
					{:else}
						✨
					{/if}
				</div>
				<div class="text-lg font-semibold">
					{$_('timer.completed')}
				</div>
				<div class="mt-1 text-sm opacity-75">
					{#if completedSessionType === 'work'}
						{$_('notifications.workComplete')}
					{:else}
						{$_('notifications.breakComplete')}
					{/if}
				</div>
			</div>
		{/if}

		<!-- Carte du timer -->
		<div class="bg-card text-card-foreground rounded-lg border p-8 shadow-lg">
			<!-- Affichage du timer -->
			<TimerDisplay showProgress={true} />

			<!-- Espacement -->
			<div class="h-8"></div>

			<!-- Sélecteur de tâche (visible uniquement quand le timer n'est pas actif) -->
			{#if timerStore.remainingSeconds === 0}
				<div class="mb-6">
					<TaskSelectDisplay
						{selectedTaskId}
						onTaskSelected={(task) => (selectedTaskId = task?.id)}
					/>
				</div>
			{/if}

			<!-- Contrôles du timer -->
			<TimerControls
				{selectedTaskId}
				onSessionStart={(type) => console.log('Session started:', type)}
				onSessionStop={() => console.log('Session stopped')}
			/>

			<!-- Contrôle des sons d'ambiance -->
			<div class="mt-6 border-t pt-6">
				<SoundControl />
			</div>
		</div>

		<!-- Informations supplémentaires -->
		<div class="text-muted-foreground mt-8 text-center text-sm">
			<p class="mb-1">
				{@html $_('keyboard.pressToStart')}
			</p>
			<p>
				{@html $_('keyboard.pressForShortcuts')}
			</p>
		</div>
	</div>
</div>
