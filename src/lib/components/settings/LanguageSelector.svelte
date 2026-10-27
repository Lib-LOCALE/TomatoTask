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
		<label for="language-select" class="text-foreground mb-2 block text-sm font-medium">
			Language / Langue
		</label>

		<select
			id="language-select"
			onchange={handleSelectChange}
			value={currentLanguage}
			class="border-input bg-background text-foreground ring-offset-background focus:ring-ring flex h-10 w-full rounded-md border px-3 py-2 text-sm focus:ring-2 focus:ring-offset-2 focus:outline-none"
		>
			{#each SUPPORTED_LANGUAGES as lang}
				<option value={lang} class="text-foreground">
					{getLanguageFlag(lang)}
					{getLanguageName(lang)} ({getLanguageCode(lang)})
				</option>
			{/each}
		</select>
	</div>
{:else}
	<!-- Groupe de boutons avec drapeaux -->
	<div class="w-full">
		<div class="text-foreground mb-3 text-sm font-medium">Language / Langue</div>

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
					class="group text-foreground relative flex min-w-[160px] items-center gap-3 rounded-lg border-2 px-4 py-3 text-sm font-medium transition-all duration-200"
				>
					<!-- Drapeau image -->
					<img
						src={getLanguageFlagPath(lang)}
						alt={getLanguageName(lang)}
						class="h-8 w-8 rounded object-contain"
					/>

					<!-- Nom de la langue -->
					<span class="text-foreground flex-1 text-left">{getLanguageName(lang)}</span>

					<!-- Icône de sélection -->
					{#if isSelected}
						<svg class="text-primary h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
					{/if}
				</button>
			{/each}
		</div>
	</div>
{/if}
