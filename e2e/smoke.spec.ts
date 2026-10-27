import { test, expect } from '@playwright/test';

test('créer une tâche, piloter le minuteur et changer le thème', async ({ page }) => {
	await page.goto('/?seed=empty');
	await expect(page.getByRole('button', { name: 'Settings' })).toBeVisible();
	await page.keyboard.press('Control+n');
	await page.getByPlaceholder('Write documentation').fill('Préparer le test');
	await page.locator('form button[type=submit]').click();
	await expect(page.getByText('Préparer le test').first()).toBeVisible();
	await page.getByRole('button', { name: 'Start', exact: true }).click();
	await expect(page.locator('.tile.painting .sketch').first()).toBeVisible();
	await page.getByRole('button', { name: 'Pause', exact: true }).click();
	await expect(page.getByRole('button', { name: 'Resume', exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Switch to dark mode' }).click();
	await expect(page.locator('html')).toHaveClass(/dark/);
	await page.getByRole('button', { name: 'Reset', exact: true }).click();
	await page.getByRole('button', { name: 'Short Break', exact: true }).click();
	await page.getByRole('button', { name: 'Start', exact: true }).click();
	await page.getByRole('button', { name: 'Skip', exact: true }).click();
	await expect(page.getByRole('button', { name: 'Work', exact: true })).toHaveClass(/active/);
});

test('la fresque et les tâches restent accessibles dans le tiroir', async ({ page }) => {
	await page.setViewportSize({ width: 700, height: 800 });
	await page.goto('/');
	await expect(page.getByRole('heading', { name: 'Préparer la réunion', level: 1 })).toBeVisible();
	await expect(page.locator('.rail')).toBeHidden();
	await page.getByRole('button', { name: 'Menu', exact: true }).click();
	await expect(
		page.getByRole('button', { name: 'Mark Préparer la réunion complete' })
	).toBeVisible();
	const task = page.getByRole('button', { name: 'Préparer la réunion', exact: true });
	await expect(task).toBeVisible();
	await task.focus();
	await expect(
		page.getByRole('button', { name: 'Edit : Préparer la réunion' }).locator('..')
	).toHaveCSS('opacity', '1');
	await page.getByRole('button', { name: 'Close', exact: true }).first().click();
	await expect(page.getByRole('heading', { name: 'Weekly fresco' })).toBeVisible();
	await expect(page.locator('.week-labels span')).toHaveCount(7);
	await expect(page.locator('.week-wall')).toBeVisible();
});

test('planifier deux tâches, ajuster et commencer la journée', async ({ page }) => {
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
	await page.getByRole('button', { name: 'Increase estimate : Préparer la réunion' }).click();
	await expect(page.locator('.summary')).toContainText('6 Planned');
	await page.getByRole('button', { name: 'Start the day' }).click();
	await expect(page.getByRole('heading', { name: 'Préparer la réunion', level: 1 })).toBeVisible();
});

test('créer un projet dans le rail et fermer son édition au clavier', async ({ page }) => {
	await page.goto('/');
	const add = page.getByRole('button', { name: 'New Project' });
	await add.focus();
	await page.keyboard.press('Enter');
	await page.getByRole('textbox', { name: 'Project Name' }).fill('Atelier');
	await page.getByRole('button', { name: 'Create Project' }).click();
	const row = page.locator('.projects').getByRole('group').filter({ hasText: 'Atelier' });
	await expect(row).toBeVisible();
	await row.getByRole('button', { name: 'Edit' }).focus();
	await page.keyboard.press('Enter');
	await expect(page.getByRole('dialog')).toBeVisible();
	await page.keyboard.press('Escape');
	await expect(page.getByRole('dialog')).toBeHidden();
});

test('le bilan change de période et présente un état vide', async ({ page }) => {
	await page.goto('/');
	await page.getByRole('button', { name: 'Review', exact: true }).click();
	await page.getByRole('button', { name: '30 days' }).click();
	await expect(page.getByText(/pomodoros in 30 days/)).toBeVisible();
	await page.getByRole('button', { name: '90 days' }).click();
	await expect(page.getByText(/pomodoros in 90 days/)).toBeVisible();
	await page.goto('/?seed=empty');
	await page.getByRole('button', { name: 'Review', exact: true }).click();
	await expect(page.getByRole('heading', { name: 'Your first tile is waiting' })).toBeVisible();
});

test('onboarding et aide clavier accessibles', async ({ page }) => {
	await page.goto('/?onboarding=1&seed=empty');
	await expect(page.getByRole('heading', { name: 'A session becomes a tile' })).toBeVisible();
	await page.getByRole('button', { name: 'Next' }).click();
	await page.getByRole('button', { name: 'Next' }).click();
	await page.getByLabel('Task name').fill('Premier carreau');
	await page.getByRole('button', { name: 'Create task' }).click();
	await expect(page.getByText('Premier carreau').first()).toBeVisible();
	await page.keyboard.press('Control+/');
	await expect(page.getByRole('dialog', { name: 'Keyboard Shortcuts' })).toBeVisible();
	await page.keyboard.press('Escape');
	await expect(page.getByRole('dialog', { name: 'Keyboard Shortcuts' })).toBeHidden();
});

test('le thème système suit les changements de préférence', async ({ page }) => {
	await page.emulateMedia({ colorScheme: 'dark' });
	await page.goto('/');
	await page.getByRole('button', { name: 'Settings' }).click();
	await page.getByLabel('Theme', { exact: true }).selectOption('system');
	await page.getByRole('button', { name: 'Save', exact: true }).click();
	await expect(page.locator('html')).toHaveClass(/dark/);
	await page.emulateMedia({ colorScheme: 'light' });
	await expect(page.locator('html')).not.toHaveClass(/dark/);
});

test('le sélecteur de langue est modal et se ferme avec Échap', async ({ page }) => {
	await page.goto('/');
	await page.getByRole('button', { name: 'Settings' }).focus();
	await page.keyboard.press('Control+l');
	const dialog = page.locator('dialog:modal');
	await expect(dialog).toBeVisible();
	await page.keyboard.press('Escape');
	await expect(dialog).toHaveCount(0);
	await expect(page.getByRole('button', { name: 'Settings' })).toBeFocused();
});

test('le mini-mode affiche les commandes dans une fenêtre de 300 par 150', async ({ page }) => {
	await page.goto('/');
	await page.getByRole('button', { name: 'Mini mode' }).click();
	await page.setViewportSize({ width: 300, height: 150 });
	await expect(page.getByText('25:00')).toBeVisible();
	await expect(page.getByRole('button', { name: 'Start', exact: true })).toBeVisible();
	await expect(page.getByRole('button', { name: 'Return to full window' })).toBeVisible();
});
