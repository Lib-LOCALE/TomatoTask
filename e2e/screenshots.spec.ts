import { test } from '@playwright/test';
import { mkdirSync } from 'node:fs';

test('captures des vues principales', async ({ page }) => {
	mkdirSync('screenshots', { recursive: true });
	await page.goto('/');
	await page.getByRole('button', { name: 'Settings' }).waitFor();
	await page.screenshot({ path: 'screenshots/principal-clair.png' });
	await page.getByRole('button', { name: 'Switch to dark mode' }).click();
	await page.screenshot({ path: 'screenshots/principal-sombre.png' });
	await page.keyboard.press('Control+n');
	await page.getByPlaceholder('Write documentation').waitFor();
	await page.screenshot({ path: 'screenshots/tache.png' });
	await page.keyboard.press('Escape');
	await page.getByRole('button', { name: 'Settings' }).click();
	await page.screenshot({ path: 'screenshots/parametres.png' });
	await page.setViewportSize({ width: 900, height: 800 });
	await page.goto('/');
	await page.getByRole('button', { name: 'Settings' }).waitFor();
	await page.screenshot({ path: 'screenshots/largeur-900.png' });
	await page.setViewportSize({ width: 1280, height: 800 });
	await page.goto('/?onboarding=1');
	await page.getByText('Choose your language / Choisissez votre langue').waitFor();
	await page.screenshot({ path: 'screenshots/onboarding.png' });
});
