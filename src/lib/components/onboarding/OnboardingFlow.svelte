<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import { changeLanguage, SUPPORTED_LANGUAGES, getLanguageName, getLanguageCode, getLanguageFlag } from '$lib/services/i18n-service';
	import type { Language } from '$lib/types';

	interface Props {
		onComplete: () => void;
	}

	let { onComplete }: Props = $props();

	let currentStep = $state(0);
	let selectedLanguage = $state<Language>('en');

	const steps = [
		'language',
		'welcome',
		'guide'
	];

	async function handleLanguageSelect(lang: Language) {
		selectedLanguage = lang;
		changeLanguage(lang);
		await settingsStore.updateSetting('language', lang);
		nextStep();
	}

	function nextStep() {
		if (currentStep < steps.length - 1) {
			currentStep++;
		} else {
			complete();
		}
	}

	function prevStep() {
		if (currentStep > 0) {
			currentStep--;
		}
	}

	async function complete() {
		// Marque l'onboarding comme complété dans le localStorage
		localStorage.setItem('tomatotask_onboarding_completed', 'true');
		onComplete();
	}
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm">
	<div class="max-w-2xl w-full bg-background rounded-xl shadow-2xl p-8 m-4 max-h-[90vh] overflow-y-auto">
		<!-- Progress indicator -->
		<div class="flex justify-center gap-2 mb-8">
			{#each steps as _, index}
				<div
					class="h-2 w-16 rounded-full transition-colors"
					class:bg-primary={index <= currentStep}
					class:bg-muted={index > currentStep}
				></div>
			{/each}
		</div>

		<!-- Step 1: Language Selection -->
		{#if currentStep === 0}
			<div class="text-center">
				<h1 class="text-3xl font-bold mb-4">🍅 TomatoTask</h1>
				<h2 class="text-xl mb-6">Choose your language / Choisissez votre langue</h2>

				<div class="grid grid-cols-2 md:grid-cols-3 gap-3 max-w-lg mx-auto">
					{#each SUPPORTED_LANGUAGES as lang}
						<button
							type="button"
							onclick={() => handleLanguageSelect(lang)}
							class="p-4 rounded-lg border-2 transition-all hover:border-primary hover:bg-accent flex flex-col items-center gap-2"
						>
							<div class="text-4xl">{getLanguageFlag(lang)}</div>
							<div class="text-sm font-medium">{getLanguageName(lang)}</div>
						</button>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Step 2: Pomodoro Explanation -->
		{#if currentStep === 1}
			<div class="text-center">
				<div class="text-6xl mb-6">🍅</div>
				<h2 class="text-2xl font-bold mb-4">{$_('onboarding.welcome.title')}</h2>
				<p class="text-lg text-muted-foreground mb-6">{$_('onboarding.welcome.subtitle')}</p>

				<div class="text-left max-w-lg mx-auto space-y-4 mb-8">
					<div class="flex items-start gap-3">
						<div class="text-2xl">⏱️</div>
						<div>
							<h3 class="font-semibold mb-1">{$_('onboarding.pomodoro.step1.title')}</h3>
							<p class="text-sm text-muted-foreground">{$_('onboarding.pomodoro.step1.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">☕</div>
						<div>
							<h3 class="font-semibold mb-1">{$_('onboarding.pomodoro.step2.title')}</h3>
							<p class="text-sm text-muted-foreground">{$_('onboarding.pomodoro.step2.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">🎯</div>
						<div>
							<h3 class="font-semibold mb-1">{$_('onboarding.pomodoro.step3.title')}</h3>
							<p class="text-sm text-muted-foreground">{$_('onboarding.pomodoro.step3.desc')}</p>
						</div>
					</div>
				</div>

				<div class="flex gap-3 justify-center">
					<button
						type="button"
						onclick={prevStep}
						class="px-6 py-2 rounded-md border hover:bg-muted"
					>
						{$_('common.back')}
					</button>
					<button
						type="button"
						onclick={nextStep}
						class="px-6 py-2 rounded-md bg-primary text-primary-foreground hover:bg-primary/90"
					>
						{$_('common.next')}
					</button>
				</div>
			</div>
		{/if}

		<!-- Step 3: Quick Guide -->
		{#if currentStep === 2}
			<div class="text-center">
				<h2 class="text-2xl font-bold mb-6">{$_('onboarding.guide.title')}</h2>

				<div class="text-left max-w-lg mx-auto space-y-4 mb-8">
					<div class="flex items-start gap-3">
						<div class="text-2xl">1️⃣</div>
						<div>
							<h3 class="font-semibold mb-1">{$_('onboarding.guide.step1.title')}</h3>
							<p class="text-sm text-muted-foreground">{$_('onboarding.guide.step1.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">2️⃣</div>
						<div>
							<h3 class="font-semibold mb-1">{$_('onboarding.guide.step2.title')}</h3>
							<p class="text-sm text-muted-foreground">{$_('onboarding.guide.step2.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">3️⃣</div>
						<div>
							<h3 class="font-semibold mb-1">{$_('onboarding.guide.step3.title')}</h3>
							<p class="text-sm text-muted-foreground">{$_('onboarding.guide.step3.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">⌨️</div>
						<div>
							<h3 class="font-semibold mb-1">{$_('onboarding.guide.shortcuts.title')}</h3>
							<p class="text-sm text-muted-foreground">{$_('onboarding.guide.shortcuts.desc')}</p>
						</div>
					</div>
				</div>

				<div class="flex gap-3 justify-center">
					<button
						type="button"
						onclick={prevStep}
						class="px-6 py-2 rounded-md border hover:bg-muted"
					>
						{$_('common.back')}
					</button>
					<button
						type="button"
						onclick={complete}
						class="px-6 py-2 rounded-md bg-primary text-primary-foreground hover:bg-primary/90"
					>
						{$_('onboarding.guide.start')} 🚀
					</button>
				</div>
			</div>
		{/if}
	</div>
</div>
