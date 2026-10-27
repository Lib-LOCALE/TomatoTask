import { describe, expect, it } from 'vitest';
import { frescoSerials } from '$lib/services/stats-service';

describe('numéros de la fresque', () => {
	it('conserve les numéros à vie à travers les jours vides', () => {
		expect(frescoSerials([2, 0, 1], 12)).toEqual([[10, 11], [], [12]]);
	});
});
