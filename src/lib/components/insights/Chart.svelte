<script lang="ts">
	interface Point {
		label: string;
		value: number;
	}
	let { title, points, unit }: { title: string; points: Point[]; unit: string } = $props();
	let selected = $state<Point | null>(null);
	const max = $derived(Math.max(1, ...points.map((p) => p.value)));
	const width = $derived(Math.max(420, points.length * 18));
	const step = $derived(width / Math.max(1, points.length));
</script>

<section class="chart">
	<h3>{title}</h3>
	<div class="scroll">
		<svg
			viewBox={`0 0 ${width + 32} 150`}
			role="group"
			aria-label={title}
			style={`min-width:${width + 32}px`}
		>
			<line x1="30" y1="120" x2={width + 30} y2="120" stroke="var(--grout)" />
			<text x="2" y="20">{max}</text><text x="12" y="123">0</text>
			{#each points as point, i}
				<g
					role="img"
					tabindex="0"
					onfocus={() => (selected = point)}
					onblur={() => (selected = null)}
					onmouseenter={() => (selected = point)}
					onmouseleave={() => (selected = null)}
					aria-label={`${point.label}: ${point.value} ${unit}`}
				>
					<title>{point.label}: {point.value} {unit}</title>
					<rect
						x={32 + i * step}
						y={120 - (point.value / max) * 100}
						width={Math.max(3, step - 3)}
						height={Math.max(1, (point.value / max) * 100)}
						rx="1"
					/>
					{#if points.length <= 7 || i % Math.ceil(points.length / 8) === 0}<text
							x={32 + i * step}
							y="140">{point.label}</text
						>{/if}
				</g>
			{/each}
		</svg>
	</div>
	<p class="value" aria-live="polite">
		{#if selected}{selected.label}: {selected.value} {unit}{:else}&nbsp;{/if}
	</p>
</section>

<style>
	.chart {
		min-width: 0;
	}
	h3 {
		font-family: var(--font-body);
		font-size: 1rem;
		font-weight: 700;
		margin-bottom: 12px;
	}
	.value {
		min-height: 1.5em;
		color: var(--muted-foreground);
		font-size: 0.875rem;
	}
	.scroll {
		overflow-x: auto;
	}
	svg {
		width: 100%;
		height: 150px;
		overflow: visible;
	}
	text {
		font-size: 10px;
		fill: var(--muted-foreground);
	}
	rect {
		fill: var(--cobalt);
	}
	g:hover rect,
	g:focus rect {
		fill: var(--tomato);
	}
	g:focus {
		outline: 2px solid var(--ring);
		outline-offset: 2px;
	}
</style>
