<script lang="ts">
	// Composant de contrôle du timer (boutons Start/Pause/Resume/Stop)
	import { timerStore } from '$lib/stores/timer.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { startSession, pauseTimer, resumeTimer, stopTimer } from '$lib/services/timer-service';
	import { _ } from 'svelte-i18n';
	import Button from '$lib/components/ui/button/button.svelte';
	import type { SessionType } from '$lib/types';

	// Props
	interface Props {
		selectedTaskId?: number;
		onSessionStart?: (type: SessionType) => void;
		onSessionStop?: () => void;
	}

	let { selectedTaskId, onSessionStart, onSessionStop }: Props = $props();

	// État local
	let isStarting = $state(false);

	/**
	 * Démarre une nouvelle session de travail
	 */
	async function handleStart() {
		if (isStarting) return;

		isStarting = true;
		try {
			await startSession('work', selectedTaskId);
			onSessionStart?.('work');
		} catch (error) {
			console.error('Failed to start session:', error);
		} finally {
			isStarting = false;
		}
	}

	/**
	 * Met le timer en pause
	 */
	function handlePause() {
		pauseTimer();
	}

	/**
	 * Reprend le timer
	 */
	function handleResume() {
		resumeTimer();
	}

	/**
	 * Arrête complètement le timer
	 */
	async function handleStop() {
		await stopTimer(true);
		onSessionStop?.();
	}
</script>

<div class="flex items-center justify-center gap-3">
	{#if !timerStore.isRunning && timerStore.remainingSeconds === 0}
		<!-- État initial: bouton Start -->
		<Button
			size="lg"
			onclick={handleStart}
			disabled={isStarting}
			class="px-8 py-6 text-lg"
		>
			{isStarting ? $_('common.loading') : $_('timer.start')}
		</Button>
	{:else if timerStore.isRunning}
		<!-- Timer en cours: boutons Pause et Stop -->
		<Button
			size="lg"
			variant="secondary"
			onclick={handlePause}
			class="px-6"
		>
			{$_('timer.pause')}
		</Button>

		<Button
			size="lg"
			variant="destructive"
			onclick={handleStop}
			class="px-6"
		>
			{$_('timer.stop')}
		</Button>
	{:else}
		<!-- Timer en pause: boutons Resume et Stop -->
		<Button
			size="lg"
			onclick={handleResume}
			class="px-6"
		>
			{$_('timer.resume')}
		</Button>

		<Button
			size="lg"
			variant="destructive"
			onclick={handleStop}
			class="px-6"
		>
			{$_('timer.stop')}
		</Button>
	{/if}
</div>

<!-- Affichage de la durée configurée quand le timer n'est pas actif -->
{#if timerStore.remainingSeconds === 0}
	<div class="text-center text-sm text-muted-foreground mt-4">
		{$_('settings.workDuration')}: {settingsStore.settings.workDuration} {$_('common.minutes')}
	</div>
{/if}
