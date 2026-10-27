<script lang="ts">
	// Panneau de paramètres complet
	import { _ } from 'svelte-i18n';
	import { untrack } from 'svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import LanguageSelector from './LanguageSelector.svelte';
	import SoundControl from '../sounds/SoundControl.svelte';
	import { playNotificationSound } from '$lib/services/notification-service';

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
	let errorMessage = $state('');
	let drawer = $state<HTMLElement>();

	// Synchronise avec les paramètres du store
	$effect(() => {
		if (isOpen) localSettings = untrack(() => ({ ...settingsStore.settings }));
	});

	/**
	 * Sauvegarde les paramètres
	 */
	async function handleSave() {
		if (!drawer) return;
		const invalid = Array.from(
			drawer.querySelectorAll<HTMLInputElement>('input[type="number"]')
		).find((input) => !input.checkValidity());
		if (invalid) {
			invalid.reportValidity();
			return;
		}
		isSaving = true;
		try {
			await settingsStore.save({ ...localSettings, language: settingsStore.settings.language });
			errorMessage = '';
			saveSuccess = true;

			// Cache le message de succès après 2 secondes
			setTimeout(() => {
				saveSuccess = false;
			}, 2000);
		} catch (error) {
			console.error('Failed to save settings:', error);
			errorMessage = $_('chrome.settings.saveError');
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
</script>

{#if isOpen}
	<aside bind:this={drawer} class="settings-drawer" aria-labelledby="settings-title">
		<div class="w-full" role="none">
			<!-- Header -->
			<div class="flex items-center justify-between border-b px-6 py-4">
				<h2 id="settings-title" class="text-foreground text-lg font-semibold">
					{$_('settings.title')}
				</h2>

				<button
					type="button"
					onclick={handleClose}
					class="hover:bg-muted rounded-md p-1"
					aria-label={$_('common.close')}
				>
					<svg
						class="h-5 w-5"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						viewBox="0 0 24 24"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
					</svg>
				</button>
			</div>

			<!-- Contenu -->
			<div class="space-y-6 px-6 py-4">
				<!-- Timer Settings -->
				<section>
					<h3 class="text-md text-foreground mb-4 font-semibold">{$_('settings.timer')}</h3>
					<div class="space-y-4">
						<div class="flex flex-wrap gap-2" aria-label={$_('chrome.settings.presets')}>
							{#each [[25, 5, 15], [50, 10, 30]] as preset}
								<button
									type="button"
									class="rounded border px-3 py-1"
									onclick={() => {
										[
											localSettings.workDuration,
											localSettings.shortBreakDuration,
											localSettings.longBreakDuration
										] = preset;
									}}>{preset.join(' / ')}</button
								>
							{/each}
						</div>
						<!-- Work Duration -->
						<div>
							<label for="work-duration" class="text-foreground mb-1 block text-sm font-medium">
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
								<span class="text-foreground/70 text-sm">{$_('common.minutes')}</span>
							</div>
						</div>

						<!-- Short Break Duration -->
						<div>
							<label
								for="short-break-duration"
								class="text-foreground mb-1 block text-sm font-medium"
							>
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
								<span class="text-foreground/70 text-sm">{$_('common.minutes')}</span>
							</div>
						</div>

						<!-- Long Break Duration -->
						<div>
							<label
								for="long-break-duration"
								class="text-foreground mb-1 block text-sm font-medium"
							>
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
								<span class="text-foreground/70 text-sm">{$_('common.minutes')}</span>
							</div>
						</div>

						<!-- Pomodoros Until Long Break -->
						<div>
							<label
								for="pomodoros-until-long-break"
								class="text-foreground mb-1 block text-sm font-medium"
							>
								{$_('settings.pomodorosUntilLongBreak')}
							</label>
							<Input
								id="pomodoros-until-long-break"
								type="number"
								bind:value={localSettings.pomodorosUntilLongBreak}
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
									class="border-input h-4 w-4 rounded"
								/>
								<span class="text-foreground text-sm">{$_('settings.autoStartBreaks')}</span>
							</label>

							<label class="flex items-center gap-2">
								<input
									type="checkbox"
									bind:checked={localSettings.autoStartPomodoros}
									class="border-input h-4 w-4 rounded"
								/>
								<span class="text-foreground text-sm">{$_('settings.autoStartPomodoros')}</span>
							</label>
						</div>
					</div>
				</section>

				<section>
					<SoundControl flat />
					<div class="mt-4 flex flex-wrap items-end gap-3">
						<label for="notification-sound"
							>{$_('chrome.settings.endSound')}
							<select
								id="notification-sound"
								bind:value={localSettings.notificationSound}
								class="border-input bg-background mt-1 block rounded border p-2"
							>
								<option value="default">{$_('chrome.settings.soundDefault')}</option>
								<option value="none">{$_('chrome.settings.soundNone')}</option>
							</select>
						</label>
						<button
							type="button"
							class="rounded border px-3 py-2"
							disabled={localSettings.notificationSound === 'none'}
							onclick={() => playNotificationSound('squashed_tomato')}
							>{$_('chrome.settings.preview')}</button
						>
					</div>
				</section>

				<!-- Appearance Settings -->
				<section>
					<h3 class="text-md text-foreground mb-4 font-semibold">{$_('settings.appearance')}</h3>
					<div class="space-y-4">
						<!-- Theme Toggle -->
						<div>
							<div class="text-foreground mb-2 block text-sm font-medium">
								{$_('settings.theme')}
							</div>
							<select
								class="border-input bg-background rounded border p-2"
								aria-label={$_('settings.theme')}
								bind:value={localSettings.theme}
							>
								<option value="light">{$_('settings.light')}</option>
								<option value="dark">{$_('settings.dark')}</option>
								<option value="system">{$_('chrome.settings.system')}</option>
							</select>
						</div>

						<!-- Language Selector -->
						<div>
							<LanguageSelector variant="buttons" />
						</div>
					</div>
				</section>

				<!-- Data Management -->
				<!-- Data Management -->
				<section>
					<h3 class="text-md text-foreground mb-4 font-semibold">
						{$_('settings.dataManagement.title')}
					</h3>
					<div class="flex flex-col gap-3">
						<div class="flex items-center justify-between rounded-lg border p-3">
							<div>
								<div class="font-medium">{$_('settings.dataManagement.backupTitle')}</div>
								<div class="text-muted-foreground text-xs">
									{$_('settings.dataManagement.backupDesc')}
								</div>
							</div>
							<Button
								variant="outline"
								size="sm"
								onclick={async () => {
									try {
										const { exportBackup } = await import('$lib/services/backup-service');
										await exportBackup();
										errorMessage = '';
									} catch {
										errorMessage = $_('chrome.settings.exportError');
									}
								}}
							>
								{$_('settings.dataManagement.export')}
							</Button>
						</div>

						<div class="flex items-center justify-between rounded-lg border p-3">
							<div>
								<div class="font-medium">{$_('settings.dataManagement.restoreTitle')}</div>
								<div class="text-muted-foreground text-xs">
									{$_('settings.dataManagement.restoreDesc')}
								</div>
							</div>
							<Button
								variant="outline"
								size="sm"
								onclick={async () => {
									if (confirm($_('settings.dataManagement.overwriteConfirm'))) {
										try {
											const { importBackup } = await import('$lib/services/backup-service');
											await importBackup();
											errorMessage = '';
										} catch {
											errorMessage = $_('chrome.settings.importError');
										}
									}
								}}
							>
								{$_('settings.dataManagement.import')}
							</Button>
						</div>
					</div>
				</section>

				<section>
					<h3 class="text-md mb-3 font-semibold">{$_('keyboard.shortcuts')}</h3>
					<p>Ctrl+S · {$_('keyboard.startStop')}</p>
					<p>Ctrl+N · {$_('keyboard.newTask')}</p>
					<p>Ctrl+L · {$_('keyboard.changeLanguage')}</p>
					<p>Ctrl+/ · {$_('keyboard.shortcuts')}</p>
				</section>
				{#if errorMessage}<p role="alert" class="text-destructive">{errorMessage}</p>{/if}
				<!-- Success Message -->
				{#if saveSuccess}
					<div
						class="rounded-md border border-green-200 bg-green-50 p-3 text-sm text-green-700 dark:border-green-800 dark:bg-green-950 dark:text-green-300"
					>
						{$_('settings.saved')}
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="flex justify-between border-t px-6 py-4">
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
	</aside>
{/if}

<style>
	.settings-drawer {
		position: fixed;
		z-index: 50;
		inset: 0 0 0 auto;
		width: min(480px, 100%);
		overflow-y: auto;
		border-left: 1px solid var(--border);
		background: var(--background);
		color: var(--foreground);
		box-shadow: -2px 0 8px rgb(16 28 51 / 0.15);
	}
</style>
