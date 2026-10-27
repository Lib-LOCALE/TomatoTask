<script lang="ts">
	import { onMount } from 'svelte';
	import { _ } from 'svelte-i18n';
	import { taskStore } from '$lib/stores/tasks.svelte';
	import Tile from '$lib/components/majolica/Tile.svelte';

	let { onComplete }: { onComplete: () => void } = $props();
	let step = $state(0);
	let title = $state('');
	let error = $state(false);
	let saving = $state(false);
	let dialog: HTMLDialogElement;
	onMount(() => {
		dialog.showModal();
	});
	function complete() {
		localStorage.setItem('tomatotask_onboarding_completed', 'true');
		onComplete();
	}
	async function createTask() {
		if (!title.trim()) return;
		saving = true;
		error = false;
		try {
			const task = await taskStore.create({ title: title.trim(), estimatedPomodoros: 1 });
			taskStore.select(task);
			complete();
		} catch {
			error = true;
		} finally {
			saving = false;
		}
	}
</script>

<dialog bind:this={dialog} oncancel={complete} aria-labelledby="onboarding-title">
	<header>
		<span>TomatoTask</span><button type="button" onclick={complete}
			>{$_('chrome.onboarding.skip')}</button
		>
	</header>
	{#if step === 0}
		<div class="tiles">
			<Tile state="empty" /><Tile state="painting" progress={0.5} /><Tile state="glazed" />
		</div>
		<h1 id="onboarding-title">{$_('chrome.onboarding.tileTitle')}</h1>
		<p>{$_('chrome.onboarding.tileText')}</p>
	{:else if step === 1}
		<div class="tiles">
			<Tile state="glazed" /><Tile state="glazed" position={1} /><Tile state="empty" position={2} />
		</div>
		<h1 id="onboarding-title">{$_('chrome.onboarding.dayTitle')}</h1>
		<p>{$_('chrome.onboarding.dayText')}</p>
	{:else}
		<h1 id="onboarding-title">{$_('chrome.onboarding.taskTitle')}</h1>
		<p>{$_('chrome.onboarding.taskText')}</p>
		<form
			onsubmit={(event) => {
				event.preventDefault();
				void createTask();
			}}
		>
			<label for="first-task">{$_('chrome.onboarding.taskLabel')}</label>
			<input id="first-task" bind:value={title} required maxlength="200" />
			{#if error}<p role="alert">{$_('chrome.onboarding.error')}</p>{/if}
			<button class="primary" type="submit" disabled={saving || !title.trim()}
				>{saving ? $_('common.loading') : $_('chrome.onboarding.create')}</button
			>
		</form>
	{/if}
	<footer>
		<span>{step + 1} / 3</span>
		<div>
			{#if step > 0}<button type="button" onclick={() => step--}>{$_('common.back')}</button
				>{/if}{#if step < 2}<button class="primary" type="button" onclick={() => step++}
					>{$_('common.next')}</button
				>{/if}
		</div>
	</footer>
</dialog>

<style>
	dialog {
		width: min(520px, calc(100% - 32px));
		margin: auto;
		padding: 24px;
		border: 1px solid var(--border);
		border-radius: 4px;
		color: var(--foreground);
		background: var(--card);
	}
	dialog::backdrop {
		background: rgb(16 28 51 / 0.55);
	}
	header,
	footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 16px;
	}
	header {
		margin-bottom: 28px;
	}
	header span {
		font-weight: 700;
	}
	header button {
		text-decoration: underline;
	}
	.tiles {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1px;
		width: min(100%, 300px);
		margin-bottom: 24px;
		padding: 1px;
		background: var(--grout);
	}
	h1 {
		font-size: 2rem;
		margin-bottom: 12px;
	}
	p {
		color: var(--muted-foreground);
	}
	form {
		display: grid;
		gap: 8px;
		margin-top: 20px;
	}
	input {
		border: 1px solid var(--input);
		border-radius: 4px;
		padding: 8px;
		background: var(--background);
		color: var(--foreground);
	}
	footer {
		margin-top: 32px;
		border-top: 1px solid var(--border);
		padding-top: 16px;
	}
	footer div {
		display: flex;
		gap: 8px;
	}
	footer button,
	.primary {
		padding: 7px 14px;
		border-radius: 4px;
	}
	.primary {
		background: var(--primary);
		color: var(--primary-foreground);
	}
	.primary:disabled {
		opacity: 0.6;
	}
</style>
