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

    for line in stdin.lock().lines() {
        let line = line?;
        if !line.is_empty() {
            let links = extractor.extract_links(&line)?;
            total_links += links.len();

            for (i, link) in links.iter().enumerate() {
                println!("{}. Title: '{}'", total_links + i, link.title);
                if let Some(label) = &link.label {
                    println!("   Label: '{}'", label);
                }
                println!("   Position: bytes {}-{}", link.start_byte, link.end_byte);
            }
        }
    }

    println!("Total links found: {}", total_links);

    Ok(())
}
