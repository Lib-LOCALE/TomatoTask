<script lang="ts">
	interface Point {
		label: string;
		value: number;
		tiles?: number;
	}
	let { title, points, unit }: { title: string; points: Point[]; unit: string } = $props();
	let selected = $state<Point | null>(null);
	const max = $derived(
		Math.max(points.some((p) => p.tiles !== undefined) ? 1 : 5, ...points.map((p) => p.value))
	);
	let containerWidth = $state(0);
	const width = $derived(Math.max(containerWidth - 64, points.length * 18, 1));
	const step = $derived(width / Math.max(1, points.length));
</script>

<section class="chart">
	<h3>{title}</h3>
	<div class="scroll" bind:clientWidth={containerWidth}>
		<svg
			viewBox={`0 0 ${width + 64} 150`}
			role="group"
			aria-label={title}
			style={`min-width:${width + 64}px`}
		>
			<line x1="62" y1="120" x2={width + 62} y2="120" stroke="var(--grout)" />
			<text x="2" y="20">{max} {unit}</text><text x="2" y="123">0</text>
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
					{#each Array(point.tiles ?? point.value) as _, tile}
						<rect
							x={64 +
								i * step +
								Math.max(
									0,
									(step -
										Math.min(
											step - 3,
											(point.value / Math.max(1, point.tiles ?? point.value) / max) * 100
										)) /
										2
								)}
							y={120 -
								(((tile + 1) * point.value) / Math.max(1, point.tiles ?? point.value) / max) * 100}
							width={Math.max(
								3,
								Math.min(
									step - 3,
									(point.value / Math.max(1, point.tiles ?? point.value) / max) * 100
								)
							)}
							height={Math.max(
								1,
								(point.value / Math.max(1, point.tiles ?? point.value) / max) * 100 - 2
							)}
						/>
					{/each}
					{#if points.length <= 7 || i % Math.ceil(points.length / 8) === 0}<text
							x={64 +
								i * step +
								Math.max(
									0,
									(step -
										Math.min(
											step - 3,
											(point.value / Math.max(1, point.tiles ?? point.value) / max) * 100
										)) /
										2
								)}
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
