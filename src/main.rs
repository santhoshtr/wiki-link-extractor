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
    let mut line_number = 1;
    let mut buffer = String::new();
    for line in stdin.lock().lines() {
        let line = line?;
        buffer.push_str(&line);
        buffer.push('\n');

        if buffer.contains("\n\n") {
            let parts: Vec<&str> = buffer.splitn(2, "\n\n").collect();
            let block = parts[0];
            buffer = parts[1].to_string();

            println!("Processing block at line: {}", line_number);
            line_number += block.lines().count();

            let links = extractor.extract_links(block)?;
            total_links += links.len();

            for link in links.iter() {
                println!("{}\t{}", link.title, link.label.as_deref().unwrap_or(""),);
            }
        }
    }

    // Process any remaining text in the buffer
    if !buffer.is_empty() {
        println!("Processing remaining block at line: {}", line_number);
        let links = extractor.extract_links(&buffer)?;
        total_links += links.len();

        for link in links.iter() {
            println!("{}\t{}", link.title, link.label.as_deref().unwrap_or(""),);
        }
    }

    println!("Total links found: {}", total_links);

    Ok(())
}
