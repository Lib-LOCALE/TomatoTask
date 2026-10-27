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

<div class="tile" class:glazed={state === 'glazed'} class:animate aria-hidden="true">
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
		background: var(--glaze);
		box-shadow: inset 0 1px rgba(255, 255, 255, 0.6);
	}
	.paint,
	svg {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
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
