<script lang="ts">
	import { timerStore } from '$lib/stores/timer.svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import { startSession, pauseTimer, resumeTimer } from '$lib/services/timer-service';
	import { _ } from 'svelte-i18n';
	import { Maximize2 } from '@lucide/svelte';
	import Tile from '$lib/components/majolica/Tile.svelte';

	let { onExpand }: { onExpand: () => void } = $props();

	function toggleTimer() {
		if (timerStore.isRunning) pauseTimer();
		else if (timerStore.remainingSeconds > 0) resumeTimer();
		else void startSession('work', taskStore.selectedTask?.id);
	}
</script>

<div class="mini" data-tauri-drag-region>
	<div class="tile"><Tile state="painting" progress={timerStore.progress() / 100} /></div>
	<div class="content">
		<div class="time" data-tauri-drag-region>
			{timerStore.totalSeconds
				? timerStore.displayTime()
				: `${String(settingsStore.settings.workDuration).padStart(2, '0')}:00`}
		</div>
		<div class="controls">
			<button class="start" type="button" onclick={toggleTimer}>
				{timerStore.isRunning ? $_('timer.pause') : $_('timer.start')}
			</button>
			<button
				class="expand"
				type="button"
				onclick={onExpand}
				aria-label={$_('chrome.mini.expand')}
				title={$_('chrome.mini.expand')}
			>
				<Maximize2 size={18} />
			</button>
		</div>
	</div>
</div>

<style>
	.mini {
		display: flex;
		align-items: center;
		gap: 12px;
		height: 100vh;
		padding: 12px;
		background: var(--background);
		color: var(--foreground);
	}
	.tile {
		flex: 0 0 76px;
		border: 1px solid var(--grout);
	}
	.content {
		flex: 1;
		min-width: 0;
	}
	.time {
		font-family: var(--font-heading);
		font-variant-numeric: tabular-nums;
		font-size: clamp(2.4rem, 15vw, 3.5rem);
		line-height: 1;
	}
	.controls {
		display: flex;
		gap: 8px;
		margin-top: 8px;
	}
	button {
		border: 1px solid var(--border);
		border-radius: 4px;
		padding: 4px 12px;
	}
	.start {
		background: var(--primary);
		color: var(--primary-foreground);
		flex: 1;
	}
	.expand {
		display: grid;
		place-items: center;
		background: var(--card);
	}
</style>
