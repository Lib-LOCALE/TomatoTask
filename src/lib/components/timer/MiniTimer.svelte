<script lang="ts">
	import { timerStore } from '$lib/stores/timer.svelte';
	import { startSession } from '$lib/services/timer-service';
	import { _ } from 'svelte-i18n';
	import { Play, Pause, Square, Maximize2 } from '@lucide/svelte';
	import Button from '$lib/components/ui/button/button.svelte';

	let { onExpand }: { onExpand: () => void } = $props();

	function formatTime(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = seconds % 60;
		return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
	}

	function toggleTimer() {
		if (timerStore.isRunning) {
			timerStore.pause();
		} else {
			if (timerStore.remainingSeconds > 0) {
				timerStore.resume();
			} else {
				startSession('work');
			}
		}
	}

	function stopTimer() {
		timerStore.stop();
	}
</script>

<div
	class="bg-background flex h-full w-full flex-col items-center justify-center p-4"
	data-tauri-drag-region
>
	<!-- Time Display -->
	<div class="font-mono text-4xl font-bold tracking-wider" data-tauri-drag-region>
		{formatTime(timerStore.remainingSeconds)}
	</div>

	<!-- Status Text -->
	<div class="text-muted-foreground mb-2 text-xs tracking-widest uppercase" data-tauri-drag-region>
		{#if timerStore.sessionType === 'work'}
			{$_('timer.work')}
		{:else if timerStore.sessionType === 'short_break'}
			{$_('timer.shortBreak')}
		{:else}
			{$_('timer.longBreak')}
		{/if}
	</div>

	<!-- Controls -->
	<div class="flex items-center gap-2">
		<Button variant="ghost" size="icon" onclick={toggleTimer} class="h-8 w-8">
			{#if timerStore.isRunning}
				<Pause class="h-4 w-4" />
			{:else}
				<Play class="h-4 w-4" />
			{/if}
		</Button>

		<Button variant="ghost" size="icon" onclick={stopTimer} class="h-8 w-8">
			<Square class="h-4 w-4" />
		</Button>

		<div class="bg-border mx-1 h-4 w-px"></div>

		<Button variant="ghost" size="icon" onclick={onExpand} class="h-8 w-8" title="Expand">
			<Maximize2 class="h-4 w-4" />
		</Button>
	</div>
</div>
