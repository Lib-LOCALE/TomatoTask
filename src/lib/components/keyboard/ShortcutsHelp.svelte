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
		// Vérifie si on clique directement sur le dialog (et non sur son contenu)
		const target = event.target as HTMLElement;
		if (target.tagName === 'DIALOG') {
			handleClose();
		}
	}
</script>

{#if isOpen}
	<dialog
		open
		onclick={handleBackdropClick}
		class="border-border bg-background fixed top-1/2 left-1/2 z-50 m-0 -translate-x-1/2 -translate-y-1/2 rounded-lg border p-0 shadow-2xl"
	>
		<div class="w-full max-w-2xl">
			<!-- Header -->
			<div class="flex items-center justify-between border-b px-6 py-4">
				<h2 class="text-foreground text-lg font-semibold">
					{$_('keyboard.shortcuts')}
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
				<!-- Timer Shortcuts -->
				<div>
					<h3 class="text-foreground/70 mb-2 text-sm font-semibold uppercase">
						{$_('timer.work')}
					</h3>
					<div class="space-y-2">
						{#each groupedShortcuts().timer as shortcut}
							<div class="flex items-center justify-between">
								<span class="text-foreground text-sm">{$_(shortcut.description)}</span>
								<kbd
									class="border-muted bg-muted rounded-md border px-3 py-1.5 font-mono text-xs font-semibold"
								>
									{shortcut.key}
								</kbd>
							</div>
						{/each}
					</div>
				</div>

				<!-- Task Shortcuts -->
				<div>
					<h3 class="text-foreground/70 mb-2 text-sm font-semibold uppercase">
						{$_('tasks.title')}
					</h3>
					<div class="space-y-2">
						{#each groupedShortcuts().tasks as shortcut}
							<div class="flex items-center justify-between">
								<span class="text-foreground text-sm">{$_(shortcut.description)}</span>
								<kbd
									class="border-muted bg-muted rounded-md border px-3 py-1.5 font-mono text-xs font-semibold"
								>
									{shortcut.key}
								</kbd>
							</div>
						{/each}
					</div>
				</div>

				<!-- Settings Shortcuts -->
				<div>
					<h3 class="text-foreground/70 mb-2 text-sm font-semibold uppercase">
						{$_('settings.title')}
					</h3>
					<div class="space-y-2">
						{#each groupedShortcuts().settings as shortcut}
							<div class="flex items-center justify-between">
								<span class="text-foreground text-sm">{$_(shortcut.description)}</span>
								<kbd
									class="border-muted bg-muted rounded-md border px-3 py-1.5 font-mono text-xs font-semibold"
								>
									{shortcut.key}
								</kbd>
							</div>
						{/each}
					</div>
				</div>

				<!-- Help Shortcuts -->
				<div>
					<h3 class="text-foreground/70 mb-2 text-sm font-semibold uppercase">Help</h3>
					<div class="space-y-2">
						{#each groupedShortcuts().help as shortcut}
							<div class="flex items-center justify-between">
								<span class="text-foreground text-sm">{$_(shortcut.description)}</span>
								<kbd
									class="border-muted bg-muted rounded-md border px-3 py-1.5 font-mono text-xs font-semibold"
								>
									{shortcut.key}
								</kbd>
							</div>
						{/each}
					</div>
				</div>
			</div>

			<!-- Footer -->
			<div class="flex justify-end border-t px-6 py-4">
				<button
					type="button"
					onclick={handleClose}
					class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-4 py-2"
				>
					{$_('common.close')}
				</button>
			</div>
		</div>
	</dialog>
{/if}

<style>
	dialog {
		position: fixed;
		margin: 0;
	}

	dialog::backdrop {
		background-color: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
	}
</style>
