use std::{io::Error, time::Instant};

use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, ParseOptions, ParseState, Parser, Query, QueryCursor};
use tree_sitter_wikitext::LANGUAGE;

#[derive(Debug, Clone)]
pub struct WikiLink {
    pub label: Option<String>,
    pub title: String,
    pub start_byte: usize,
    pub end_byte: usize,
}

pub struct LinkExtractor {
    parser: Parser,
    query: Query,
}

impl LinkExtractor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let language = LANGUAGE.into();
        let mut parser = Parser::new();
        parser.set_language(&language)?;

        // Tree-sitter query to match different types of links
        let query_str = r#"
            ; Inline links: [text](url "title")
            (wikilink
              (wikilink_page) @link.title
              (page_name_segment)? @link.label
            )
        "#;

        let query = Query::new(&language, query_str)?;

        Ok(LinkExtractor { parser, query })
    }

    pub fn extract_links(&mut self, wikitext: &str) -> Result<Vec<WikiLink>, &'static str> {
        let start_time = Instant::now();
        let timeout = 100000;
        let progress_callback = &mut |_: &ParseState| {
            if timeout > 0 && start_time.elapsed().as_micros() > timeout as u128 {
                return true;
            }

            false
        };

        let parse_opts = ParseOptions::new().progress_callback(progress_callback);
        let tree = self.parser.parse_with_options(
            &mut |i, _| {
                if i < wikitext.len() {
                    &wikitext[i..]
                } else {
                    ""
                }
            },
            None,
            Some(parse_opts),
        );
        if let Some(tree) = tree {
            let root_node = tree.root_node();
            let mut cursor = QueryCursor::new();
            let mut captures = cursor.captures(&self.query, root_node, wikitext.as_bytes());

            let mut links = Vec::new();
            while let Some((mat, capture_index)) = captures.next() {
                let capture = mat.captures[*capture_index];
                let capture_name = &self.query.capture_names()[capture.index as usize];
                let node_text = get_node_text(capture.node, wikitext);
                match *capture_name {
                    // Inline links
                    "link.title" => {
                        let title = node_text.trim_matches('"').trim_matches('\'');
                        // if title has : and . followed by image extensions, skip. AI!
                        links.push(WikiLink {
                            label: Some(String::new()),
                            title: title.to_string(),
                            start_byte: capture.node.start_byte(),
                            end_byte: capture.node.end_byte(),
                        });
                    }
                    "link.label" => {
                        if let Some(last_link) = links.last_mut() {
                            last_link.label = Some(node_text);
                        }
                    }
                    _ => {}
                }
            }
            Ok(links)
        } else {
            Err("Parse error")
        }
    }
}

fn get_node_text(node: Node, source: &str) -> String {
    source[node.start_byte()..node.end_byte()].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_links() -> Result<(), Box<dyn std::error::Error>> {
        let mut extractor = LinkExtractor::new()?;
        let wikitext = "[[Example| title]]";
        let links = extractor.extract_links(wikitext)?;

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].label, Some("Example".to_string()));
        assert_eq!(links[0].title, "https://example.com");

        Ok(())
    }
}
