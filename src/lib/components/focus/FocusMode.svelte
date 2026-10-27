<script lang="ts">
	// Mode focus immersif plein écran
	import PomodoroTimer from '$lib/components/timer/PomodoroTimer.svelte';
	import { _ } from 'svelte-i18n';

	let { onClose }: { onClose: () => void } = $props();
</script>

<main class="bg-background fixed inset-0 grid place-items-center">
	<button class="absolute top-4 right-4" onclick={onClose}>{$_('common.close')}</button>
	<PomodoroTimer autoAdvance={true} />
</main>
