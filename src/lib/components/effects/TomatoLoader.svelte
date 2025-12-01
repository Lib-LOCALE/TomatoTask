<script lang="ts">
	interface Props {
		size?: number;
		text?: string;
	}

	let { size = 64, text = 'Loading...' }: Props = $props();
</script>

<div class="flex flex-col items-center justify-center gap-4">
	<div class="tomato-loader" style:width="{size}px" style:height="{size}px">
		<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
			<defs>
				<style>
					.stem {
						fill: #4caf50;
					}
					.stem-dark {
						fill: #2e7d32;
					}
					.body {
						fill: #f44336;
					}
					.body-dark {
						fill: #c62828;
					}
					.body-light {
						fill: #ffcdd2;
					}
				</style>
			</defs>

			<!-- Stem -->
			<rect x="7" y="0" width="2" height="1" class="stem" />
			<rect x="6" y="1" width="4" height="1" class="stem" />
			<rect x="5" y="2" width="6" height="1" class="stem" />
			<rect x="7" y="2" width="2" height="1" class="stem-dark" />

			<!-- Body -->
			<rect x="4" y="3" width="8" height="1" class="body" />
			<rect x="3" y="4" width="10" height="1" class="body" />
			<rect x="2" y="5" width="12" height="1" class="body" />
			<rect x="1" y="6" width="14" height="6" class="body" />
			<rect x="2" y="12" width="12" height="1" class="body" />
			<rect x="3" y="13" width="10" height="1" class="body" />
			<rect x="4" y="14" width="8" height="1" class="body" />

			<!-- Shine -->
			<rect x="3" y="5" width="2" height="1" class="body-light" />
			<rect x="2" y="6" width="1" height="2" class="body-light" />
			<rect x="3" y="6" width="1" height="1" class="body-light" />
		</svg>
	</div>

	{#if text}
		<p class="text-muted-foreground animate-pulse text-sm font-medium">{text}</p>
	{/if}
</div>

<style>
	.tomato-loader {
		animation: spin 1.5s cubic-bezier(0.68, -0.55, 0.265, 1.55) infinite;
	}

	@keyframes spin {
		0% {
			transform: rotate(0deg) scale(1);
		}
		50% {
			transform: rotate(180deg) scale(1.2);
		}
		100% {
			transform: rotate(360deg) scale(1);
		}
	}
</style>
