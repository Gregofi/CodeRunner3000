/*
import { test, expect } from '@playwright/test';

test('test generating share link and accessing it', async ({ page }) => {
	await page.goto('/code', { waitUntil: 'networkidle' });

	// switch executor
	const executor = page.locator('[name="executor"]');
	await expect(executor).toBeVisible();
	await executor.selectOption({ value: 'lua5.1.5' });

	// edit the code a bit
	await page.evaluate(() => window.setMonacoEditorValue('print("hello")'));

	const modalButton = page.locator('[name="share-modal-btn"]');
	await modalButton.click();

	const shareLink = await page.locator('[name="share-input"]').inputValue();
	expect(shareLink).toBeTruthy();

	// close modal
	const closeButton = page.locator('[name="share-modal-close-btn"]');
	await closeButton.click();

	// TODO: probably check the whole div?
	await expect(page.locator('[name="share-input"]')).not.toBeVisible();

	const editorValueBefore = await page.evaluate(() => window.getMonacoEditorValue());

	// change values so that the share link changes

	const language = page.locator('[name="language"]');
	await expect(language).toBeVisible();
	await language.selectOption({ value: 'python3' });

	const editorValuePython = await page.evaluate(() => window.getMonacoEditorValue());
	expect(editorValuePython).not.toEqual(editorValueBefore);

	// go to the share link
	await page.goto(shareLink!, { waitUntil: 'networkidle' });
	const editorValueAfter = await page.evaluate(() => window.getMonacoEditorValue());
	expect(editorValueBefore).toEqual(editorValueAfter);

	const languageAfter = page.locator('[name="language"]');
	await expect(languageAfter).toBeVisible();
	await expect(languageAfter.locator('option:checked')).toHaveText('Lua');

	const executorAfter = page.locator('[name="executor"]');
	await expect(executorAfter).toBeVisible();
	await expect(executorAfter).toHaveValue('lua5.1.5');

	const codeAfter = await page.evaluate(() => window.getMonacoEditorValue());
	expect(codeAfter).toEqual('print("hello")');
});
*/
