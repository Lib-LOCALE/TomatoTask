export type TileState = 'empty' | 'planned' | 'painting' | 'glazed';
export type FrescoTile = { state: TileState; serial?: number };

export function patternIndex(position: number): number {
	return ((position % 12) + 12) % 12;
}

export function frescoTiles(
	planned: number,
	completed: number,
	total: number,
	painting: boolean
): FrescoTile[] {
	const done = Math.max(0, Math.floor(completed));
	const count = Math.max(0, Math.floor(planned), done + (painting ? 1 : 0));
	return Array.from({ length: count }, (_, index) => {
		if (index < done) return { state: 'glazed', serial: Math.max(0, total - done) + index + 1 };
		return { state: painting && index === done ? 'painting' : 'planned' };
	});
}

// Les courbes rejoignent toujours le milieu des quatre bords, même entre deux motifs différents.
const bends = [-18, 12, -9, 20, 4, -15, 16, -5, 10, -20, 7, 18];
const leaves = [
	'M50 50 Q18 18 15 37 Q24 52 50 50',
	'M50 50 Q82 18 85 37 Q76 52 50 50',
	'M50 50 Q16 45 20 24 Q37 23 50 50',
	'M50 50 Q84 45 80 24 Q63 23 50 50'
];
export const patterns = bends.map((bend, i) => ({
	stem: `M0 50 C25 ${50 + bend} 30 ${50 - bend} 50 50 S75 ${50 + bend} 100 50 M50 0 C${50 + bend} 25 ${50 - bend} 30 50 50 S${50 + bend} 75 50 100`,
	leaf: leaves[i % leaves.length],
	fruitX: [30, 70, 25, 70, 30, 72, 27, 70, 30, 72, 25, 70][i],
	fruitY: [70, 70, 72, 72, 68, 68, 70, 70, 72, 72, 68, 68][i],
	flower: i % 3 === 0
}));
