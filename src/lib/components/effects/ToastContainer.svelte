<script lang="ts">
	import { toastStore } from '$lib/stores/toast.svelte';
	import Toast from './Toast.svelte';
</script>

<div class="toast-container fixed right-4 top-4 z-50 flex flex-col gap-2">
	{#each toastStore.toasts as toast (toast.id)}
		<Toast
			message={toast.message}
			type={toast.type}
			duration={toast.duration}
			onClose={() => toastStore.remove(toast.id)}
		/>
	{/each}
</div>
