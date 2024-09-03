use std::error::Error;
use std::fs;
use std::io::Cursor;
use std::path::Path;

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
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
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&body, options);
        let data = parser
            .map(|event| match event {
                Event::Start(t) => match t {
                    Tag::Heading { level, .. } => match level {
                        HeadingLevel::H2 => "## ".to_owned(),
                        _ => unimplemented!(),
                    },
                    Tag::Table(_) => "\n| - | - |".to_owned(),
                    Tag::TableHead => "\n".to_owned(),
                    Tag::TableRow => "\n".to_owned(),
                    Tag::TableCell => "|".to_owned(),
                    Tag::Link { .. } => "".to_owned(),
                    _ => unimplemented!(),
                },
                Event::End(t) => match t {
                    TagEnd::Table => "".to_owned(),
                    TagEnd::TableHead => "|".to_owned(),
                    TagEnd::TableRow => "|".to_owned(),
                    TagEnd::TableCell => "".to_owned(),
                    TagEnd::Heading(_) => "".to_owned(),
                    TagEnd::Link => "".to_owned(),
                    _ => unimplemented!(),
                },
                Event::Text(t) => t.to_string(),

                _ => unimplemented!(),
            })
            .collect::<String>();

        println!("\n{}", termimad::term_text(&data));
        pause();
    }

    Ok(())
}

fn pause() {
    use std::io::Read;
    use std::io::Write;

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
