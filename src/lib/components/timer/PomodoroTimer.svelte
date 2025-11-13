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

	// Props
	interface Props {
		autoAdvance?: boolean;
	}

	let { autoAdvance = false }: Props = $props();

	// État local pour la tâche sélectionnée
	let selectedTaskId = $state<number | undefined>(undefined);

	/**
	 * Gère la complétion d'une session
	 *
	 * 1. Envoie une notification
	 * 2. Démarre automatiquement la prochaine session si configuré
	 */
	async function handleSessionComplete(sessionType: SessionType) {
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
		<div class="mt-6 text-center text-sm text-muted-foreground">
			<p>
				Press <kbd class="px-2 py-1 bg-muted rounded text-xs font-mono">Ctrl+S</kbd> to start/stop
				the timer
			</p>
		</div>
	</div>
</div>
