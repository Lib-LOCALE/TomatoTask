<script lang="ts">
	// Sélecteur de langue avec persistance dans les paramètres
	import { locale } from 'svelte-i18n';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import {
		SUPPORTED_LANGUAGES,
		changeLanguage,
		getLanguageName,
		getLanguageFlag
	} from '$lib/services/i18n-service';
	import type { Language } from '$lib/types';

	// Props
	interface Props {
		variant?: 'dropdown' | 'buttons';
	}

	let { variant = 'dropdown' }: Props = $props();

	// Langue courante depuis svelte-i18n
	let currentLanguage = $derived($locale as Language);

	/**
	 * Change la langue et sauvegarde dans les paramètres
	 */
	async function handleLanguageChange(newLanguage: Language) {
		try {
			// Change la langue dans svelte-i18n
			changeLanguage(newLanguage);

			// Sauvegarde dans les paramètres
			await settingsStore.updateSetting('language', newLanguage);
		} catch (error) {
			console.error('Failed to change language:', error);
		}
	}

	/**
	 * Gère le changement via dropdown
	 */
	function handleSelectChange(event: Event) {
		const select = event.target as HTMLSelectElement;
		handleLanguageChange(select.value as Language);
	}
</script>

{#if variant === 'dropdown'}
	<!-- Dropdown standard -->
	<div class="w-full">
		<label for="language-select" class="mb-2 block text-sm font-medium">
			Language / Langue
		</label>

		<select
			id="language-select"
			onchange={handleSelectChange}
			value={currentLanguage}
			class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
		>
			{#each SUPPORTED_LANGUAGES as lang}
				<option value={lang}>
					{getLanguageFlag(lang)} {getLanguageName(lang)}
				</option>
			{/each}
		</select>
	</div>
{:else}
	<!-- Groupe de boutons -->
	<div class="w-full">
		<div class="mb-2 text-sm font-medium">Language / Langue</div>

		<div class="flex flex-wrap gap-2">
			{#each SUPPORTED_LANGUAGES as lang}
				<button
					type="button"
					onclick={() => handleLanguageChange(lang)}
					class="rounded-md border px-4 py-2 text-sm font-medium transition-colors flex items-center gap-2"
					class:border-primary={currentLanguage === lang}
					class:bg-primary={currentLanguage === lang}
					class:text-primary-foreground={currentLanguage === lang}
					class:hover:bg-muted={currentLanguage !== lang}
				>
					<span class="text-lg">{getLanguageFlag(lang)}</span>
					<span>{getLanguageName(lang)}</span>
				</button>
			{/each}
		</div>
	</div>
{/if}
