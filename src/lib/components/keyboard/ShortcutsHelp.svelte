<script lang="ts">
	// Modal d'aide pour les raccourcis clavier
	import { _ } from 'svelte-i18n';

	// Props
	interface Props {
		isOpen: boolean;
		onClose: () => void;
	}

	let { isOpen = $bindable(), onClose }: Props = $props();

	// Liste des raccourcis disponibles
	const shortcuts = [
		{
			key: 'Ctrl+S',
			description: 'keyboard.startStop',
			category: 'timer'
		},
		{
			key: 'Ctrl+N',
			description: 'keyboard.newTask',
			category: 'tasks'
		},
		{
			key: 'Ctrl+L',
			description: 'keyboard.changeLanguage',
			category: 'settings'
		},
		{
			key: 'Ctrl+/',
			description: 'keyboard.shortcuts',
			category: 'help'
		}
	];

	// Groupement par catégorie
	const groupedShortcuts = $derived(() => {
		const groups: Record<string, typeof shortcuts> = {
			timer: [],
			tasks: [],
			settings: [],
			help: []
		};

		shortcuts.forEach((shortcut) => {
			groups[shortcut.category].push(shortcut);
		});

		return groups;
	});

	/**
	 * Gère la fermeture du modal
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
		class="rounded-lg border bg-background p-0 shadow-lg backdrop:bg-black/50"
	>
		<div class="w-full max-w-2xl">
			<!-- Header -->
			<div class="flex items-center justify-between border-b px-6 py-4">
				<h2 class="text-lg font-semibold">
					{$_('keyboard.shortcuts')}
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
				<!-- Timer Shortcuts -->
				<div>
					<h3 class="text-sm font-semibold text-muted-foreground mb-2 uppercase">
						{$_('timer.work')}
					</h3>
					<div class="space-y-2">
						{#each groupedShortcuts().timer as shortcut}
							<div class="flex items-center justify-between">
								<span class="text-sm">{$_(shortcut.description)}</span>
								<kbd
									class="rounded-md border border-muted bg-muted px-3 py-1.5 text-xs font-mono font-semibold"
								>
									{shortcut.key}
								</kbd>
							</div>
						{/each}
					</div>
				</div>

				<!-- Task Shortcuts -->
				<div>
					<h3 class="text-sm font-semibold text-muted-foreground mb-2 uppercase">
						{$_('tasks.title')}
					</h3>
					<div class="space-y-2">
						{#each groupedShortcuts().tasks as shortcut}
							<div class="flex items-center justify-between">
								<span class="text-sm">{$_(shortcut.description)}</span>
								<kbd
									class="rounded-md border border-muted bg-muted px-3 py-1.5 text-xs font-mono font-semibold"
								>
									{shortcut.key}
								</kbd>
							</div>
						{/each}
					</div>
				</div>

				<!-- Settings Shortcuts -->
				<div>
					<h3 class="text-sm font-semibold text-muted-foreground mb-2 uppercase">
						{$_('settings.title')}
					</h3>
					<div class="space-y-2">
						{#each groupedShortcuts().settings as shortcut}
							<div class="flex items-center justify-between">
								<span class="text-sm">{$_(shortcut.description)}</span>
								<kbd
									class="rounded-md border border-muted bg-muted px-3 py-1.5 text-xs font-mono font-semibold"
								>
									{shortcut.key}
								</kbd>
							</div>
						{/each}
					</div>
				</div>

				<!-- Help Shortcuts -->
				<div>
					<h3 class="text-sm font-semibold text-muted-foreground mb-2 uppercase">
						Help
					</h3>
					<div class="space-y-2">
						{#each groupedShortcuts().help as shortcut}
							<div class="flex items-center justify-between">
								<span class="text-sm">{$_(shortcut.description)}</span>
								<kbd
									class="rounded-md border border-muted bg-muted px-3 py-1.5 text-xs font-mono font-semibold"
								>
									{shortcut.key}
								</kbd>
							</div>
						{/each}
					</div>
				</div>
			</div>

			<!-- Footer -->
			<div class="border-t px-6 py-4 flex justify-end">
				<button
					type="button"
					onclick={handleClose}
					class="rounded-md bg-primary px-4 py-2 text-primary-foreground hover:bg-primary/90"
				>
					{$_('common.close')}
				</button>
			</div>
		</div>
	</dialog>
{/if}

<style>
	dialog::backdrop {
		background-color: rgba(0, 0, 0, 0.5);
	}
</style>
