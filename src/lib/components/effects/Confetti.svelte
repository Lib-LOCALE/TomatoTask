<script lang="ts">
	interface Props {
		duration?: number;
		particleCount?: number;
		colors?: string[];
	}
	let {
		duration = 3000,
		particleCount = 20,
		colors = ['var(--tomato)', 'var(--copper)', 'var(--cobalt)', 'var(--yolk)']
	}: Props = $props();
</script>

<div class="shower" aria-hidden="true">
	{#each Array.from({ length: particleCount }, (_, i) => i) as i}
		<span
			style:left="{(i * 37 + 11) % 100}%"
			style:--delay="{(i % 7) * 0.13}s"
			style:--duration="{duration / 1000}s"
			style:--pigment={colors[i % colors.length]}
		></span>
	{/each}
</div>

<style>
	.shower {
		position: fixed;
		inset: 0;
		z-index: 50;
		overflow: hidden;
		pointer-events: none;
	}
	span {
		position: absolute;
		top: -12px;
		width: 7px;
		height: 10px;
		background: var(--pigment);
		border-radius: 1px 6px 1px 6px;
		animation: fall var(--duration) var(--delay) ease-in both;
	}
	@keyframes fall {
		to {
			transform: translateY(100vh) rotate(280deg);
			opacity: 0;
		}
	}
	@media (prefers-reduced-motion: reduce) {
		.shower {
			display: none;
		}
	}
</style>
