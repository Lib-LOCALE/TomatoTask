<script lang="ts">
	// Toggle pour basculer entre thème clair et sombre
	import { settingsStore } from '$lib/stores/settings.svelte';
	import type { Theme } from '$lib/types';

	// Props
	interface Props {
		variant?: 'button' | 'toggle';
	}

	let { variant = 'toggle' }: Props = $props();

	// Thème courant depuis les paramètres
	const currentTheme = $derived(settingsStore.settings.theme);

	/**
	 * Bascule entre les thèmes clair et sombre
	 */
	async function toggleTheme() {
		const newTheme: Theme = currentTheme === 'light' ? 'dark' : 'light';

		try {
			// Sauvegarde dans les paramètres
			await settingsStore.updateSetting('theme', newTheme);

			// Applique le thème au document
			applyTheme(newTheme);
		} catch (error) {
			console.error('Failed to toggle theme:', error);
		}
	}

	/**
	 * Applique le thème au document HTML
	 */
	function applyTheme(theme: Theme) {
		if (theme === 'dark') {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	}
</script>

{#if variant === 'button'}
	<!-- Bouton avec icône -->
	<button
		type="button"
		onclick={toggleTheme}
		class="rounded-md p-2 hover:bg-muted transition-colors"
		aria-label={currentTheme === 'light' ? 'Switch to dark mode' : 'Switch to light mode'}
		title={currentTheme === 'light' ? 'Switch to dark mode' : 'Switch to light mode'}
	>
		{#if currentTheme === 'light'}
			<!-- Icône lune (mode sombre) -->
			<svg
				class="h-5 w-5"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"
				/>
			</svg>
		{:else}
			<!-- Icône soleil (mode clair) -->
			<svg
				class="h-5 w-5"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				viewBox="0 0 24 24"
			>
				<circle cx="12" cy="12" r="4" />
				<path d="M12 2v2m0 16v2M4.93 4.93l1.41 1.41m11.32 11.32l1.41 1.41M2 12h2m16 0h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" />
			</svg>
		{/if}
	</button>
{:else}
	<!-- Toggle switch style -->
	<button
		type="button"
		onclick={toggleTheme}
		class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors"
		class:bg-primary={currentTheme === 'dark'}
		class:bg-muted={currentTheme === 'light'}
		aria-label={currentTheme === 'light' ? 'Switch to dark mode' : 'Switch to light mode'}
		role="switch"
		aria-checked={currentTheme === 'dark'}
	>
		<span
			class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform"
			class:translate-x-6={currentTheme === 'dark'}
			class:translate-x-1={currentTheme === 'light'}
		>
			{#if currentTheme === 'dark'}
				<svg class="h-4 w-4 text-primary" fill="currentColor" viewBox="0 0 20 20">
					<path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
				</svg>
			{:else}
				<svg class="h-4 w-4 text-yellow-500" fill="currentColor" viewBox="0 0 20 20">
					<path
						fill-rule="evenodd"
						d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z"
						clip-rule="evenodd"
					/>
				</svg>
			{/if}
		</span>
	</button>
{/if}
