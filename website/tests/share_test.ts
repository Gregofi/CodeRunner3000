import { test, expect } from "@playwright/test";

test("test generating share link and accessing it", async ({ page }) => {
  await page.goto("/code", { waitUntil: "networkidle" });

  // switch executor
  const executor = page.locator('[name="executor"]');
  await expect(executor).toBeVisible();
  await executor.selectOption({ value: "lua5.1.5" });

  // edit the code a bit
  await page.evaluate(() =>
    window.setMonacoEditorValue('print("hello from lua5.1.5")'),
  );

  const triggerShareBtn = page.locator('[name="share-dialog-btn"]');
  const shareDialog = page.locator('[data-pw="share-dialog"]');
  const closeButton = page.locator('[name="share-dialog-close-btn"]');

  await expect(shareDialog).not.toBeVisible();

  await triggerShareBtn.click();
  await expect(shareDialog).toBeVisible();

  const link = await page
    .locator('[name="share-dialog-link-input"]')
    .inputValue();
  expect(link).toBeTruthy();

  await closeButton.click();
  await expect(shareDialog).not.toBeVisible();

  // change values because the previous code will be in local storage
  const language = page.locator('[name="language"]');
  await expect(language).toBeVisible();
  await language.selectOption({ value: "python3" });
  await page.evaluate(() =>
    window.setMonacoEditorValue('print("hello from python3")'),
  );

  // go to the share link
  await page.goto(link, { waitUntil: "networkidle" });
  const editorValueAfter = await page.evaluate(() =>
    window.getMonacoEditorValue(),
  );
  expect(editorValueAfter).toEqual('print("hello from lua5.1.5")');

  const languageAfter = page.locator('[name="language"]');
  await expect(languageAfter).toBeVisible();
  await expect(languageAfter.locator("option:checked")).toHaveText("Lua");

  const executorAfter = page.locator('[name="executor"]');
  await expect(executorAfter).toBeVisible();
  await expect(executorAfter).toHaveValue("lua5.1.5");
});
