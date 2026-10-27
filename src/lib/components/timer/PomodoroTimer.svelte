<script lang="ts">
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { Volume2, Play, Pause } from '@lucide/svelte';
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
	import SoundControl from '$lib/components/sounds/SoundControl.svelte';
	import Tile from '$lib/components/majolica/Tile.svelte';
	import type { SessionType } from '$lib/types';
	let { autoAdvance = false }: { autoAdvance?: boolean } = $props();
	let selectedType = $state<SessionType>('work');
	let busy = $state(false);
	const task = $derived(
		timerStore.taskId
			? taskStore.tasks.find((t) => t.id === timerStore.taskId)
			: taskStore.selectedTask
	);
	const type = $derived(timerStore.remainingSeconds > 0 ? timerStore.sessionType : selectedType);
	const duration = $derived(
		type === 'work'
			? settingsStore.settings.workDuration
			: type === 'short_break'
				? settingsStore.settings.shortBreakDuration
				: settingsStore.settings.longBreakDuration
	);
	const time = $derived(
		timerStore.remainingSeconds > 0
			? timerStore.displayTime()
			: `${String(duration).padStart(2, '0')}:00`
	);
	async function toggle() {
		if (busy) return;
		if (timerStore.isRunning) {
			pauseTimer();
			return;
		}
		if (timerStore.remainingSeconds > 0) {
			resumeTimer();
			return;
		}
		busy = true;
		try {
			await startSession(selectedType, taskStore.selectedTask?.id);
		} finally {
			busy = false;
		}
	}
	async function skip() {
		const currentType = type;
		await stopTimer();
		selectedType = currentType === 'work' ? 'short_break' : 'work';
	}
	async function complete(sessionType: SessionType) {
		if (sessionType === 'work') await taskStore.load();
		await notifyComplete(sessionType);
		if (
			autoAdvance &&
			((sessionType === 'work' && settingsStore.settings.autoStartBreaks) ||
				(sessionType !== 'work' && settingsStore.settings.autoStartPomodoros))
		) {
			setTimeout(() => void startNextSession(taskStore.selectedTask?.id), 3000);
		} else selectedType = sessionType === 'work' ? 'short_break' : 'work';
	}
	onMount(() => onSessionCompleteCallback(complete));
</script>

<div class="timer" class:short={type === 'short_break'} class:long={type === 'long_break'}>
	<div class="timer-top">
		<div class="tabs" role="group" aria-label={$_('timer.title')}>
			{#each ['work', 'short_break', 'long_break'] as item}
				<button
					type="button"
					class:active={type === item}
					disabled={timerStore.remainingSeconds > 0}
					onclick={() => (selectedType = item as SessionType)}
					>{$_(
						item === 'work'
							? 'timer.work'
							: item === 'short_break'
								? 'timer.shortBreak'
								: 'timer.longBreak'
					)}</button
				>
			{/each}
		</div>
		<details class="sounds">
			<summary aria-label={$_('majolica.sounds')} title={$_('majolica.sounds')}
				><Volume2 size={19} /></summary
			>
			<div><SoundControl /></div>
		</details>
	</div>
	<div class="spotlight">
		<h1>{task?.title || $_('majolica.noTask')}</h1>
		{#if task && task.estimatedPomodoros > 0}<p>
				{$_('majolica.pomodoro', {
					values: {
						current: Math.min(task.completedPomodoros + 1, task.estimatedPomodoros),
						total: task.estimatedPomodoros
					}
				})}
			</p>
			<div class="mini-tiles" aria-hidden="true">
				{#each Array(task.estimatedPomodoros) as _, i}<span class:done={i < task.completedPomodoros}
					></span>{/each}
			</div>{/if}
	</div>
	<div class="time-row">
		<div class="digits" aria-label={time}>
			{#each time.split('') as digit}<span class:colon={digit === ':'}>{digit}</span>{/each}
		</div>
		<div class="progress-tile">
			<Tile state="painting" position={4} progress={timerStore.progress() / 100} />
		</div>
	</div>
	<div class="actions">
		<button class="primary-action" type="button" onclick={toggle} disabled={busy}
			>{#if timerStore.isRunning}<Pause size={20} />{$_('timer.pause')}{:else}<Play
					size={20}
				/>{timerStore.remainingSeconds > 0 ? $_('timer.resume') : $_('timer.start')}{/if}</button
		>
		<div class="secondary-actions">
			<button type="button" onclick={skip}>{$_('majolica.skip')}</button><button
				type="button"
				onclick={() => stopTimer()}>{$_('majolica.reset')}</button
			>
		</div>
	</div>
</div>

<style>
	.timer {
		--session: var(--session-work);
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		min-height: 650px;
		padding: 24px 34px 48px;
	}
	.timer.short {
		--session: var(--session-short-break);
	}
	.timer.long {
		--session: var(--session-long-break);
	}
	.timer-top {
		display: flex;
		justify-content: space-between;
		align-items: start;
		gap: 12px;
	}
	.tabs {
		display: flex;
		flex-wrap: wrap;
		gap: 4px;
	}
	.tabs button {
		padding: 8px 10px;
		border-radius: 3px;
		font-weight: 700;
		color: var(--muted-foreground);
	}
	.tabs button.active {
		background: var(--primary);
		color: var(--primary-foreground);
	}
	.tabs button:disabled {
		opacity: 0.75;
	}
	.sounds {
		position: relative;
	}
	summary {
		list-style: none;
		cursor: pointer;
		padding: 8px;
	}
	summary::-webkit-details-marker {
		display: none;
	}
	.sounds div {
		position: absolute;
		right: 0;
		top: 40px;
		width: 300px;
		z-index: 4;
		background: var(--popover);
		color: var(--popover-foreground);
		border: 1px solid var(--border);
		box-shadow: var(--enamel-shadow);
		padding: 12px;
	}
	.spotlight {
		text-align: center;
		margin: 30px 0 10px;
	}
	h1 {
		font-size: clamp(1.5rem, 2.5vw, 2.3rem);
	}
	.spotlight p {
		color: var(--muted-foreground);
		margin-top: 10px;
	}
	.mini-tiles {
		display: flex;
		gap: 3px;
		justify-content: center;
		margin-top: 8px;
	}
	.mini-tiles span {
		width: 12px;
		height: 12px;
		border: 1px solid var(--grout);
		background: var(--glaze);
	}
	.mini-tiles .done {
		background: var(--session);
	}
	.time-row {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 12px;
	}
	.digits {
		display: flex;
		font-family: var(--font-heading);
		font-size: clamp(5rem, 10vw, 12rem);
		font-weight: 700;
		line-height: 1;
		font-variant-numeric: tabular-nums;
		letter-spacing: -0.04em;
	}
	.digits span {
		width: 0.62em;
		text-align: center;
	}
	.digits .colon {
		width: 0.28em;
		color: var(--session);
	}
	.progress-tile {
		width: clamp(45px, 5vw, 72px);
		flex: none;
		border: 1px solid var(--grout);
	}
	.actions {
		width: min(100%, 420px);
		margin: 30px auto 0;
	}
	.primary-action {
		width: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 10px;
		background: var(--session);
		color: #fff;
		padding: 15px;
		font-weight: 700;
		border-radius: 4px;
		box-shadow: var(--enamel-shadow);
	}
	:global(.dark) .primary-action {
		color: #172e59;
	}
	.short .primary-action {
		color: #172e59;
	}
	.long .primary-action {
		color: #102d20;
	}
	.secondary-actions {
		display: flex;
		justify-content: center;
		gap: 24px;
		margin-top: 16px;
	}
	.secondary-actions button {
		text-decoration: underline;
		text-underline-offset: 4px;
	}
	@media (max-width: 760px) {
		.timer {
			min-height: 570px;
			padding: 20px;
		}
		.digits {
			font-size: clamp(3.8rem, 14vw, 8rem);
		}
	}
</style>
