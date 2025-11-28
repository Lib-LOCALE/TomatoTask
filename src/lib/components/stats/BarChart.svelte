<script lang="ts">
	interface DataPoint {
		label: string;
		value: number;
	}

	let {
		data,
		title,
		color = 'bg-primary'
	}: { data: DataPoint[]; title: string; color?: string } = $props();

	let maxValue = $derived(Math.max(...data.map((d) => d.value), 1));
</script>

<div class="bg-card text-card-foreground rounded-lg border p-4 shadow-sm">
	<h3 class="mb-4 text-lg font-semibold">{title}</h3>

	<div class="flex h-48 items-end gap-2">
		{#each data as item}
			<div class="group relative flex h-full flex-1 flex-col justify-end">
				<!-- Tooltip -->
				<div
					class="bg-popover text-popover-foreground absolute -top-8 left-1/2 hidden -translate-x-1/2 rounded px-2 py-1 text-xs shadow-md group-hover:block"
				>
					{item.value} min
				</div>

				<!-- Bar -->
				<div
					class="w-full rounded-t-md transition-all duration-500 {color}"
					style="height: {(item.value / maxValue) * 100}%"
				></div>

				<!-- Label -->
				<div class="text-muted-foreground mt-2 truncate text-center text-xs">
					{item.label}
				</div>
			</div>
		{/each}
	</div>
</div>
