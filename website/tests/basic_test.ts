import { test, expect } from "@playwright/test";

declare global {
  interface Window {
    getMonacoEditorValue: () => string;
    setMonacoEditorValue: (value: string) => void;
  }
}

test("test basic lang, compiler and executor.", async ({ page }) => {
  await page.goto("/code", { waitUntil: "networkidle" });

  const monacoValue1 = await page.evaluate(() => window.getMonacoEditorValue());
  expect(monacoValue1).toBeTruthy();

  const language = page.locator('[name="language"]');
  await expect(language).toBeVisible();
  await expect(language.locator("option:checked")).toHaveText("Lua");

  const executor = page.locator('[name="executor"]');
  await expect(executor).toBeVisible();
  await executor.selectOption({ value: "lua5.1.5" });
  await expect(executor).toHaveValue("lua5.1.5");
  const monacoValue2 = await page.evaluate(() => window.getMonacoEditorValue());
  expect(monacoValue2).toBeTruthy();

  const compilerSelect = page.locator('[name="compiler"]');
  await expect(compilerSelect).not.toBeVisible();
});

test("test changing lang, should change editor", async ({ page }) => {
  await page.goto("/code", { waitUntil: "networkidle" });

  const monacoValueLua = await page.evaluate(() =>
    window.getMonacoEditorValue(),
  );
  expect(monacoValueLua).toBeTruthy();
  const language = page.locator('[name="language"]');
  await expect(language).toBeVisible();
  await expect(language.locator("option:checked")).toHaveText("Lua");

  await language.selectOption({ value: "python3" });
  await expect(language).toHaveValue("python3");

  const monacoValuePython = await page.evaluate(() =>
    window.getMonacoEditorValue(),
  );

  expect(monacoValuePython).not.toEqual(monacoValueLua);
});
