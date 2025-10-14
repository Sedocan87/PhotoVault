import time
from playwright.sync_api import sync_playwright, expect

def run(playwright):
    # Note: The Tauri app needs to be running for this script to work.
    # The script connects to the running Tauri application window.
    # This is a simplified example; a real script might need to handle window discovery.

    time.sleep(10) # Wait for the server to start

    # This is a placeholder and may not work in all environments.
    # A more robust solution would involve finding the correct window handle.
    try:
        app = playwright.chromium.connect_over_cdp("http://localhost:1420")
        page = app.contexts[0].pages[0]
    except:
        # Fallback for when CDP is not available
        browser = playwright.chromium.launch()
        page = browser.new_page()
        page.goto("http://localhost:1420")


    # Wait for the SyncQueue component to be visible
    sync_queue_component = page.locator('div:has-text("Sync Status")')
    expect(sync_queue_component).to_be_visible()

    # Take a screenshot
    page.screenshot(path="jules-scratch/verification/verification.png")

    if 'browser' in locals():
        browser.close()

with sync_playwright() as playwright:
    run(playwright)