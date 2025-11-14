<script lang="ts">
	// Sélecteur de langue avec persistance dans les paramètres
	import { locale } from 'svelte-i18n';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import {
		SUPPORTED_LANGUAGES,
		changeLanguage,
		getLanguageName,
		getLanguageCode,
		getLanguageFlag,
		getLanguageFlagPath
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
					{getLanguageFlag(lang)} {getLanguageName(lang)} ({getLanguageCode(lang)})
				</option>
			{/each}
		</select>
	</div>
{:else}
	<!-- Groupe de boutons avec drapeaux -->
	<div class="w-full">
		<div class="mb-3 text-sm font-medium">Language / Langue</div>

		<div class="flex flex-wrap gap-3">
			{#each SUPPORTED_LANGUAGES as lang}
				{@const isSelected = currentLanguage === lang}
				<button
					type="button"
					onclick={() => handleLanguageChange(lang)}
					class:border-primary={isSelected}
					class:bg-accent={isSelected}
					class:shadow-md={isSelected}
					class:border-border={!isSelected}
					class:hover:bg-muted={!isSelected}
					class="group relative rounded-lg border-2 px-4 py-3 text-sm font-medium transition-all duration-200 flex items-center gap-3 min-w-[160px]"
				>
					<!-- Drapeau image -->
					<img
						src={getLanguageFlagPath(lang)}
						alt={getLanguageName(lang)}
						class="w-8 h-8 object-contain rounded"
					/>

					<!-- Nom de la langue -->
					<span class="flex-1 text-left">{getLanguageName(lang)}</span>

					<!-- Icône de sélection -->
					{#if isSelected}
						<svg class="w-5 h-5 text-primary" fill="currentColor" viewBox="0 0 20 20">
							<path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
						</svg>
					{/if}
				</button>
			{/each}
		</div>
	</div>
{/if}
