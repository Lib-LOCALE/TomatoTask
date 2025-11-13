<script lang="ts">
	// Carte de statistique individuelle
	import { _ } from 'svelte-i18n';

	// Props
	interface Props {
		title: string;
		value: string | number;
		icon?: string;
		color?: 'primary' | 'success' | 'warning';
	}

	let { title, value, icon = '📊', color = 'primary' }: Props = $props();

	// Classes de couleur selon le type
	const colorClasses = {
		primary: 'border-blue-500 bg-blue-50 dark:bg-blue-950 text-blue-700 dark:text-blue-300',
		success: 'border-green-500 bg-green-50 dark:bg-green-950 text-green-700 dark:text-green-300',
		warning: 'border-amber-500 bg-amber-50 dark:bg-amber-950 text-amber-700 dark:text-amber-300'
	};
</script>

<div
	class="rounded-lg border-2 p-4 {colorClasses[color]} transition-all hover:shadow-md"
>
	<!-- Icône et titre -->
	<div class="mb-2 flex items-center gap-2">
		<span class="text-2xl">{icon}</span>
		<h3 class="text-sm font-medium opacity-80">{title}</h3>
	</div>

	<!-- Valeur -->
	<div class="text-3xl font-bold">
		{value}
	</div>
</div>
