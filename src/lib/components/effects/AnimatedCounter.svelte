<script lang="ts">
	import { onMount } from 'svelte';

	interface Props {
		value: number;
		duration?: number;
		format?: (n: number) => string;
		class?: string;
	}

	let {
		value,
		duration = 1000,
		format = (n) => Math.round(n).toString(),
		class: className = ''
	}: Props = $props();

	let displayValue = $state(0);
	let previousValue = 0;

	function animateCounter(from: number, to: number) {
		const startTime = performance.now();
		const difference = to - from;

		function update(currentTime: number) {
			const elapsed = currentTime - startTime;
			const progress = Math.min(elapsed / duration, 1);

			// Easing function (ease-out)
			const eased = 1 - Math.pow(1 - progress, 3);
			displayValue = from + difference * eased;

			if (progress < 1) {
				requestAnimationFrame(update);
			} else {
				displayValue = to;
			}
		}

		requestAnimationFrame(update);
	}

	$effect(() => {
		if (value !== previousValue) {
			animateCounter(displayValue, value);
			previousValue = value;
		}
	});

	onMount(() => {
		animateCounter(0, value);
	});
</script>

<span class="animated-counter {className}">
	{format(displayValue)}
</span>

<style>
	.animated-counter {
		display: inline-block;
		min-width: 2ch;
		font-variant-numeric: tabular-nums;
	}
</style>
