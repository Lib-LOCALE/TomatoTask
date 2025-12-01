<script lang="ts">
	import { onMount } from 'svelte';

	interface Props {
		duration?: number;
		particleCount?: number;
		colors?: string[];
	}

	let {
		duration = 3000,
		particleCount = 50,
		colors = ['#F44336', '#4CAF50', '#2196F3', '#FFEB3B', '#FF9800', '#9C27B0']
	}: Props = $props();

	interface Particle {
		id: number;
		x: number;
		y: number;
		size: number;
		color: string;
		velocityX: number;
		velocityY: number;
		rotation: number;
		rotationSpeed: number;
	}

	let particles = $state<Particle[]>([]);
	let container = $state<HTMLDivElement>();

	function createParticle(id: number): Particle {
		return {
			id,
			x: 50, // Start from center
			y: 50,
			size: Math.random() * 8 + 4,
			color: colors[Math.floor(Math.random() * colors.length)],
			velocityX: (Math.random() - 0.5) * 15,
			velocityY: Math.random() * -20 - 10,
			rotation: Math.random() * 360,
			rotationSpeed: (Math.random() - 0.5) * 20
		};
	}

	onMount(() => {
		// Create initial particles
		particles = Array.from({ length: particleCount }, (_, i) => createParticle(i));

		// Animate particles
		const gravity = 0.8;
		const interval = setInterval(() => {
			particles = particles.map((p) => ({
				...p,
				x: p.x + p.velocityX,
				y: p.y + p.velocityY,
				velocityY: p.velocityY + gravity,
				rotation: p.rotation + p.rotationSpeed
			}));
		}, 1000 / 60);

		// Clean up after duration
		setTimeout(() => {
			clearInterval(interval);
			particles = [];
		}, duration);

		return () => clearInterval(interval);
	});
</script>

<div bind:this={container} class="confetti-container pointer-events-none fixed inset-0 z-50">
	{#each particles as particle (particle.id)}
		<div
			class="confetti-particle absolute"
			style:left="{particle.x}%"
			style:top="{particle.y}%"
			style:width="{particle.size}px"
			style:height="{particle.size}px"
			style:background-color={particle.color}
			style:transform="rotate({particle.rotation}deg)"
		></div>
	{/each}
</div>

<style>
	.confetti-container {
		overflow: hidden;
	}

	.confetti-particle {
		transition: all 0.016s linear;
		border-radius: 2px;
	}
</style>
