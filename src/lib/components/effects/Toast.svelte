<script lang="ts">
	import { onMount } from 'svelte';
	import AnimatedIcon from '$lib/components/ui/AnimatedIcon.svelte';

	interface Props {
		message: string;
		type?: 'success' | 'error' | 'info' | 'celebration';
		duration?: number;
		onClose?: () => void;
	}

	let { message, type = 'info', duration = 3000, onClose }: Props = $props();

	let visible = $state(false);

	const typeStyles = {
		success: 'bg-green-50 dark:bg-green-950 border-green-500 text-green-900 dark:text-green-100',
		error: 'bg-red-50 dark:bg-red-950 border-red-500 text-red-900 dark:text-red-100',
		info: 'bg-blue-50 dark:bg-blue-950 border-blue-500 text-blue-900 dark:text-blue-100',
		celebration:
			'bg-gradient-to-r from-purple-50 to-pink-50 dark:from-purple-950 dark:to-pink-950 border-purple-500 text-purple-900 dark:text-purple-100'
	};

	const icons = {
		success: 'check',
		error: '✕',
		info: 'ℹ',
		celebration: 'tomato-celebration'
	};

	onMount(() => {
		// Trigger enter animation
		requestAnimationFrame(() => {
			visible = true;
		});

		// Auto-hide after duration
		const timer = setTimeout(() => {
			visible = false;
			setTimeout(() => {
				onClose?.();
			}, 300); // Wait for exit animation
		}, duration);

		return () => clearTimeout(timer);
	});
</script>

<div
	class="toast fixed right-4 top-4 z-50 min-w-80 max-w-md transform rounded-lg border-2 p-4 shadow-2xl transition-all duration-300 {typeStyles[type]}"
	class:translate-x-0={visible}
	class:translate-x-[120%]={!visible}
	class:opacity-100={visible}
	class:opacity-0={!visible}
>
	<div class="flex items-start gap-3">
		<!-- Icon -->
		<div class="shrink-0">
			{#if type === 'celebration'}
				<AnimatedIcon name="tomato-celebration" size={32} />
			{:else if type === 'success'}
				<AnimatedIcon name="check" size={32} />
			{:else}
				<div class="flex h-8 w-8 items-center justify-center text-2xl">
					{icons[type]}
				</div>
			{/if}
		</div>

		<!-- Message -->
		<div class="flex-1">
			<p class="font-medium">{message}</p>
		</div>

		<!-- Close button -->
		<button
			onclick={() => {
				visible = false;
				setTimeout(onClose, 300);
			}}
			class="shrink-0 rounded p-1 transition-colors hover:bg-black/10 dark:hover:bg-white/10"
		>
			<svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
			</svg>
		</button>
	</div>
</div>

<style>
	.toast {
		animation: slide-bounce 0.5s cubic-bezier(0.68, -0.55, 0.265, 1.55);
	}

	@keyframes slide-bounce {
		0% {
			transform: translateX(120%);
		}
		70% {
			transform: translateX(-10px);
		}
		100% {
			transform: translateX(0);
		}
	}
</style>
