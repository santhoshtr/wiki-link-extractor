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
    let input = std::fs::read_to_string(file_name)?;

    // Pass the entire input to extract_links
    let links = extractor.extract_links(&input)?;
    total_links += links.len();
    for link in links.iter() {
        println!("{}\t{}", link.title, link.label.as_deref().unwrap_or(""),);
    }

    Ok(())
}
