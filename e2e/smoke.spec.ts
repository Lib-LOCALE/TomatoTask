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
	await page.getByText('Weekly fresco').click();
	await expect(page.locator('.week-row')).toHaveCount(7);
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
