<script lang="ts">
	// Application principale TomatoTask
	import { onMount } from 'svelte';
	import PomodoroTimer from '$lib/components/timer/PomodoroTimer.svelte';
	import { timerStore } from '$lib/stores/timer.svelte';
	import { startSession, pauseTimer, resumeTimer, stopTimer } from '$lib/services/timer-service';
	import { registerShortcut } from '$lib/utils/keyboard';

	/**
	 * Gère le raccourci Ctrl+S (Start/Stop timer)
	 */
	function handleStartStopShortcut() {
		const state = timerStore.getState();

		if (state.remainingSeconds === 0) {
			// Timer non démarré: démarre une session de travail
			startSession('work');
		} else if (state.isRunning) {
			// Timer en cours: met en pause
			pauseTimer();
		} else {
			// Timer en pause: reprend
			resumeTimer();
		}
	}

	onMount(() => {
		// Enregistre les raccourcis clavier
		registerShortcut('s', handleStartStopShortcut, true); // Ctrl+S
	});
</script>

<main class="min-h-screen bg-background text-foreground">
	<!-- Timer Pomodoro -->
	<PomodoroTimer autoAdvance={true} />
</main>
