import { describe, expect, it } from 'vitest';
import { frescoTiles, patternIndex } from '../src/lib/components/majolica/patterns';

describe('fresque du jour', () => {
	it('garde les carreaux faits même au-delà des prévisions', () => {
		expect(frescoTiles(2, 3, 10, false)).toEqual([
			{ state: 'glazed', serial: 8 },
			{ state: 'glazed', serial: 9 },
			{ state: 'glazed', serial: 10 },
			...Array(3).fill({ state: 'empty' })
		]);
	});
	it('réserve le prochain carreau à la session en cours', () => {
		expect(frescoTiles(3, 1, 12, true)).toEqual([
			{ state: 'glazed', serial: 12 },
			{ state: 'painting' },
			{ state: 'planned' },
			...Array(3).fill({ state: 'empty' })
		]);
	});
	it('ajoute un carreau en cours même sans prévision', () => {
		expect(frescoTiles(0, 0, 0, true)).toEqual([
			{ state: 'painting' },
			...Array(5).fill({ state: 'empty' })
		]);
	});
	it('complète une nouvelle rangée sans perdre les carreaux prévus', () => {
		expect(frescoTiles(7, 0, 0, false).map((tile) => tile.state)).toEqual([
			...Array(7).fill('planned'),
			...Array(5).fill('empty')
		]);
	});
	it('répète les motifs selon la position dans la fresque', () => {
		expect([0, 1, 11, 12].map(patternIndex)).toEqual([0, 1, 11, 0]);
	});
});
