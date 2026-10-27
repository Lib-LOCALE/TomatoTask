<script lang="ts">
	import { locale, _ } from 'svelte-i18n';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { SUPPORTED_LANGUAGES, changeLanguage, getLanguageName } from '$lib/services/i18n-service';
	import type { Language } from '$lib/types';

	let { variant = 'dropdown' }: { variant?: 'dropdown' | 'buttons' } = $props();
	let currentLanguage = $derived($locale as Language);
	let error = $state(false);
	async function selectLanguage(language: Language) {
		try {
			await settingsStore.updateSetting('language', language);
			changeLanguage(language);
			error = false;
		} catch {
			error = true;
		}
	}
</script>

{#if variant === 'dropdown'}
	<label for="language-select">{$_('chrome.settings.language')}</label>
	<select
		id="language-select"
		value={currentLanguage}
		onchange={(event) => selectLanguage(event.currentTarget.value as Language)}
	>
		{#each SUPPORTED_LANGUAGES as language}<option value={language}
				>{getLanguageName(language)}</option
			>{/each}
	</select>
{:else}
	<fieldset>
		<legend>{$_('chrome.settings.language')}</legend>
		<div class="languages">
			{#each SUPPORTED_LANGUAGES as language}
				<button
					type="button"
					aria-pressed={currentLanguage === language}
					class:selected={currentLanguage === language}
					onclick={() => selectLanguage(language)}>{getLanguageName(language)}</button
				>
			{/each}
		</div>
	</fieldset>
{/if}
{#if error}<p role="alert">{$_('chrome.settings.languageError')}</p>{/if}

<style>
	legend,
	label {
		margin-bottom: 8px;
		font-weight: 600;
	}
	.languages {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}
	button,
	select {
		border: 1px solid var(--border);
		border-radius: 4px;
		background: var(--background);
		color: var(--foreground);
		padding: 6px 12px;
	}
	button.selected {
		background: var(--primary);
		color: var(--primary-foreground);
		border-color: var(--primary);
	}
	p {
		color: var(--destructive);
	}
</style>
