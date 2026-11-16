import { test, expect } from '@playwright/test';

test.describe('VeloMotion Clone E2E Tests', () => {
  test('Flow 1: Dashboard to Editor Navigation & Project Creation', async ({ page }) => {
    // Navigate to Dashboard (Page 1)
    await page.goto('/');

    // Verify Dashboard header and recent projects
    await expect(page.locator('h1')).toContainText('VeloMotion Pro Studio');
    await expect(page.locator('text=Recent Video Projects')).toBeVisible();

    // Click 'Create New Video Project'
    const createBtn = page.locator('button', { hasText: 'Create New Video Project' });
    await expect(createBtn).toBeVisible();
    await createBtn.click();

    // Verify navigation to Page 2 (Editor Workspace)
    await expect(page).toHaveURL(/\/editor\/.+/);
    await expect(page.locator('text=Zone A: Canvas Preview')).toBeVisible();
  });

  test('Flow 2: Editor Workspace 5 Zones - Timeline Operations (Split & Ripple Delete)', async ({ page }) => {
    // Navigate directly to editor workspace
    await page.goto('/editor/test-project-123');

    // Verify 5 zones are present
    await expect(page.locator('text=Zone A: Canvas Preview')).toBeVisible();
    await expect(page.locator('text=Zone B: Video Timeline NLE')).toBeVisible();
    await expect(page.locator('text=Zone C: Media Import')).toBeVisible();
    await expect(page.locator('text=Zone D: Editing Toolbar')).toBeVisible();
    await expect(page.locator('text=Zone E: Export Engine')).toBeVisible();

    // Verify initial timeline clips exist
    const clipItem = page.locator('text=sample_footage_01.mp4');
    await expect(clipItem).toBeVisible();

    // Select clip for editing
    await clipItem.click();

    // Test Zone D: Split Clip
    const splitBtn = page.locator('button', { hasText: 'Split Clip' });
    await expect(splitBtn).toBeVisible();
    await splitBtn.click();

    // Verify timeline now has 3 clips after split
    await expect(page.locator('text=Total Clips: 3')).toBeVisible();

    // Test Zone D: Ripple Delete
    const deleteBtn = page.locator('button', { hasText: 'Ripple Delete' });
    await expect(deleteBtn).toBeVisible();
    await deleteBtn.click();

    // Verify timeline count decreases
    await expect(page.locator('text=Total Clips: 2')).toBeVisible();
  });

  test('Flow 3: Cloud Sync & Export Engine', async ({ page }) => {
    await page.goto('/editor/test-project-123');

    // Test Cloud Sync button
    const syncBtn = page.locator('button', { hasText: 'Sync to Cloud (JSON)' });
    await expect(syncBtn).toBeVisible();
    await syncBtn.click();

    // Test Zone E: Export MP4 (FFmpeg.wasm)
    const exportBtn = page.locator('button', { hasText: 'Export MP4 (FFmpeg.wasm)' });
    await expect(exportBtn).toBeVisible();
    await expect(exportBtn).toBeEnabled();
  });
});
