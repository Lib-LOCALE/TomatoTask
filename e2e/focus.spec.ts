import { test, expect } from '@playwright/test';

test('ouvrir le mode focus, démarrer, mettre en pause avec Espace et quitter avec Échap', async ({
	page
}) => {
	await page.goto('/?seed=empty');

	// Ouvre le mode focus via le bouton de l'en-tête
	const focusButton = page.getByRole('button', { name: 'Focus mode' });
	await expect(focusButton).toBeVisible();
	await focusButton.click();

	// Vérifie la présence du dialogue accessible
	const dialog = page.getByRole('dialog', { name: 'Focus mode' });
	await expect(dialog).toBeVisible();

	// Vérifie le bouton principal et son focus initial
	const startButton = page.getByRole('button', { name: 'Start' });
	await expect(startButton).toBeVisible();
	await expect(startButton).toBeFocused();

	// Démarre la session
	await startButton.click();
	await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();

	// Met en pause avec Espace
	await page.keyboard.press('Space');
	await expect(page.getByRole('button', { name: 'Resume' })).toBeVisible();

	// Reprend avec Espace
	await page.keyboard.press('Space');
	await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();

	// Quitte avec Échap
	await page.screenshot({ path: 'screenshots/focus-travail.png' });
	await page.keyboard.press('Escape');
	await expect(dialog).toBeHidden();
});

test('complétion d’une session en mode focus : carreau émaillé et passage en pause', async ({
	page
}) => {
	await page.goto('/?seed=empty');

	// Ouvre le mode focus
	await page.getByRole('button', { name: 'Focus mode' }).click();
	const dialog = page.getByRole('dialog', { name: 'Focus mode' });
	await expect(dialog).toBeVisible();

	// Démarre la session
	await page.getByRole('button', { name: 'Start' }).click();
	await expect(page.getByRole('button', { name: 'Pause' })).toBeVisible();

	// Fait progresser la session à mi-parcours pour capturer la peinture en cours
	await page.evaluate(async () => {
		const path = '/src/lib/stores/timer.svelte.ts';
		const { timerStore } = await import(/* @vite-ignore */ path);
		timerStore.remainingSeconds = Math.floor(timerStore.totalSeconds / 2);
	});
	await page.waitForTimeout(500);
	await page.screenshot({ path: 'screenshots/focus-en-cours.png' });

	// Force la fin imminente de la session
	await page.evaluate(async () => {
		const path = '/src/lib/stores/timer.svelte.ts';
		const { timerStore } = await import(/* @vite-ignore */ path);
		timerStore.remainingSeconds = 1;
	});

	// Message « Carreau émaillé » / « Tile glazed »
	await expect(page.getByRole('status')).toBeVisible({ timeout: 5000 });
	await expect(page.getByRole('status')).toContainText(/Tile glazed|Carreau émaillé/);
	await page.screenshot({ path: 'screenshots/focus-carreau-emaille.png' });

	// Attendre le passage automatique en pause
	await page.waitForTimeout(3200);
	await page.screenshot({ path: 'screenshots/focus-pause-jaune.png' });

	// Quitte avec Échap
	await page.keyboard.press('Escape');
	await expect(dialog).toBeHidden();
});

test('mode focus avec tâche existante et bouton Passer', async ({ page }) => {
	await page.goto('/');
	await expect(page.getByRole('button', { name: 'Settings' })).toBeVisible();

	// Ouvre le mode focus
	await page.getByRole('button', { name: 'Focus mode' }).click();
	const dialog = page.getByRole('dialog', { name: 'Focus mode' });
	await expect(dialog).toBeVisible();

	// Vérifie que l'en-tête de tâche est présent
	await expect(page.getByRole('heading', { level: 1 })).toBeVisible();

	// Teste le bouton Passer (Skip)
	await page.getByRole('button', { name: 'Skip' }).click();
	await expect(page.getByText(/Short break|Pause courte/i)).toBeVisible();

	// Quitte avec le bouton Quitter
	await page.getByRole('button', { name: 'Exit focus mode' }).click();
	await expect(dialog).toBeHidden();
});
