from playwright.sync_api import sync_playwright, expect
import time

def run(playwright):
    browser = playwright.chromium.launch(headless=True)
    context = browser.new_context()
    page = context.new_page()

    try:
        page.goto("http://localhost:1420")
    except Exception as e:
        print("Could not connect to the development server. Please ensure it is running and accessible.")
        print(f"Error: {e}")
        browser.close()
        return

    # Give the app time to load
    time.sleep(5)

    # Check for the main heading
    expect(page.get_by_role("button", name="Library")).to_be_visible()

    # Check for the settings button
    expect(page.get_by_role("button", name="Settings")).to_be_visible()

    # Check for the SyncQueue component
    expect(page.get_by_text("Pending operations:")).to_be_visible()

    # Check for the status bar
    expect(page.get_by_text("Loading status...")).to_be_visible()

    page.screenshot(path="jules-scratch/verification/verification.png")

    browser.close()

with sync_playwright() as playwright:
    run(playwright)