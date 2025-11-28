<script lang="ts">
	interface DataPoint {
		label: string;
		value: number;
		color?: string;
		percentage: number;
	}

	let { data, title }: { data: DataPoint[]; title: string } = $props();

	// Calcul des segments du donut
	let segments = $derived.by(() => {
		let cumulativePercent = 0;
		return data.map((item) => {
			const start = cumulativePercent;
			cumulativePercent += item.percentage;
			return {
				...item,
				start,
				end: cumulativePercent
			};
		});
	});

	function getCoordinatesForPercent(percent: number) {
		const x = Math.cos(2 * Math.PI * percent);
		const y = Math.sin(2 * Math.PI * percent);
		return [x, y];
	}
</script>

<div class="bg-card text-card-foreground rounded-lg border p-4 shadow-sm">
	<h3 class="mb-4 text-lg font-semibold">{title}</h3>

	<div class="flex flex-col items-center gap-6 sm:flex-row">
		<!-- Donut SVG -->
		<div class="relative h-48 w-48 shrink-0">
			<svg viewBox="-1 -1 2 2" style="transform: rotate(-90deg)" class="h-full w-full">
				{#each segments as segment}
					<path
						d="M {getCoordinatesForPercent(segment.start / 100)[0]} {getCoordinatesForPercent(
							segment.start / 100
						)[1]} A 1 1 0 {segment.percentage > 50 ? 1 : 0} 1 {getCoordinatesForPercent(
							segment.end / 100
						)[0]} {getCoordinatesForPercent(segment.end / 100)[1]} L 0 0"
						fill={segment.color || 'currentColor'}
						class="transition-opacity hover:opacity-80"
					></path>
				{/each}
				<!-- Trou central pour faire un donut -->
				<circle cx="0" cy="0" r="0.6" class="fill-card"></circle>
			</svg>
		</div>

		<!-- Légende -->
		<div class="flex flex-1 flex-col gap-2">
			{#each data as item}
				<div class="flex items-center justify-between text-sm">
					<div class="flex items-center gap-2">
						<div
							class="h-3 w-3 rounded-full"
							style="background-color: {item.color || 'currentColor'}"
						></div>
						<span class="truncate">{item.label}</span>
					</div>
					<div class="text-muted-foreground font-mono text-xs">
						{Math.round(item.percentage)}% ({item.value}m)
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
