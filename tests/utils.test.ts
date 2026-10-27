import { describe, expect, it, vi } from 'vitest';
import {
	calculateProgress,
	formatMinutes,
	formatSecondsToMMSS
} from '../src/lib/utils/time-formatter';
import {
	sanitizeString,
	validateColorHex,
	validatePomodoroCount,
	validateTaskTitle,
	validateTimerDuration
} from '../src/lib/utils/validators';
import {
	cleanupKeyboardShortcuts,
	initKeyboardShortcuts,
	registerShortcut
} from '../src/lib/utils/keyboard';

describe('utilitaires', () => {
	it('formate les durées et la progression', () => {
		expect(formatSecondsToMMSS(65)).toBe('01:05');
		expect(formatMinutes(90)).toBe('1h 30min');
		expect(calculateProgress(1, 4)).toBe(25);
		expect(calculateProgress(0, 0)).toBe(0);
	});
	it('valide les entrées', () => {
		expect(validateTaskTitle('  ')).not.toBe(true);
		expect(validateTimerDuration(1)).toBe(true);
		expect(validateTimerDuration(1.5)).not.toBe(true);
		expect(validatePomodoroCount(21)).not.toBe(true);
		expect(validateColorHex('#aabbcc')).toBe(true);
		expect(sanitizeString(' a   b ')).toBe('a b');
	});
	it('ignore les raccourcis dans un champ texte', () => {
		const handler = vi.fn();
		initKeyboardShortcuts();
		registerShortcut('n', handler);
		document.body.dispatchEvent(
			new KeyboardEvent('keydown', { key: 'n', ctrlKey: true, bubbles: true })
		);
		const input = document.createElement('input');
		document.body.append(input);
		input.dispatchEvent(new KeyboardEvent('keydown', { key: 'n', ctrlKey: true, bubbles: true }));
		expect(handler).toHaveBeenCalledTimes(1);
		input.remove();
		cleanupKeyboardShortcuts();
	});
});
