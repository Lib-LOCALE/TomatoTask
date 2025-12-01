<script lang="ts">
	import { onMount } from 'svelte';

	interface Props {
		count?: number;
		colors?: string[];
	}

	let { count = 20, colors = ['#F4433620', '#4CAF5020', '#2196F320'] }: Props = $props();

	interface Particle {
		id: number;
		x: number;
		y: number;
		size: number;
		duration: number;
		delay: number;
		color: string;
	}

	let particles = $state<Particle[]>([]);

	onMount(() => {
		particles = Array.from({ length: count }, (_, i) => ({
			id: i,
			x: Math.random() * 100,
			y: Math.random() * 100,
			size: Math.random() * 8 + 4,
			duration: Math.random() * 20 + 15,
			delay: Math.random() * 5,
			color: colors[Math.floor(Math.random() * colors.length)]
		}));
	});
</script>

<div class="particles-container pointer-events-none fixed inset-0 overflow-hidden opacity-40">
	{#each particles as particle (particle.id)}
		<div
			class="particle absolute rounded-full"
			style:left="{particle.x}%"
			style:top="{particle.y}%"
			style:width="{particle.size}px"
			style:height="{particle.size}px"
			style:background-color={particle.color}
			style:animation-duration="{particle.duration}s"
			style:animation-delay="{particle.delay}s"
		></div>
	{/each}
</div>

<style>
	.particle {
		animation: float-particle infinite ease-in-out;
		filter: blur(1px);
	}

	@keyframes float-particle {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
			opacity: 0.3;
		}
		25% {
			transform: translate(30px, -30px) scale(1.2);
			opacity: 0.6;
		}
		50% {
			transform: translate(-20px, -60px) scale(0.8);
			opacity: 0.4;
		}
		75% {
			transform: translate(40px, -40px) scale(1.1);
			opacity: 0.5;
		}
	}

	/* Respect prefers-reduced-motion */
	@media (prefers-reduced-motion: reduce) {
		.particle {
			animation: none;
			opacity: 0.2;
		}
	}
</style>
