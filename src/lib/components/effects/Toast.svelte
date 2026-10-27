<script lang="ts">
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { CheckCircle2, CircleAlert, Info, X } from '@lucide/svelte';
	interface Props {
		message: string;
		type?: 'success' | 'error' | 'info' | 'celebration';
		duration?: number;
		onClose?: () => void;
	}
	let { message, type = 'info', duration = 3000, onClose }: Props = $props();
	onMount(() => {
		const timer = setTimeout(() => onClose?.(), duration);
		return () => clearTimeout(timer);
	});
</script>

<div class="toast" role={type === 'error' ? 'alert' : 'status'}>
	{#if type === 'error'}<CircleAlert size={20} />{:else if type === 'info'}<Info
			size={20}
		/>{:else}<CheckCircle2 size={20} />{/if}
	<p>{message}</p>
	<button type="button" onclick={() => onClose?.()} aria-label={$_('common.close')}
		><X size={18} /></button
	>
</div>

<style>
	.toast {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		width: min(360px, calc(100vw - 32px));
		padding: 14px;
		border: 1px solid var(--border);
		border-radius: 4px;
		background: var(--card);
		color: var(--foreground);
		box-shadow: var(--enamel-shadow);
	}
	p {
		flex: 1;
	}
	button {
		display: grid;
		place-items: center;
	}
</style>
