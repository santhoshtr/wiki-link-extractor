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
    use std::io::{self, Read};

    let mut markup = String::new();
    io::stdin().read_to_string(&mut markup)?;
    if !markup.ends_with('\n') {
        markup.push('\n');
    }

    let mut extractor = LinkExtractor::new()?;
    let links = extractor.extract_links(&markup)?;

    println!("Found {} links:", links.len());
    for (i, link) in links.iter().enumerate() {
        println!("{}. Title: '{}'", i + 1, link.title);
        if let Some(label) = &link.label {
            println!("   Label: '{}'", label);
        }
        println!("   Position: bytes {}-{}", link.start_byte, link.end_byte);
    }

    Ok(())
}
