import { test, expect } from '@playwright/test';

test('créer une tâche, piloter le minuteur et changer le thème', async ({ page }) => {
	await page.goto('/?seed=empty');
	await expect(page.getByRole('button', { name: 'Settings' })).toBeVisible();
	await page.keyboard.press('Control+n');
	await page.getByPlaceholder('Write documentation').fill('Préparer le test');
	await page.locator('form button[type=submit]').click();
	await expect(page.getByText('Préparer le test').first()).toBeVisible();
	await page.getByRole('button', { name: 'Start', exact: true }).click();
	await page.getByRole('button', { name: 'Pause', exact: true }).click();
	await expect(page.getByRole('button', { name: 'Resume', exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Switch to dark mode' }).click();
	await expect(page.locator('html')).toHaveClass(/dark/);
});
