use std::fs;

use extractor::LinkExtractor;
mod extractor;

#[derive(Debug, Clone)]
pub struct MarkdownLink {
    pub label: String,
    pub url: String,
    pub title: Option<String>,
    pub start_byte: usize,
    pub end_byte: usize,
}

// Example usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut extractor = LinkExtractor::new()?;
    let mut total_links = 0;

    // Read the file and pass content to extract_links. No need to read from stdin.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }
    let file_name = &args[1];
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_file(file_name)?;
    reader.config_mut().trim_text(true);
    reader.config_mut().allow_unmatched_ends = true;
    reader.config_mut().check_end_names = false;
    let mut buf = Vec::new();
    let mut text_content = String::new();
    let mut id_content = String::new();
    let mut current_tag: Option<String> = None;
    // Extract the text under the <text> node
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                current_tag = Some(
                    e.name()
                        .into_inner()
                        .to_vec()
                        .into_iter()
                        .map(|c| c as char)
                        .collect(),
                );
            }
            Ok(Event::Text(e)) => {
                if let Some(tag) = &current_tag {
                    match tag.as_str() {
                        "text" => {
                            text_content = e.unescape().unwrap().into_owned();
                        }
                        "id" => {
                            id_content = e.unescape().unwrap().into_owned();
                        }
                        _ => (),
                    }
                }
            }
            Ok(Event::End(e)) => {
                if let Some(tag) = &current_tag {
                    match tag.as_str() {
                        "text" => {
                            // dbg!(&id_content);
                            // let file_path = format!("data/{}.wikitext", id_content);
                            text_content.push('\n');
                            // fs::write(&file_path, &text_content)?;
                            // let file_content = fs::read_to_string(&file_path)?;
                            // Add error handling for extract_links. Create new extractor when error
                            // happens and continue.
                            let links = match extractor.extract_links(&text_content) {
                                Ok(links) => links,
                                Err(_) => {
                                    extractor = LinkExtractor::new()?;
                                    continue;
                                }
                            };
                            total_links += links.len();

                            let mut tsv_file = fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open("links.tsv")?;
                            for link in links.iter() {
                                writeln!(
                                    tsv_file,
                                    "{}\t{}",
                                    link.title,
                                    link.label.as_deref().unwrap_or(&link.title),
                                )?;
                            }
                            }
                            current_tag = None;
                            text_content.clear();
                            id_content.clear();
                        }
                        "id" => {}
                        _ => (),
                    }
                }
            }
            // There are several other `Event`s we do not consider here
            _ => (),
        }
    }

    Ok(())
}
