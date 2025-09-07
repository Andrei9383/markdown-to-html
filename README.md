# Markdown-to-HTML

A Rust application designed to convert Markdown files into HTML. By default, it doesn't use any styling, but it is possible to provide custom styling.

[![Repobeats analytics image](https://repobeats.axiom.co/api/embed/6dea00d51565eae612b4e8d766fd63b65fc4c712.svg)](https://repobeats.axiom.co/ "Repobeats analytics image")

## Overview

This project provides a command-line tool for converting Markdown files to HTML.  It's built with Rust for performance and reliability. The core functionality involves reading Markdown files from a specified directory, processing them, and outputting corresponding HTML files.  Currently, the processing step is a placeholder, and custom styling is not yet implemented.

## Key Features

*   **Markdown to HTML Conversion:** Converts Markdown files to HTML.
*   **Directory Processing:** Scans a directory for Markdown files.
*   **Custom Styling (Planned):**  Future support for applying custom CSS styling.
*   **Test Coverage:** Includes unit tests to ensure code quality.

## Tech Stack

*   **Rust:** Programming language.
*   **Cargo:** Rust's build system and package manager.

## Setup Instructions

1.  **Install Rust:** If you don't have Rust installed, download it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2.  **Clone the Repository:**
    ```bash
    git clone https://github.com/andrei9383/markdown-to-html.git
    cd markdown-to-html
    ```

3.  **Build the Project:**
    ```bash
    cargo build
    ```

## Usage Examples

1.  **Run the application:**
    ```bash
    cargo run
    ```
    This will process all Markdown files in the current directory and (eventually) output HTML files.  Currently, it only finds and reads the files; the conversion logic is a TODO.

## Architecture Summary

The application consists of the following modules:

*   **`src/main.rs`:** The main entry point of the application.  It finds Markdown files and calls the conversion logic.
*   **`src/lib.rs`:** Contains the core logic for finding Markdown files (`get_md_files_dir`) and reading their content (`get_content_from_md_file`).

The `get_md_files_dir` function recursively searches for markdown files within a directory.
The `get_content_from_md_file` reads the content of the markdown file.

## Tests

To run the tests:

```bash
cargo test
```

The following tests are currently implemented:

*   `get_files`: Tests the `get_md_files_dir` function by asserting that it correctly identifies Markdown files within a test directory.
*   `get_content`: Tests the `get_content_from_md_file` by reading the content of a markdown file and comparing it against the expected result.

## Contribution Guidelines

Contributions are welcome! Here's how you can contribute:

1.  **Fork the repository.**
2.  **Create a new branch for your feature or bug fix.**
3.  **Implement your changes and write tests.**
4.  **Submit a pull request.**

## License

MIT License

Copyright (c) 2025 Andrei9383

See the [LICENSE](LICENSE) file for details.
