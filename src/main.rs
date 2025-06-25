use extractor::LinkExtractor;
use std::fs;
use std::io::Write;
mod extractor;

#[derive(Debug, Clone)]
pub struct MarkdownLink {
    pub label: String,
    pub url: String,
    pub title: Option<String>,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone)]
pub struct Article {
    pub text: String,
    pub id: String,
    pub namespace: usize,
    pub title: String,
    pub redirect: bool,
}

// Example usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut extractor = LinkExtractor::new()?;
    let mut total_links = 0;
    let mut articles_processed = 0;
    let mut parsing_errors = 0;
    use std::io::BufWriter;
    let tsv_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("links.tsv")?;
    let mut tsv_writer = BufWriter::new(tsv_file);

    // Read the file and pass content to extract_links. No need to read from stdin.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }
    let file_name = &args[1];
    use quick_xml::Reader;
    use quick_xml::events::Event;

    use std::io::BufReader;
    let file = fs::File::open(file_name)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.config_mut().trim_text(true);
    // reader.config_mut().allow_unmatched_ends = true;
    // reader.config_mut().check_end_names = false;
    let mut buf = Vec::new();
    let mut article = Article {
        text: String::new(),
        id: String::new(),
        namespace: 0,
        title: String::new(),
        redirect: false,
    };
    let mut tag_stack: Vec<String> = Vec::new();
    // Extract the text under the <text> node
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                let base_tag = e
                    .name()
                    .into_inner()
                    .to_vec()
                    .into_iter()
                    .map(|c| c as char)
                    .collect::<String>();

                tag_stack.push(base_tag);
                let path = tag_stack.join("/");
                if path == "mediawiki/page" {
                    article.text.clear();
                    article.id.clear();
                    article.namespace = 0;
                    article.redirect = false;
                }
            }
            Ok(Event::Empty(e)) => {
                let base_tag = e
                    .name()
                    .into_inner()
                    .to_vec()
                    .into_iter()
                    .map(|c| c as char)
                    .collect::<String>();
                if base_tag == "redirect" {
                    article.redirect = true
                }
            }
            Ok(Event::Text(e)) => {
                let path = tag_stack.join("/");
                match path.as_str() {
                    "mediawiki/page/revision/text" => {
                        article.text = e.unescape().unwrap().into_owned();
                    }
                    "mediawiki/page/id" => {
                        article.id = e.unescape().unwrap().into_owned();
                    }
                    "mediawiki/page/ns" => {
                        article.namespace =
                            e.unescape().unwrap().parse::<usize>().unwrap_or(999999);
                    }
                    "mediawiki/page/title" => {
                        article.title = e.unescape().unwrap().into_owned();
                    }
                    _ => (),
                }
            }
            Ok(Event::End(_e)) => {
                let path = tag_stack.join("/");
                // dbg!(&path);
                tag_stack.pop();
                if path.as_str() == "mediawiki/page/revision/text" {
                    article.text.push('\n');

                    // Only process links if namespace is 0 and redirect is false
                    if article.namespace == 0 && !article.redirect {
                        articles_processed += 1;
                        let links = match extractor.extract_links(&article.text) {
                            Ok(links) => links,
                            Err(_) => {
                                eprintln!(
                                    "Error parsing article: id={}, title={}, namespace={}",
                                    article.id, article.title, article.namespace
                                );
                                let dir = "data";
                                fs::create_dir_all(dir)?;
                                let file_path = format!("{}/{}.wikitext", dir, article.id);
                                let mut file = fs::File::create(file_path)?;
                                file.write_all(article.text.as_bytes())?;
                                extractor = LinkExtractor::new()?;
                                parsing_errors += 1;
                                continue;
                            }
                        };
                        total_links += links.len();

                        for link in links.iter() {
                            writeln!(
                                tsv_writer,
                                "{}\t{}\t{}",
                                article.title,
                                link.title,
                                link.label.as_deref().unwrap_or(&link.title),
                            )?;
                        }
                        if articles_processed % 1000 == 0 {
                            println!(
                                "Articles processed: {}, Links collected {}",
                                articles_processed, total_links
                            );
                        }
                    }
                }
            }
            // There are several other `Event`s we do not consider here
            _ => (),
        }
    }
    println!(
        "Articles processed: {}\nLinks collected: {}\nErrors: {}\n",
        articles_processed, total_links, parsing_errors
    );
    Ok(())
}
