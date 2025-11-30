<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import {
		changeLanguage,
		SUPPORTED_LANGUAGES,
		getLanguageName,
		getLanguageFlagPath
	} from '$lib/services/i18n-service';
	import type { Language } from '$lib/types';

	interface Props {
		onComplete: () => void;
	}

	let { onComplete }: Props = $props();

	let currentStep = $state(0);
	let selectedLanguage = $state<Language>('en');

	const steps = ['language', 'welcome', 'guide', 'features'];

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
	<div
		class="bg-background m-4 max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-xl p-8 shadow-2xl"
	>
		<!-- Progress indicator -->
		<div class="mb-8 flex justify-center gap-2">
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
				<h1 class="text-foreground mb-4 text-3xl font-bold">🍅 TomatoTask</h1>
				<h2 class="text-foreground mb-6 text-xl">Choose your language / Choisissez votre langue</h2>

				<div class="mx-auto grid max-w-lg grid-cols-2 gap-3 md:grid-cols-3">
					{#each SUPPORTED_LANGUAGES as lang}
						<button
							type="button"
							onclick={() => handleLanguageSelect(lang)}
							class="hover:border-primary hover:bg-accent flex flex-col items-center gap-2 rounded-lg border-2 p-4 transition-all"
						>
							<img
								src={getLanguageFlagPath(lang)}
								alt={getLanguageName(lang)}
								class="h-12 w-12 rounded object-contain"
							/>
							<div class="text-foreground text-sm font-medium">{getLanguageName(lang)}</div>
						</button>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Step 2: Pomodoro Explanation -->
		{#if currentStep === 1}
			<div class="text-center">
				<div class="mb-6 text-6xl">🍅</div>
				<h2 class="text-foreground mb-4 text-2xl font-bold">{$_('onboarding.welcome.title')}</h2>
				<p class="text-foreground/70 mb-6 text-lg">{$_('onboarding.welcome.subtitle')}</p>

				<div class="mx-auto mb-8 max-w-lg space-y-4 text-left">
					<div class="flex items-start gap-3">
						<div class="text-2xl">⏱️</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.pomodoro.step1.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.pomodoro.step1.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">☕</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.pomodoro.step2.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.pomodoro.step2.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">🎯</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.pomodoro.step3.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.pomodoro.step3.desc')}</p>
						</div>
					</div>
				</div>

				<div class="flex justify-center gap-3">
					<button
						type="button"
						onclick={prevStep}
						class="hover:bg-muted rounded-md border px-6 py-2"
					>
						{$_('common.back')}
					</button>
					<button
						type="button"
						onclick={nextStep}
						class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-6 py-2"
					>
						{$_('common.next')}
					</button>
				</div>
			</div>
		{/if}

		<!-- Step 3: Quick Guide -->
		{#if currentStep === 2}
			<div class="text-center">
				<h2 class="mb-6 text-2xl font-bold">{$_('onboarding.guide.title')}</h2>

				<div class="mx-auto mb-8 max-w-lg space-y-4 text-left">
					<div class="flex items-start gap-3">
						<div class="text-2xl">1️⃣</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.guide.step1.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.guide.step1.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">2️⃣</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.guide.step2.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.guide.step2.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">3️⃣</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.guide.step3.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.guide.step3.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">4️⃣</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.guide.step4.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.guide.step4.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">⌨️</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.guide.shortcuts.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.guide.shortcuts.desc')}</p>
						</div>
					</div>
				</div>

				<div class="flex justify-center gap-3">
					<button
						type="button"
						onclick={prevStep}
						class="hover:bg-muted rounded-md border px-6 py-2"
					>
						{$_('common.back')}
					</button>
					<button
						type="button"
						onclick={nextStep}
						class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-6 py-2"
					>
						{$_('common.next')}
					</button>
				</div>
			</div>
		{/if}

		<!-- Step 4: Features -->
		{#if currentStep === 3}
			<div class="text-center">
				<h2 class="mb-6 text-2xl font-bold">{$_('onboarding.features.title')}</h2>

				<div class="mx-auto mb-8 max-w-lg space-y-4 text-left">
					<div class="flex items-start gap-3">
						<div class="text-2xl">⇅</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.features.dragDrop.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.features.dragDrop.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">📊</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.features.stats.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.features.stats.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">💾</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.features.backup.title')}
							</h3>
							<p class="text-foreground/70 text-sm">{$_('onboarding.features.backup.desc')}</p>
						</div>
					</div>

					<div class="flex items-start gap-3">
						<div class="text-2xl">🔔</div>
						<div>
							<h3 class="text-foreground mb-1 font-semibold">
								{$_('onboarding.features.notifications.title')}
							</h3>
							<p class="text-foreground/70 text-sm">
								{$_('onboarding.features.notifications.desc')}
							</p>
						</div>
					</div>
				</div>

				<div class="flex justify-center gap-3">
					<button
						type="button"
						onclick={prevStep}
						class="hover:bg-muted rounded-md border px-6 py-2"
					>
						{$_('common.back')}
					</button>
					<button
						type="button"
						onclick={complete}
						class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-6 py-2"
					>
						{$_('onboarding.guide.start')} 🚀
					</button>
				</div>
			</div>
		{/if}
	</div>
</div>
