# Link Extractor

This program extracts links from wikitext input and outputs the results in a valid TSV (Tab-Separated Values) format. It is designed to handle large volumes of input efficiently by processing the input line by line as a stream.

## Features

- Extracts links from wikitext input.
- Outputs link details in TSV format with the following columns:
  - `Title`: The title of the link.
  - `Label`: The label of the link (if available).
- Processes input line by line, making it suitable for large files.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.
2. Clone this repository:

   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

The program reads wikitext input from `stdin` and outputs the extracted links in TSV format to `stdout`.

### Example Input

Input wikitext:

```
= Example Document =

This is [[Another|Title]].
```

### Example Output

TSV output:

```
Another Title 10 25
```

### Running the Program

You can run the program by piping input to it. For example:

#### Single File

```bash
cat input.txt | cargo run --release
```

#### Multiple Files

You can concatenate multiple files and pipe them to the program:

```bash
cat file1.txt file2.txt | cargo run --release
```

#### Large Files

The program processes input line by line, so it can handle large files efficiently:

```bash
cat large_file.txt | cargo run --release
```

### Redirecting Output

To save the TSV output to a file, redirect the output:

```bash
cat input.txt | cargo run --release > output.tsv
```

## Testing

To run the tests for the program:

```bash
cargo test
```

## Notes

- Ensure that the input is in valid wikitext format for accurate link extraction.
- The program appends a newline to each line of input before processing to ensure proper parsing.
- **Powered by [tree_sitter_wikitext](https://github.com/santhoshtr/tree-sitter-wikitext):**
  This program uses the `tree_sitter_wikitext` library to parse wikitext efficiently. `tree_sitter_wikitext` is a Tree-sitter grammar for parsing wikitext, enabling structured and efficient extraction of elements like links, headings, and more.
