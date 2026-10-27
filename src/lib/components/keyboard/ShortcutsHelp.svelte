<script lang="ts">
	import { _ } from 'svelte-i18n';
	import { X } from '@lucide/svelte';
	let { isOpen = $bindable(), onClose }: { isOpen: boolean; onClose: () => void } = $props();
	let dialog: HTMLDialogElement;
	const shortcuts = [
		['Ctrl+S', 'keyboard.startStop'],
		['Ctrl+N', 'keyboard.newTask'],
		['Ctrl+L', 'keyboard.changeLanguage'],
		['Ctrl+/', 'keyboard.shortcuts']
	];
	$effect(() => {
		if (isOpen && dialog && !dialog.open) dialog.showModal();
		else if (!isOpen && dialog?.open) dialog.close();
	});
	function close() {
		isOpen = false;
		onClose();
	}
</script>

<dialog bind:this={dialog} onclose={close} aria-labelledby="shortcuts-title">
	<header>
		<h2 id="shortcuts-title">{$_('keyboard.shortcuts')}</h2>
		<button type="button" onclick={close} aria-label={$_('common.close')}><X size={20} /></button>
	</header>
	<dl>
		{#each shortcuts as [key, description]}
			<div>
				<dt>{$_(description)}</dt>
				<dd><kbd>{key}</kbd></dd>
			</div>
		{/each}
	</dl>
	<footer><button type="button" onclick={close}>{$_('common.close')}</button></footer>
</dialog>

<style>
	dialog {
		width: min(440px, calc(100% - 32px));
		margin: auto;
		padding: 0;
		border: 1px solid var(--border);
		border-radius: 4px;
		background: var(--card);
		color: var(--foreground);
	}
	dialog::backdrop {
		background: rgb(16 28 51 / 0.55);
	}
	header,
	footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
	}
	header {
		border-bottom: 1px solid var(--border);
	}
	h2 {
		font-size: 1.5rem;
	}
	dl {
		padding: 8px 20px;
	}
	dl div {
		display: flex;
		justify-content: space-between;
		gap: 16px;
		padding: 10px 0;
	}
	kbd {
		padding: 2px 7px;
		border: 1px solid var(--border);
		border-radius: 3px;
		background: var(--muted);
		white-space: nowrap;
	}
	footer {
		justify-content: flex-end;
		border-top: 1px solid var(--border);
	}
	footer button {
		padding: 6px 14px;
		background: var(--primary);
		color: var(--primary-foreground);
		border-radius: 4px;
	}
</style>
