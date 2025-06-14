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
    let markdown = "= Example Document =\n\nThis is [[Another|Title]]\n";

    let mut extractor = LinkExtractor::new()?;
    let links = extractor.extract_links(markdown)?;

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
