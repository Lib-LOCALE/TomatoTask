<script lang="ts">
	// Panneau de paramètres complet
	import { _ } from 'svelte-i18n';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import LanguageSelector from './LanguageSelector.svelte';
	import ThemeToggle from './ThemeToggle.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import type { Settings } from '$lib/types';

	// Props
	interface Props {
		isOpen: boolean;
		onClose: () => void;
	}

	let { isOpen = $bindable(), onClose }: Props = $props();

	// Copie locale des paramètres pour l'édition
	let localSettings = $state<Settings>({ ...settingsStore.settings });
	let isSaving = $state(false);
	let saveSuccess = $state(false);

	// Synchronise avec les paramètres du store
	$effect(() => {
		localSettings = { ...settingsStore.settings };
	});

	/**
	 * Sauvegarde les paramètres
	 */
	async function handleSave() {
		isSaving = true;
		try {
			await settingsStore.save(localSettings);
			saveSuccess = true;

			// Cache le message de succès après 2 secondes
			setTimeout(() => {
				saveSuccess = false;
			}, 2000);
		} catch (error) {
			console.error('Failed to save settings:', error);
		} finally {
			isSaving = false;
		}
	}

	/**
	 * Réinitialise aux valeurs par défaut
	 */
	async function handleReset() {
		if (confirm($_('settings.resetDefaults') + '?')) {
			try {
				await settingsStore.reset();
				localSettings = { ...settingsStore.settings };
			} catch (error) {
				console.error('Failed to reset settings:', error);
			}
		}
	}

	/**
	 * Gère la fermeture du panneau
	 */
	function handleClose() {
		isOpen = false;
		onClose();
	}

	/**
	 * Gère le clic sur le backdrop
	 */
	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			handleClose();
		}
	}
</script>

{#if isOpen}
	<dialog
		open
		onclick={handleBackdropClick}
		class="rounded-lg border bg-background p-0 shadow-lg backdrop:bg-black/50 max-w-2xl w-full max-h-[90vh] overflow-y-auto"
	>
		<div class="w-full">
			<!-- Header -->
			<div class="flex items-center justify-between border-b px-6 py-4">
				<h2 class="text-lg font-semibold">
					{$_('settings.title')}
				</h2>

				<button
					type="button"
					onclick={handleClose}
					class="rounded-md p-1 hover:bg-muted"
				>
					<svg
						class="h-5 w-5"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						viewBox="0 0 24 24"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Contenu -->
			<div class="px-6 py-4 space-y-6">
				<!-- Timer Settings -->
				<section>
					<h3 class="text-md font-semibold mb-4">{$_('settings.timer')}</h3>
					<div class="space-y-4">
						<!-- Work Duration -->
						<div>
							<label for="work-duration" class="block text-sm font-medium mb-1">
								{$_('settings.workDuration')}
							</label>
							<div class="flex items-center gap-2">
								<Input
									id="work-duration"
									type="number"
									bind:value={localSettings.workDuration}
									min="1"
									max="60"
									class="w-24"
								/>
								<span class="text-sm text-muted-foreground">{$_('common.minutes')}</span>
							</div>
						</div>

						<!-- Short Break Duration -->
						<div>
							<label for="short-break-duration" class="block text-sm font-medium mb-1">
								{$_('settings.shortBreakDuration')}
							</label>
							<div class="flex items-center gap-2">
								<Input
									id="short-break-duration"
									type="number"
									bind:value={localSettings.shortBreakDuration}
									min="1"
									max="30"
									class="w-24"
								/>
								<span class="text-sm text-muted-foreground">{$_('common.minutes')}</span>
							</div>
						</div>

						<!-- Long Break Duration -->
						<div>
							<label for="long-break-duration" class="block text-sm font-medium mb-1">
								{$_('settings.longBreakDuration')}
							</label>
							<div class="flex items-center gap-2">
								<Input
									id="long-break-duration"
									type="number"
									bind:value={localSettings.longBreakDuration}
									min="1"
									max="60"
									class="w-24"
								/>
								<span class="text-sm text-muted-foreground">{$_('common.minutes')}</span>
							</div>
						</div>

						<!-- Pomodoros Until Long Break -->
						<div>
							<label for="pomodoros-until-long-break" class="block text-sm font-medium mb-1">
								{$_('settings.pomodorosUntilLongBreak')}
							</label>
							<Input
								id="pomodoros-until-long-break"
								type="number"
								bind:value={localSettings.pomosorosUntilLongBreak}
								min="2"
								max="10"
								class="w-24"
							/>
						</div>

						<!-- Auto-start Options -->
						<div class="space-y-2">
							<label class="flex items-center gap-2">
								<input
									type="checkbox"
									bind:checked={localSettings.autoStartBreaks}
									class="h-4 w-4 rounded border-input"
								/>
								<span class="text-sm">{$_('settings.autoStartBreaks')}</span>
							</label>

							<label class="flex items-center gap-2">
								<input
									type="checkbox"
									bind:checked={localSettings.autoStartPomodoros}
									class="h-4 w-4 rounded border-input"
								/>
								<span class="text-sm">{$_('settings.autoStartPomodoros')}</span>
							</label>
						</div>
					</div>
				</section>

				<!-- Appearance Settings -->
				<section>
					<h3 class="text-md font-semibold mb-4">{$_('settings.appearance')}</h3>
					<div class="space-y-4">
						<!-- Theme Toggle -->
						<div>
							<label class="block text-sm font-medium mb-2">
								{$_('settings.theme')}
							</label>
							<div class="flex items-center gap-2">
								<ThemeToggle variant="toggle" />
								<span class="text-sm text-muted-foreground">
									{localSettings.theme === 'dark' ? $_('settings.dark') : $_('settings.light')}
								</span>
							</div>
						</div>

						<!-- Language Selector -->
						<div>
							<LanguageSelector variant="dropdown" />
						</div>
					</div>
				</section>

				<!-- Success Message -->
				{#if saveSuccess}
					<div class="rounded-md bg-green-50 dark:bg-green-950 border border-green-200 dark:border-green-800 p-3 text-sm text-green-700 dark:text-green-300">
						{$_('settings.saved')}
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="border-t px-6 py-4 flex justify-between">
				<Button type="button" variant="outline" onclick={handleReset}>
					{$_('settings.resetDefaults')}
				</Button>

				<div class="flex gap-2">
					<Button type="button" variant="outline" onclick={handleClose}>
						{$_('common.cancel')}
					</Button>

					<Button type="button" onclick={handleSave} disabled={isSaving}>
						{isSaving ? $_('common.loading') : $_('common.save')}
					</Button>
				</div>
			</div>
		</div>
	</dialog>
{/if}

<style>
	dialog::backdrop {
		background-color: rgba(0, 0, 0, 0.5);
	}
</style>
