<script lang="ts">
	import { soundService, sounds, type Sound } from '$lib/services/sound-service';
	import { Volume2, VolumeX, Play, Pause, Music } from '@lucide/svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { _ } from 'svelte-i18n';

	// Stores
	const isPlaying = soundService.isPlaying;
	const currentSound = soundService.currentSound;
	const volume = soundService.volume;

	let showVolumeControl = $state(false);

	function handleSoundSelect(sound: Sound) {
		if ($currentSound?.id === sound.id) {
			soundService.toggle();
		} else {
			soundService.play(sound);
		}
	}

	function handleVolumeChange(e: Event) {
		const target = e.target as HTMLInputElement;
		soundService.setVolume(parseFloat(target.value));
	}
</script>

<div class="bg-card flex flex-col gap-4 rounded-lg border p-4 shadow-sm">
	<div class="flex items-center justify-between">
		<h3 class="flex items-center gap-2 font-semibold">
			<Music class="h-4 w-4" />
			{$_('sounds.title')}
		</h3>

		<!-- Volume Toggle -->
		<div class="relative">
			<Button
				variant="ghost"
				size="icon"
				class="h-8 w-8"
				onclick={() => (showVolumeControl = !showVolumeControl)}
			>
				{#if $volume === 0}
					<VolumeX class="h-4 w-4" />
				{:else}
					<Volume2 class="h-4 w-4" />
				{/if}
			</Button>

			{#if showVolumeControl}
				<div
					class="bg-popover absolute top-full right-0 z-10 mt-2 w-32 rounded-md border p-2 shadow-md"
				>
					<input
						type="range"
						min="0"
						max="1"
						step="0.01"
						value={$volume}
						oninput={handleVolumeChange}
						class="accent-primary w-full cursor-pointer"
					/>
				</div>
			{/if}
		</div>
	</div>

	<!-- Sound List -->
	<div class="grid grid-cols-2 gap-2">
		{#each sounds as sound}
			<button
				class="hover:bg-muted flex items-center justify-between rounded-md border p-2 text-sm transition-colors
                {$currentSound?.id === sound.id && $isPlaying
					? 'border-primary bg-primary/10 text-primary'
					: ''}"
				onclick={() => handleSoundSelect(sound)}
			>
				<span>{sound.name}</span>
				{#if $currentSound?.id === sound.id && $isPlaying}
					<Pause class="h-3 w-3" />
				{:else}
					<Play class="h-3 w-3 opacity-50" />
				{/if}
			</button>
		{/each}
	</div>
</div>
