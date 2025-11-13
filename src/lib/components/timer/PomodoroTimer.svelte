<script lang="ts">
	// Composant principal du timer Pomodoro
	import { onMount } from 'svelte';
	import { timerStore } from '$lib/stores/timer.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { onSessionCompleteCallback, startNextSession } from '$lib/services/timer-service';
	import { notifyComplete } from '$lib/services/notification-service';
	import TimerDisplay from './TimerDisplay.svelte';
	import TimerControls from './TimerControls.svelte';
	import TaskSelector from './TaskSelector.svelte';
	import type { SessionType, Task } from '$lib/types';
	import { _ } from 'svelte-i18n';

	// Props
	interface Props {
		autoAdvance?: boolean;
	}

	let { autoAdvance = false }: Props = $props();

	// État local pour la tâche sélectionnée
	let selectedTaskId = $state<number | undefined>(undefined);

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

<div class="flex flex-col items-center justify-center min-h-screen p-8">
	<!-- Conteneur principal -->
	<div class="w-full max-w-2xl">
		<!-- Feedback de complétion animé -->
		{#if showCompletionFeedback}
			<div
				class="mb-4 rounded-lg border-2 p-4 text-center shadow-lg animate-bounce"
				class:border-green-500={completedSessionType === 'work'}
				class:bg-green-50={completedSessionType === 'work'}
				class:dark:bg-green-950={completedSessionType === 'work'}
				class:border-blue-500={completedSessionType !== 'work'}
				class:bg-blue-50={completedSessionType !== 'work'}
				class:dark:bg-blue-950={completedSessionType !== 'work'}
			>
				<div class="text-2xl mb-2">
					{#if completedSessionType === 'work'}
						🎉
					{:else}
						✨
					{/if}
				</div>
				<div class="font-semibold text-lg">
					{$_('timer.completed')}
				</div>
				<div class="text-sm opacity-75 mt-1">
					{#if completedSessionType === 'work'}
						{$_('notifications.workComplete')}
					{:else}
						{$_('notifications.breakComplete')}
					{/if}
				</div>
			</div>
		{/if}

		<!-- Carte du timer -->
		<div class="bg-card text-card-foreground rounded-lg shadow-lg p-8 border">
			<!-- Affichage du timer -->
			<TimerDisplay showProgress={true} />

			<!-- Espacement -->
			<div class="h-8"></div>

			<!-- Sélecteur de tâche (visible uniquement quand le timer n'est pas actif) -->
			{#if timerStore.remainingSeconds === 0}
				<div class="mb-6">
					<TaskSelector
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
		</div>

		<!-- Informations supplémentaires -->
		<div class="mt-6 text-center text-sm text-muted-foreground space-y-1">
			<p>
				Press <kbd class="px-2 py-1 bg-muted rounded text-xs font-mono">Ctrl+S</kbd> to start/stop
				the timer
			</p>
			<p>
				Press <kbd class="px-2 py-1 bg-muted rounded text-xs font-mono">Ctrl+/</kbd> for all keyboard shortcuts
			</p>
		</div>
	</div>
</div>
