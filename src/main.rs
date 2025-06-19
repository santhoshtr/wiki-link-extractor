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
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut text_content = String::new();

    // Extract the text under the <text> node
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"text" => {
                text_content = reader.read_text(b"text", &mut Vec::new())?;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => (),
        }
        buf.clear();
    }

    // Pass the extracted text to extract_links
    let links = extractor.extract_links(&text_content)?;
    total_links += links.len();
    for link in links.iter() {
        println!("{}\t{}", link.title, link.label.as_deref().unwrap_or(""),);
    }

    Ok(())
}
