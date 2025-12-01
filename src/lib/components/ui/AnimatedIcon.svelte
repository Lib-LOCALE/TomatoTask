<script lang="ts">
	interface Props {
		name: 'tomato' | 'timer' | 'check' | 'focus' | 'stats';
		size?: number;
		class?: string;
	}

	let { name, size = 64, class: className = '' }: Props = $props();

	const iconPaths: Record<typeof name, string> = {
		tomato: '/icons/animated-tomato.svg',
		timer: '/icons/animated-timer.svg',
		check: '/icons/animated-check.svg',
		focus: '/icons/animated-focus.svg',
		stats: '/icons/animated-stats.svg'
	};

	const iconPath = $derived(iconPaths[name]);
</script>

<div class="animated-icon {className}" style="width: {size}px; height: {size}px;">
	<object type="image/svg+xml" data={iconPath} aria-label={name} style="pointer-events: none;">
		<!-- Fallback content -->
		<img src={iconPath} alt={name} />
	</object>
</div>

<style>
	.animated-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.animated-icon object,
	.animated-icon img {
		width: 100%;
		height: 100%;
	}
</style>
