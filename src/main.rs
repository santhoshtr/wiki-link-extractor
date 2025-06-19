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
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut extractor = LinkExtractor::new()?;
    let mut total_links = 0;

    // Read the entire input into a single string
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    // Pass the entire input to extract_links
    let links = extractor.extract_links(&input)?;
    total_links += links.len();

    for link in links.iter() {
        println!("{}\t{}", link.title, link.label.as_deref().unwrap_or(""),);
    }

    println!("Total links found: {}", total_links);

    println!("Total links found: {}", total_links);

    Ok(())
}
