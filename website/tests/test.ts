import { test } from '@playwright/test';

test('index page should redirect', async ({ page }) => {
	await page.goto('/');
});
