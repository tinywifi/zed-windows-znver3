use std::error::Error;
use std::fs;
use std::io::Cursor;
use std::path::Path;

use zip::read::ZipArchive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let octocrab = octocrab::instance();

    let latest = octocrab
        .repos("MolotovCherry", "zed-windows-builds")
        .releases()
        .get_latest()
        .await?;

    let tag = latest.tag_name;

    println!("Found release {tag}");

    let asset = latest
        .assets
        .get(0)
        .ok_or(format!("No asset found on latest release {tag}"))?;

    println!("Downloading asset {}", asset.name);

    let data = reqwest::get(asset.browser_download_url.clone())
        .await?
        .bytes()
        .await?;

    let path = Path::new(&asset.name);

    let ext = path
        .extension()
        .ok_or("asset has no extension")?
        .to_string_lossy();

    match &*ext {
        "zip" => {
            let cursor = Cursor::new(&data);
            let mut zip = ZipArchive::new(cursor)?;

            zip.extract(".")?;

            for filename in zip.file_names() {
                println!("File: {filename}");
            }
        }

        "exe" => {
            fs::write(&asset.name, data)?;
            println!("File: {}", asset.name);
        }

        _ => Err(format!("extension {ext} is unsupported"))?,
    }

    if let Some(body) = latest.body {
        println!("\n{}", termimad::term_text(&body));
    }

    Ok(())
}
