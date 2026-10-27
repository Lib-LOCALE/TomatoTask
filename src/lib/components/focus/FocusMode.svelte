<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { Play, Pause, SkipForward, X } from '@lucide/svelte';
	import Tile from '$lib/components/majolica/Tile.svelte';
	import { timerStore } from '$lib/stores/timer.svelte';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import {
		onSessionCompleteCallback,
		startNextSession,
		startSession,
		pauseTimer,
		resumeTimer,
		stopTimer
	} from '$lib/services/timer-service';
	import { notifyComplete } from '$lib/services/notification-service';
	import type { SessionType } from '$lib/types';

	let { onClose }: { onClose: () => void } = $props();

	let busy = $state(false);
	let selectedType = $state<SessionType>('work');
	let controlsVisible = $state(true);
	let hideTimeout: number | null = null;
	let showGlazedToast = $state(false);
	let toastTimeout: number | null = null;
	let isGlazed = $state(false);
	let animateTile = $state(false);

	let mainButtonEl = $state<HTMLButtonElement | null>(null);

	// Tâche sélectionnée ou liée au minuteur
	const task = $derived(
		timerStore.taskId
			? taskStore.tasks.find((t) => t.id === timerStore.taskId)
			: taskStore.selectedTask
	);

	// Type de session courant (issu du minuteur en cours ou de la sélection locale)
	const currentType = $derived(
		timerStore.remainingSeconds > 0 ? timerStore.sessionType : selectedType
	);

	const isBreak = $derived(currentType === 'short_break' || currentType === 'long_break');

	// Durée configurée
	const durationMinutes = $derived(
		currentType === 'work'
			? settingsStore.settings.workDuration
			: currentType === 'short_break'
				? settingsStore.settings.shortBreakDuration
				: settingsStore.settings.longBreakDuration
	);

	// Temps formaté
	const time = $derived(
		timerStore.remainingSeconds > 0
			? timerStore.displayTime()
			: `${String(durationMinutes).padStart(2, '0')}:00`
	);

	// Progression pour le carreau
	const progress = $derived(timerStore.remainingSeconds > 0 ? timerStore.progress() / 100 : 0);

	// État du carreau
	const tileState = $derived(isGlazed ? 'glazed' : currentType === 'work' ? 'painting' : 'planned');

	/**
	 * Réinitialise le délai d'inactivité pour afficher les contrôles pendant 3 s
	 */
	function resetActivityTimer(): void {
		controlsVisible = true;
		if (hideTimeout !== null) {
			window.clearTimeout(hideTimeout);
		}
		hideTimeout = window.setTimeout(() => {
			controlsVisible = false;
		}, 3000);
	}

	/**
	 * Bascule le minuteur (Démarrer / Pause / Reprendre)
	 */
	async function toggleTimer(): Promise<void> {
		if (busy) return;
		resetActivityTimer();

		if (timerStore.isRunning) {
			pauseTimer();
			return;
		}

		if (timerStore.remainingSeconds > 0) {
			resumeTimer();
			return;
		}

		busy = true;
		isGlazed = false;
		animateTile = false;
		try {
			await startSession(selectedType, task?.id);
		} finally {
			busy = false;
		}
	}

	/**
	 * Passe à la session suivante
	 */
	async function handleSkip(): Promise<void> {
		if (busy) return;
		resetActivityTimer();
		const prev = currentType;
		await stopTimer();
		isGlazed = false;
		animateTile = false;
		selectedType = prev === 'work' ? 'short_break' : 'work';
	}

	/**
	 * Quitte le mode focus et désactive le plein écran
	 */
	function handleExit(): void {
		exitFullscreen();
		onClose();
	}

	/**
	 * Entrée en plein écran via l'API Fullscreen du navigateur
	 */
	function enterFullscreen(): void {
		try {
			if (typeof document !== 'undefined' && document.documentElement?.requestFullscreen) {
				document.documentElement.requestFullscreen().catch(() => {
					// Tolère les restrictions navigateur sans bloquer l'interface
				});
			}
		} catch {
			// Ignore les environnements sans support de fullscreen
		}
	}

	/**
	 * Sortie du plein écran
	 */
	function exitFullscreen(): void {
		try {
			if (
				typeof document !== 'undefined' &&
				document.fullscreenElement &&
				document.exitFullscreen
			) {
				document.exitFullscreen().catch(() => {
					// Ignore
				});
			}
		} catch {
			// Ignore
		}
	}

	/**
	 * Gestion des raccourcis clavier
	 */
	function handleKeyDown(e: KeyboardEvent): void {
		resetActivityTimer();

		if (e.code === 'Space') {
			e.preventDefault();
			toggleTimer();
		} else if (e.key === 'Escape') {
			e.preventDefault();
			handleExit();
		}
	}

	/**
	 * Traite la complétion d'une session
	 */
	async function handleSessionComplete(sessionType: SessionType): Promise<void> {
		if (sessionType === 'work') {
			isGlazed = true;
			animateTile = true;
			showGlazedToast = true;

			if (toastTimeout !== null) {
				window.clearTimeout(toastTimeout);
			}
			toastTimeout = window.setTimeout(() => {
				showGlazedToast = false;
			}, 3500);

			await taskStore.load();
		}

		await notifyComplete(sessionType);

		const autoStart =
			(sessionType === 'work' && settingsStore.settings.autoStartBreaks) ||
			(sessionType !== 'work' && settingsStore.settings.autoStartPomodoros);

		if (autoStart) {
			window.setTimeout(() => {
				isGlazed = false;
				animateTile = false;
				void startNextSession(task?.id);
			}, 3000);
		} else {
			selectedType = sessionType === 'work' ? 'short_break' : 'work';
		}
	}

	onMount(() => {
		enterFullscreen();
		resetActivityTimer();
		mainButtonEl?.focus();
		onSessionCompleteCallback(handleSessionComplete);
	});

	onDestroy(() => {
		exitFullscreen();
		if (hideTimeout !== null) window.clearTimeout(hideTimeout);
		if (toastTimeout !== null) window.clearTimeout(toastTimeout);
	});
</script>

<svelte:window onkeydown={handleKeyDown} />

<div
	class="focus-overlay"
	class:hide-cursor={!controlsVisible}
	role="dialog"
	aria-modal="true"
	aria-label={$_('focus.title')}
	onpointermove={resetActivityTimer}
	tabindex="-1"
>
	<!-- En-tête : session et tâche -->
	<header class="focus-header">
		<h1 class="task-title">
			{task?.title || $_('focus.noTask')}
		</h1>
		<!-- Message bref « Carreau émaillé » -->
		{#if showGlazedToast}
			<div class="glazed-toast" role="status" aria-live="polite">
				{$_('focus.tileGlazed')}
			</div>
		{/if}
	</header>

	<!-- Centre : le carreau qui se peint, et le minuteur à échelle d'affiche -->
	<div class="tile-stage">
		<div class="tile-frame">
			<Tile state={tileState} position={4} {progress} animate={animateTile} />
		</div>
		<div class="timer-section">
			<div class="time-display" class:time-break={isBreak} aria-label={time}>
				{#each time.split('') as char}
					<span class:colon={char === ':'}>{char}</span>
				{/each}
			</div>
			<span class="session-badge" class:badge-break={isBreak}>
				{#if currentType === 'work'}
					{$_('focus.workSession')}
				{:else if currentType === 'short_break'}
					{$_('focus.shortBreak')}
				{:else}
					{$_('focus.longBreak')}
				{/if}
			</span>
		</div>
	</div>

	<!-- Barre de contrôles discrets (disparaît après 3s d'inactivité) -->
	<footer class="controls-bar" class:fade-out={!controlsVisible}>
		<div class="buttons-group">
			<button
				bind:this={mainButtonEl}
				type="button"
				class="action-btn primary-action"
				class:break-theme={isBreak}
				onclick={toggleTimer}
				disabled={busy}
				aria-label={timerStore.isRunning
					? $_('focus.pause')
					: timerStore.remainingSeconds > 0
						? $_('focus.resume')
						: $_('focus.start')}
			>
				{#if timerStore.isRunning}
					<Pause size={20} aria-hidden="true" />
					<span>{$_('focus.pause')}</span>
				{:else}
					<Play size={20} aria-hidden="true" />
					<span>{timerStore.remainingSeconds > 0 ? $_('focus.resume') : $_('focus.start')}</span>
				{/if}
			</button>

			<button
				type="button"
				class="action-btn secondary-action"
				onclick={handleSkip}
				aria-label={$_('focus.skip')}
			>
				<SkipForward size={18} aria-hidden="true" />
				<span>{$_('focus.skip')}</span>
			</button>

			<button
				type="button"
				class="action-btn secondary-action"
				onclick={handleExit}
				aria-label={$_('focus.exitAria')}
			>
				<X size={18} aria-hidden="true" />
				<span>{$_('focus.exit')}</span>
			</button>
		</div>

		<p class="shortcuts-note" aria-hidden="true">
			{$_('focus.shortcutsHint')}
		</p>
	</footer>
</div>

<style>
	/* Fond nuit cobalt profonde quel que soit le thème */
	.focus-overlay {
		position: fixed;
		inset: 0;
		z-index: 1000;
		background: #0f1d38;
		color: #edf3ff;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-between;
		padding: clamp(16px, 3vh, 32px) clamp(16px, 4vw, 40px);
		overflow: hidden;
		user-select: none;
	}

	.focus-overlay.hide-cursor {
		cursor: none;
	}

	/* Message bref « Carreau émaillé » */
	.glazed-toast {
		margin-top: 6px;
		background: rgba(16, 28, 51, 0.95);
		border: 1px solid var(--yolk, #e3b435);
		color: var(--yolk, #e3b435);
		padding: 6px 20px;
		border-radius: var(--radius-md, 4px);
		font-family: var(--font-heading, 'Bodoni Moda Variable', serif);
		font-size: clamp(1rem, 1.8vw, 1.25rem);
		font-weight: 700;
		letter-spacing: 0.03em;
		box-shadow:
			0 8px 24px rgba(0, 0, 0, 0.5),
			inset 0 1px 1px rgba(255, 255, 255, 0.2);
		animation: toastFadeIn 250ms ease-out;
	}

	@keyframes toastFadeIn {
		from {
			opacity: 0;
			transform: translateY(-6px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* En-tête */
	.focus-header {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 6px;
		text-align: center;
		max-width: min(90vw, 720px);
	}

	.session-badge {
		font-size: clamp(1rem, 1.6vw, 1.25rem);
		font-weight: 600;
		color: rgba(237, 243, 255, 0.78);
	}

	.session-badge.badge-break {
		color: var(--yolk, #e3b435);
	}

	.task-title {
		font-family: var(--font-heading, 'Bodoni Moda Variable', serif);
		font-size: clamp(1.3rem, 2.5vw, 2.2rem);
		font-weight: 600;
		line-height: 1.2;
		color: #ffffff;
		margin: 0;
		text-overflow: ellipsis;
		overflow: hidden;
		white-space: nowrap;
		max-width: 100%;
	}

	/* Grand carreau de majolique */
	.tile-stage {
		display: flex;
		align-items: center;
		justify-content: center;
		flex: 1;
		flex-wrap: wrap;
		gap: clamp(24px, 6vw, 88px);
		min-height: 0;
		padding: 8px 0;
	}

	.tile-frame {
		width: min(52vh, 38vw);
		height: min(52vh, 38vw);
		max-width: 520px;
		max-height: 520px;
		min-width: 220px;
		min-height: 220px;
		aspect-ratio: 1;
		border-radius: var(--radius-xl, 6px);
		box-shadow:
			0 0 0 1px var(--grout),
			inset 0 1px rgba(255, 255, 255, 0.6);
		overflow: hidden;
		background: var(--glaze, #f6f8f7);
	}

	/* Chiffres du minuteur en Bodoni géant */
	.timer-section {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 12px;
	}

	.time-display {
		display: flex;
		align-items: center;
		justify-content: center;
		font-family: var(--font-heading, 'Bodoni Moda Variable', serif);
		font-size: clamp(4.5rem, 11vw, 10rem);
		font-weight: 700;
		line-height: 1;
		font-variant-numeric: tabular-nums;
		letter-spacing: -0.04em;
		color: #edf3ff;
	}

	.time-display.time-break {
		color: var(--yolk, #e3b435);
	}

	.time-display span {
		width: 0.62em;
		text-align: center;
	}

	.time-display .colon {
		width: 0.28em;
		color: var(--session-work, #b93424);
	}

	.time-display.time-break .colon {
		color: var(--yolk, #e3b435);
	}

	/* Contrôles discrets */
	.controls-bar {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		transition: opacity 280ms ease-out;
		min-height: 64px;
	}

	.controls-bar.fade-out {
		opacity: 0;
		pointer-events: none;
	}

	.buttons-group {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		border-radius: var(--radius-md, 4px);
		font-family: var(--font-body, 'Source Sans 3 Variable', sans-serif);
		font-weight: 700;
		cursor: pointer;
		outline: none;
		border: none;
	}

	.primary-action {
		background: var(--session-work, #b93424);
		color: #ffffff;
		font-size: 1rem;
		padding: 10px 24px;
		box-shadow: inset 0 1px rgba(255, 255, 255, 0.2);
		transition:
			filter 150ms ease,
			transform 150ms ease;
	}

	.primary-action:hover {
		filter: brightness(1.12);
	}

	.primary-action:active {
		transform: scale(0.97);
	}

	.primary-action.break-theme {
		background: var(--yolk, #e3b435);
		color: #101c33;
	}

	.secondary-action {
		background: rgba(255, 255, 255, 0.12);
		color: #edf3ff;
		font-size: 0.95rem;
		padding: 10px 18px;
		border: 1px solid rgba(255, 255, 255, 0.16);
		transition:
			background 150ms ease,
			border-color 150ms ease;
	}

	.secondary-action:hover {
		background: rgba(255, 255, 255, 0.2);
		border-color: rgba(255, 255, 255, 0.28);
	}

	.shortcuts-note {
		font-size: 0.8rem;
		color: rgba(237, 243, 255, 0.55);
		font-family: var(--font-body, 'Source Sans 3 Variable', sans-serif);
		letter-spacing: 0.02em;
		margin: 0;
	}

	/* Accessibilité et mouvement réduit */
	:focus-visible {
		outline: 3px solid var(--yolk, #e3b435);
		outline-offset: 3px;
	}

	@media (prefers-reduced-motion: reduce) {
		.controls-bar,
		.primary-action,
		.secondary-action,
		.glazed-toast {
			animation: none !important;
			transition: none !important;
		}
	}
</style>
