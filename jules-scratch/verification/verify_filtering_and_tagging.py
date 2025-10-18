from playwright.sync_api import sync_playwright, expect

def run_verification():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        page.goto("http://localhost:1420")

        # Wait for the library to load
        expect(page.locator("text=Library")).to_be_visible()

        # Click on the first photo
        first_photo = page.locator(".gallery-item").first
        first_photo.click()

        # Add a tag
        page.locator("input[placeholder='Add a tag...']").fill("test-tag")
        page.locator("button:text('Add')").click()

        # Go back to the library
        page.locator("button:text('Library')").click()

        # Filter by the new tag
        page.locator("input[placeholder='Search by filename, tag, or album...']").fill("test-tag")
        page.locator("button:text('Search')").click()

        # Expect to see one photo
        expect(page.locator(".gallery-item")).to_have_count(1)

        # Take a screenshot
        page.screenshot(path="jules-scratch/verification/verification.png")

        browser.close()

if __name__ == "__main__":
    run_verification()