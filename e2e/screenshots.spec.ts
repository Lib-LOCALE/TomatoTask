import { test } from '@playwright/test';
import { mkdirSync } from 'node:fs';

test('captures des vues principales', async ({ page }) => {
	mkdirSync('screenshots', { recursive: true });
	await page.goto('/');
	await page.getByRole('button', { name: 'Settings' }).waitFor();
	await page.screenshot({ path: 'screenshots/principal-clair.png' });
	await page.getByRole('button', { name: 'Start', exact: true }).click();
	await page.evaluate(async () => {
		const path = '/src/lib/stores/timer.svelte.ts';
		const { timerStore } = await import(/* @vite-ignore */ path);
		timerStore.remainingSeconds = Math.floor(timerStore.totalSeconds / 2);
	});
	await page.waitForTimeout(1100);
	await page.screenshot({ path: 'screenshots/session-en-cours.png' });
	await page.evaluate(async () => {
		const path = '/src/lib/stores/timer.svelte.ts';
		const { timerStore } = await import(/* @vite-ignore */ path);
		timerStore.remainingSeconds = 1;
	});
	await page.getByText('5 of 5').waitFor();
	await page.waitForTimeout(1000);
	await page.screenshot({ path: 'screenshots/fresque-peinte.png' });
	await page.getByRole('button', { name: 'Switch to dark mode' }).click();
	await page.waitForTimeout(250);
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
	await page.setViewportSize({ width: 700, height: 800 });
	await page.waitForTimeout(300);
	await page.screenshot({ path: 'screenshots/mobile.png' });
	await page.getByRole('button', { name: 'Menu', exact: true }).click();
	await page.waitForTimeout(300);
	await page.screenshot({ path: 'screenshots/mobile-tiroir.png' });
	await page.setViewportSize({ width: 1280, height: 800 });
	await page.goto('/?onboarding=1');
	await page.getByRole('heading', { name: 'A session becomes a tile' }).waitFor();
	await page.screenshot({ path: 'screenshots/onboarding.png' });
});

test('captures de la planification', async ({ page }) => {
	mkdirSync('screenshots', { recursive: true });
	await page.goto('/');
	await page.getByRole('button', { name: 'Plan', exact: true }).click();
	await page
		.locator('.plan-view')
		.getByRole('button', { name: 'Plan today : Préparer la réunion' })
		.click();
	await page
		.locator('.plan-view')
		.getByRole('button', { name: 'Plan today : Répondre aux messages' })
		.click();
	await page.screenshot({ path: 'screenshots/plan-clair.png' });
	await page.getByRole('button', { name: 'Switch to dark mode' }).click();
	await page.screenshot({ path: 'screenshots/plan-sombre.png' });
	await page.setViewportSize({ width: 900, height: 800 });
	await page.screenshot({ path: 'screenshots/plan-900.png' });
});

test('captures du bilan', async ({ page }) => {
	mkdirSync('screenshots', { recursive: true });
	await page.goto('/');
	await page.getByRole('button', { name: 'Review', exact: true }).click();
	await page.getByRole('heading', { name: 'Your fresco' }).waitFor();
	await page.screenshot({ path: 'screenshots/bilan-clair.png', fullPage: true });
	await page.getByRole('heading', { name: 'Your fresco' }).scrollIntoViewIfNeeded();
	await page.screenshot({ path: 'screenshots/bilan-fresque-clair.png' });
	await page.locator('.workspace-scroll').evaluate((element) => element.scrollTo(0, 0));
	await page.getByRole('button', { name: 'Switch to dark mode' }).click();
	await page.screenshot({ path: 'screenshots/bilan-sombre.png', fullPage: true });
	await page.getByRole('heading', { name: 'Your fresco' }).scrollIntoViewIfNeeded();
	await page.screenshot({ path: 'screenshots/bilan-fresque-sombre.png' });
	await page.locator('.workspace-scroll').evaluate((element) => element.scrollTo(0, 0));
	await page.setViewportSize({ width: 900, height: 800 });
	await page.screenshot({ path: 'screenshots/bilan-900.png', fullPage: true });
	await page.getByRole('heading', { name: 'Your fresco' }).scrollIntoViewIfNeeded();
	await page.screenshot({ path: 'screenshots/bilan-fresque-900.png' });
});

test('captures chrome Majolique', async ({ page }) => {
	mkdirSync('screenshots', { recursive: true });
	await page.goto('/');
	await page.getByRole('button', { name: 'Settings' }).click();
	await page.screenshot({ path: 'screenshots/chrome-parametres-clair.png' });
	await page.getByRole('button', { name: 'Close', exact: true }).last().click();
	await page.getByRole('button', { name: 'Switch to dark mode' }).click();
	await page.getByRole('button', { name: 'Settings' }).click();
	await page.screenshot({ path: 'screenshots/chrome-parametres-sombre.png' });
	await page.setViewportSize({ width: 900, height: 800 });
	await page.screenshot({ path: 'screenshots/chrome-parametres-900.png' });
});

test('captures des fresques et du focus — correctifs A', async ({ page }) => {
	mkdirSync('screenshots', { recursive: true });
	await page.goto('/');
	await page.getByRole('button', { name: 'Focus mode' }).click();
	await page.getByRole('dialog', { name: 'Focus mode' }).waitFor();
	await page.screenshot({ path: 'screenshots/focus-zero.png' });
	await page.getByRole('button', { name: 'Start', exact: true }).click();
	await page.evaluate(async () => {
		const path = '/src/lib/stores/timer.svelte.ts';
		const { timerStore } = await import(/* @vite-ignore */ path);
		timerStore.remainingSeconds = Math.floor(timerStore.totalSeconds / 2);
	});
	await page.waitForTimeout(1100);
	await page.screenshot({ path: 'screenshots/focus-progression.png' });
});

test('captures des correctifs B', async ({ page }) => {
	mkdirSync('screenshots', { recursive: true });
	await page.goto('/');
	await page.screenshot({ path: 'screenshots/correctifs-b-minuteur.png' });
	await page.getByRole('button', { name: 'Plan', exact: true }).click();
	await page
		.locator('.plan-view')
		.getByRole('button', { name: 'Plan today : Préparer la réunion' })
		.click();
	await page
		.locator('.plan-view')
		.getByRole('button', { name: 'Plan today : Répondre aux messages' })
		.click();
	await page.screenshot({ path: 'screenshots/correctifs-b-clair.png' });
	await page.getByRole('button', { name: 'Switch to dark mode' }).click();
	await page.waitForTimeout(300);
	await page.screenshot({ path: 'screenshots/correctifs-b-sombre.png' });
	await page.setViewportSize({ width: 900, height: 800 });
	await page.screenshot({ path: 'screenshots/correctifs-b-900.png' });
	await page.keyboard.press('Control+n');
	await page.getByPlaceholder('Write documentation').waitFor();
	await page.screenshot({ path: 'screenshots/correctifs-b-formulaire.png' });
});
