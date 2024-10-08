use std::path::Path;
use std::{env, fs};

use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, LaunchOptions};

pub fn render_html(
    path: impl AsRef<Path>,
    target: impl AsRef<Path>,
    target_el: impl AsRef<str>,
    wait_el: impl AsRef<str>,
) -> anyhow::Result<()> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .window_size(Some((3000, 5000)))
            .sandbox(false)
            .devtools(false)
            .build()?,
    )?;

    let tab = browser.new_tab()?;

    let url = format!("file://{}", env::current_dir()?.join(path).display());
    tab.navigate_to(&url)?;

    tab.wait_for_element(wait_el.as_ref())?;
    let body = tab.wait_for_element(target_el.as_ref())?;

    let png_data = body.capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;

    fs::write(target, png_data)?;

    Ok(())
}

#[test]
fn test_render_html() -> anyhow::Result<()> {
    fs::write(
        "./test.output.html",
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    Hello!
</body>
</html>"#,
    )?;
    render_html(
        "./test.output.html",
        "./test.html.output.png",
        "html",
        "body",
    )?;

    Ok(())
}
