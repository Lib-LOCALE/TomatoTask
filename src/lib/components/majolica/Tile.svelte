<script lang="ts">
	import { patterns, patternIndex, type TileState } from './patterns';
	interface Props {
		state?: TileState;
		position?: number;
		progress?: number;
		serial?: number;
		animate?: boolean;
	}
	let { state = 'empty', position = 0, progress = 0, serial, animate = false }: Props = $props();
	const pattern = $derived(patterns[patternIndex(position)]);
	const reveal = $derived(
		state === 'glazed' ? 100 : state === 'painting' ? Math.max(0, Math.min(100, progress * 100)) : 0
	);
</script>

<div
	class="tile"
	class:glazed={state === 'glazed'}
	class:painting={state === 'painting'}
	class:animate
	aria-hidden="true"
>
	{#if state !== 'empty' && state !== 'planned'}<svg
			class="ornament"
			viewBox="0 0 100 100"
			preserveAspectRatio="none"
			aria-hidden="true"
		>
			<rect
				x="3"
				y="3"
				width="94"
				height="94"
				rx="1"
				fill="none"
				stroke="var(--cobalt)"
				stroke-width="2"
			/>
			<path
				d="M0 15 Q8 14 15 0 M0 9 Q5 8 9 0 M100 15 Q92 14 85 0 M100 9 Q95 8 91 0 M0 85 Q8 86 15 100 M0 91 Q5 92 9 100 M100 85 Q92 86 85 100 M100 91 Q95 92 91 100"
				fill="none"
				stroke="var(--cobalt)"
				stroke-width="2"
			/>
		</svg>{/if}
	{#if state === 'painting'}
		<svg class="sketch" viewBox="0 0 100 100" preserveAspectRatio="none">
			<path d={pattern.stem} />
			<path d={pattern.leaf} />
			<circle cx={pattern.fruitX} cy={pattern.fruitY} r="13" />
			<path d={`M${pattern.fruitX - 7} ${pattern.fruitY - 10} l7 3 7 -3 -7 6z`} />
			{#if pattern.flower}<path d="M75 18 l3 6 7 1 -5 5 1 7 -6 -4 -6 4 1 -7 -5 -5 7 -1z" />{/if}
		</svg>
	{/if}
	<div class="paint" style:clip-path={`inset(${100 - reveal}% 0 0 0)`}>
		<svg viewBox="0 0 100 100" preserveAspectRatio="none">
			<path d={pattern.stem} fill="none" stroke="var(--copper)" stroke-width="3" />
			<path d={pattern.leaf} fill="var(--copper)" stroke="var(--manganese)" stroke-width="1" />
			<circle
				cx={pattern.fruitX}
				cy={pattern.fruitY}
				r="13"
				fill="var(--tomato)"
				stroke="var(--manganese)"
				stroke-width="1"
			/>
			<path
				d={`M${pattern.fruitX - 7} ${pattern.fruitY - 10} l7 3 7 -3 -7 6z`}
				fill="var(--copper)"
			/>
			{#if pattern.flower}<path
					d="M75 18 l3 6 7 1 -5 5 1 7 -6 -4 -6 4 1 -7 -5 -5 7 -1z"
					fill="var(--yolk)"
					stroke="var(--manganese)"
					stroke-width="1"
				/>{/if}
		</svg>
	</div>
	{#if state === 'glazed' && serial}<span class="serial">n° {serial.toLocaleString()}</span>{/if}
</div>

<style>
	.tile {
		position: relative;
		aspect-ratio: 1;
		min-width: 0;
		overflow: hidden;
		background: color-mix(in srgb, var(--manganese) 12%, var(--glaze));
		box-shadow: inset 0 1px 3px rgba(36, 28, 22, 0.18);
	}
	.tile.glazed,
	.tile.painting {
		background: color-mix(in srgb, var(--card) 65%, var(--glaze));
		box-shadow:
			inset 0 1px 2px rgba(255, 255, 255, 0.48),
			inset 0 -2px 3px rgba(30, 45, 64, 0.12);
	}
	.paint,
	svg {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
	}
	.painting .ornament {
		opacity: 0.35;
	}
	.sketch {
		fill: none;
		stroke: var(--manganese);
		stroke-width: 0.8;
		stroke-dasharray: 1 1.5;
		opacity: 0.7;
	}
	.paint {
		transition: clip-path 1s linear;
	}
	.serial {
		position: absolute;
		bottom: 3px;
		right: 4px;
		padding: 0 3px;
		color: var(--foreground);
		background: var(--glaze);
		font-size: 9px;
		line-height: 1.4;
		font-variant-numeric: tabular-nums;
	}
	.animate .paint {
		animation: paint 600ms ease-out both;
	}
	.animate::after {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(
			110deg,
			transparent 30%,
			rgba(255, 255, 255, 0.65) 50%,
			transparent 70%
		);
		transform: translateX(-120%);
		animation: shine 300ms 600ms ease-out;
	}
	@media (prefers-reduced-motion: reduce) {
		.paint {
			transition: none;
		}
		.animate .paint,
		.animate::after {
			animation: none;
		}
	}
	@keyframes paint {
		from {
			clip-path: inset(100% 0 0);
		}
		to {
			clip-path: inset(0);
		}
	}
	@keyframes shine {
		to {
			transform: translateX(120%);
		}
	}
</style>
