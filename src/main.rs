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
            let mut line_with_newline = line;
            if !line_with_newline.ends_with('\n') {
                line_with_newline.push('\n');
            }
            let links = extractor.extract_links(&line_with_newline)?;
            total_links += links.len();

            for link in links.iter() {
                println!("{}\t{}", link.title, link.label.as_deref().unwrap_or(""),);
            }
        }
    }

    println!("Total links found: {}", total_links);

    Ok(())
}
