use pulldown_cmark::{html, Options, Parser};
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

/// A single-page documentation generator for the model-generator project.
///
/// This binary generates a comprehensive single HTML page containing documentation
/// for the entire project, including module documentation, README content, and examples.
fn main() -> io::Result<()> {
    // First generate standard docs with rustdoc
    println!("Generating standard documentation...");
    let output = Command::new("cargo")
        .args(["doc", "--no-deps", "--document-private-items"])
        .output()?;

    if !output.status.success() {
        eprintln!(
            "Failed to generate documentation: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(io::Error::new(io::ErrorKind::Other, "rustdoc failed"));
    }

    // Define the output directory and file
    let output_dir = PathBuf::from("target/single-page-docs");
    fs::create_dir_all(&output_dir)?;
    let output_file = output_dir.join("index.html");
    let mut file = File::create(&output_file)?;

    // Read the Cargo.toml to get package information
    let cargo_toml = fs::read_to_string("Cargo.toml")?;
    let package_name = extract_package_name(&cargo_toml);
    let package_version = extract_package_version(&cargo_toml);
    let package_description = extract_package_description(&cargo_toml);

    // Start generating the HTML
    println!("Generating single-page documentation...");
    write_html_header(
        &mut file,
        &package_name,
        &package_version,
        &package_description,
    )?;

    // Get modules and structure from lib.rs
    let lib_content = fs::read_to_string("src/lib.rs")?;
    let modules = extract_modules(&lib_content);

    // Table of contents
    write_toc(&mut file, &modules)?;

    // Main documentation content
    write_section(&mut file, "Overview", &package_description)?;

    // Path to rustdoc-generated documentation
    let rustdoc_path = PathBuf::from("target/doc");

    // Write module documentation from rustdoc
    document_modules(&mut file, &rustdoc_path, &package_name, &modules)?;

    // Copy README content
    // Commented out to exclude README from documentation
    // if let Ok(readme) = fs::read_to_string("README.md") {
    //     write_section(&mut file, "README", &readme)?;
    // }

    // Write examples
    let examples_dir = PathBuf::from("examples");
    if examples_dir.exists() {
        document_examples(&mut file, &examples_dir)?;
    }

    // Close the HTML
    writeln!(file, "</body></html>")?;

    println!(
        "Single-page documentation generated at: {}",
        output_file.display()
    );
    Ok(())
}

fn write_html_header(
    file: &mut File,
    name: &str,
    version: &str,
    description: &str,
) -> io::Result<()> {
    writeln!(
        file,
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} v{} - Documentation</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/rust.min.js"></script>
    <script>
        document.addEventListener('DOMContentLoaded', (event) => {{
            document.querySelectorAll('pre code').forEach((el) => {{
                hljs.highlightElement(el);
            }});
        }});
    </script>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            font-size: 16px;
        }}
        pre, code {{
            font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
            border-radius: 3px;
        }}
        pre {{
            padding: 16px;
            overflow: auto;
            background-color: #f6f8fa;
            border: 1px solid #eaecef;
            margin: 1em 0;
        }}
        pre code {{
            background-color: transparent;
            padding: 0;
            white-space: pre;
            border: none;
        }}
        code {{
            padding: 0.2em 0.4em;
            background-color: rgba(27, 31, 35, 0.05);
            font-size: 85%;
        }}
        h1, h2, h3, h4, h5, h6 {{
            margin-top: 24px;
            margin-bottom: 16px;
            font-weight: 600;
            line-height: 1.25;
        }}
        h1 {{
            padding-bottom: 0.3em;
            font-size: 2em;
            border-bottom: 1px solid #eaecef;
        }}
        h2 {{
            padding-bottom: 0.3em;
            font-size: 1.5em;
            border-bottom: 1px solid #eaecef;
        }}
        h3 {{ font-size: 1.25em; }}
        h4 {{ font-size: 1em; }}
        a {{
            color: #0366d6;
            text-decoration: none;
        }}
        a:hover {{
            text-decoration: underline;
        }}
        table {{
            border-collapse: collapse;
            width: 100%;
            margin-bottom: 16px;
        }}
        table, th, td {{
            border: 1px solid #dfe2e5;
        }}
        th, td {{
            padding: 8px 13px;
        }}
        th {{
            background-color: #f6f8fa;
        }}
        .toc {{
            background-color: #f8f8f8;
            padding: 15px;
            margin-bottom: 20px;
            border-radius: 5px;
            border: 1px solid #e1e4e8;
        }}
        .toc ul {{
            list-style-type: none;
            padding-left: 20px;
        }}
        .toc li {{
            margin: 8px 0;
        }}
        .section {{
            margin-bottom: 40px;
        }}
        .rust-example {{
            background-color: #f9f9f9;
            border-left: 4px solid #ddd;
            padding: 10px 15px;
            margin-bottom: 20px;
        }}
        .module-item {{
            margin-bottom: 30px;
        }}
        blockquote {{
            color: #6a737d;
            border-left: 0.25em solid #dfe2e5;
            padding: 0 1em;
            margin: 0 0 16px 0;
        }}
        ul, ol {{
            padding-left: 2em;
        }}
        p {{
            margin-top: 0;
            margin-bottom: 16px;
            font-size: 16px;
        }}
        /* Reset font size for all elements to prevent inheritance issues */
        * {{
            font-size: inherit;
        }}
        section *, article *, div * {{
            font-size: inherit;
        }}
        .code-example {{
            font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
            background-color: #f6f8fa;
            border: 1px solid #eaecef;
            padding: 16px;
            overflow: auto;
            line-height: 1.45;
            border-radius: 3px;
            font-size: 18px;
            white-space: pre-wrap;
            margin: 1em 0;
        }}
        .hljs {{
            font-size: 18px;
            line-height: 1.45;
        }}
        pre code.language-rust {{
            font-size: 18px;
            line-height: 1.45;
        }}
        .usage-info pre {{
            margin: 1em 0;
            padding: 0;
            overflow: auto;
        }}
        .usage-info code {{
            font-size: 18px;
            padding: 16px;
            display: block;
        }}
    </style>
</head>
<body>
    <h1>{} v{}</h1>
    <p>{}</p>
"##,
        name, version, name, version, description
    )?;

    Ok(())
}

fn write_toc(file: &mut File, modules: &[String]) -> io::Result<()> {
    writeln!(
        file,
        r##"<nav class="toc">
    <h2>Table of Contents</h2>
    <ul>
        <li><a href="#overview">Overview</a></li>"##
    )?;

    for module in modules {
        writeln!(file, r##"        <li><a href="#{0}">{0}</a></li>"##, module)?;
    }

    writeln!(
        file,
        r##"        <li><a href="#examples">Examples</a></li>
    </ul>
</nav>"##
    )?;

    Ok(())
}

fn write_section(file: &mut File, title: &str, content: &str) -> io::Result<()> {
    let anchor = title.to_lowercase().replace(' ', "-");
    writeln!(
        file,
        r##"<section class="section" id="{}">
    <h2>{}</h2>
    {}
</section>"##,
        anchor,
        title,
        markdown_to_html(content)
    )?;

    Ok(())
}

fn document_modules(
    file: &mut File,
    rustdoc_path: &Path,
    crate_name: &str,
    modules: &[String],
) -> io::Result<()> {
    // Extract root module documentation
    let crate_index = rustdoc_path.join(format!("{}/index.html", crate_name));
    if crate_index.exists() {
        let root_doc = extract_rustdoc_content(&crate_index)?;
        write_section(file, "Root Module", &root_doc)?;
    }

    // Document each module
    for module_name in modules {
        let module_path = rustdoc_path.join(format!("{}/{}/index.html", crate_name, module_name));

        if module_path.exists() {
            let module_doc = extract_rustdoc_content(&module_path)?;
            write_section(file, module_name, &module_doc)?;

            // Look for submodules
            let module_dir = rustdoc_path.join(format!("{}/{}", crate_name, module_name));
            if module_dir.exists() && module_dir.is_dir() {
                document_submodules(file, rustdoc_path, crate_name, module_name)?;
            }
        } else {
            writeln!(
                file,
                r##"<section class="section" id="{0}">
    <h2>{1}</h2>
    <p>Module documentation not found.</p>
</section>"##,
                module_name, module_name
            )?;
        }
    }

    Ok(())
}

fn document_submodules(
    file: &mut File,
    rustdoc_path: &Path,
    crate_name: &str,
    parent_module: &str,
) -> io::Result<()> {
    let parent_dir = rustdoc_path.join(format!("{}/{}", crate_name, parent_module));

    if let Ok(entries) = fs::read_dir(parent_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let submodule_name = path.file_name().unwrap().to_string_lossy().to_string();
                let submodule_index = path.join("index.html");

                if submodule_index.exists() {
                    let full_name = format!("{}::{}", parent_module, submodule_name);
                    let submodule_doc = extract_rustdoc_content(&submodule_index)?;
                    write_section(file, &full_name, &submodule_doc)?;
                }
            }
        }
    }

    Ok(())
}

fn extract_rustdoc_content(html_path: &Path) -> io::Result<String> {
    let html_content = fs::read_to_string(html_path)?;
    let document = Html::parse_document(&html_content);
    let mut content = String::new();

    // Extract module docstring
    if let Some(docblock) = extract_docblock(&document) {
        content.push_str(&docblock);
        content.push_str("\n\n");
    }

    // Extract API items (structs, functions, etc.)
    content.push_str("## API Reference\n\n");

    // Extract structs
    if let Some(structs) = extract_items(&document, "struct") {
        content.push_str("### Structs\n\n");
        content.push_str(&structs);
        content.push_str("\n\n");
    }

    // Extract enums
    if let Some(enums) = extract_items(&document, "enum") {
        content.push_str("### Enums\n\n");
        content.push_str(&enums);
        content.push_str("\n\n");
    }

    // Extract functions
    if let Some(functions) = extract_items(&document, "fn") {
        content.push_str("### Functions\n\n");
        content.push_str(&functions);
        content.push_str("\n\n");
    }

    // Extract traits
    if let Some(traits) = extract_items(&document, "trait") {
        content.push_str("### Traits\n\n");
        content.push_str(&traits);
        content.push_str("\n\n");
    }

    // Extract type definitions
    if let Some(types) = extract_items(&document, "type") {
        content.push_str("### Type Definitions\n\n");
        content.push_str(&types);
        content.push_str("\n\n");
    }

    // Extract constants
    if let Some(constants) = extract_items(&document, "constant") {
        content.push_str("### Constants\n\n");
        content.push_str(&constants);
        content.push_str("\n\n");
    }

    // Extract macros
    if let Some(macros) = extract_items(&document, "macro") {
        content.push_str("### Macros\n\n");
        content.push_str(&macros);
        content.push_str("\n\n");
    }

    Ok(content)
}

fn extract_docblock(document: &Html) -> Option<String> {
    // Select the docblock element which contains the module documentation
    let docblock_selector = Selector::parse(".docblock").ok()?;
    let docblock = document.select(&docblock_selector).next()?;

    let mut markdown = String::new();
    for node in docblock.text() {
        markdown.push_str(node);
        markdown.push('\n');
    }

    Some(markdown)
}

fn extract_items(document: &Html, item_type: &str) -> Option<String> {
    let section_selector = match Selector::parse(&format!(".section-header.{}-item", item_type)) {
        Ok(selector) => selector,
        Err(_) => return None,
    };

    let mut items = String::new();
    let sections: Vec<_> = document.select(&section_selector).collect();

    if sections.is_empty() {
        return None;
    }

    for section in sections {
        // Get the item name
        if let Some(id_attr) = section.value().attr("id") {
            let item_name = id_attr.replace(&format!("{}.", item_type), "");
            items.push_str(&format!("#### {}\n\n", item_name));

            // Try to get the item description
            let docblock_selector = Selector::parse(&format!("#{} + .docblock", id_attr)).ok()?;
            if let Some(docblock) = document.select(&docblock_selector).next() {
                for node in docblock.text() {
                    items.push_str(node);
                    items.push('\n');
                }
            }

            // Try to get the item signature
            let code_selector = Selector::parse(&format!("#{} + .item-decl pre", id_attr)).ok()?;
            if let Some(code_block) = document.select(&code_selector).next() {
                items.push_str("\n```rust\n");
                for node in code_block.text() {
                    items.push_str(node);
                }
                items.push_str("\n```\n\n");
            }
        }
    }

    if items.is_empty() {
        None
    } else {
        Some(items)
    }
}

fn extract_package_name(cargo_toml: &str) -> String {
    extract_toml_value(cargo_toml, "name")
}

fn extract_package_version(cargo_toml: &str) -> String {
    extract_toml_value(cargo_toml, "version")
}

fn extract_package_description(cargo_toml: &str) -> String {
    extract_toml_value(cargo_toml, "description")
}

fn extract_toml_value(content: &str, key: &str) -> String {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(key) && line.contains('=') {
            return line
                .split('=')
                .nth(1)
                .unwrap_or("")
                .trim()
                .trim_matches('"')
                .to_string();
        }
    }
    String::new()
}

fn extract_modules(content: &str) -> Vec<String> {
    let mut modules = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("pub mod ") && line.ends_with(';') {
            let module_name = line
                .strip_prefix("pub mod ")
                .unwrap()
                .strip_suffix(';')
                .unwrap()
                .trim();

            modules.push(module_name.to_string());
        }
    }

    modules
}

fn markdown_to_html(markdown: &str) -> String {
    // Set up options for pulldown-cmark parser
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    // Parse the markdown and convert to HTML
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Replace duplicate class attributes and ensure code blocks have proper syntax highlighting
    let mut output = String::new();
    let mut in_code_block = false;

    // Map of file extensions to language identifiers
    let mut ext_to_lang = HashMap::new();
    ext_to_lang.insert("rs", "rust");
    ext_to_lang.insert("toml", "toml");
    ext_to_lang.insert("bash", "bash");
    ext_to_lang.insert("sh", "bash");

    for line in html_output.lines() {
        if line.contains("<pre><code") && !in_code_block {
            in_code_block = true;

            // Default to rust for code blocks without a language
            let mut language = "rust";

            // Extract the language from class attribute if it exists
            if line.contains("class=\"language-") {
                // Find the language specified in the first class attribute
                if let Some(start_idx) = line.find("language-") {
                    let start = start_idx + 9;
                    if let Some(end_idx) = line[start..].find("\"") {
                        language = &line[start..start + end_idx];
                    }
                }
            }

            // If this line has a code block, replace everything between <pre><code and >
            // with a single, clean class
            let pre_code_start = line.find("<pre><code").unwrap_or(0);
            let tag_end = line[pre_code_start..]
                .find(">")
                .map(|idx| pre_code_start + idx + 1)
                .unwrap_or(line.len());

            let before_tag = &line[0..pre_code_start];
            let after_tag = &line[tag_end..];

            // Create a clean opening tag with the correct language
            let clean_tag = format!("<pre><code class=\"language-{}\">", language);
            output.push_str(before_tag);
            output.push_str(&clean_tag);
            if !after_tag.is_empty() {
                output.push_str(after_tag);
            }
        } else if line.contains("</code></pre>") && in_code_block {
            in_code_block = false;
            output.push_str(line);
        } else {
            output.push_str(line);
        }
        output.push('\n');
    }

    output
}

fn document_examples(file: &mut File, examples_dir: &Path) -> io::Result<()> {
    writeln!(
        file,
        r##"<section class="section" id="examples">
    <h2>Examples</h2>
    <p>Here are examples demonstrating various features of the library:</p>"##
    )?;

    let mut entries: Vec<_> = fs::read_dir(examples_dir)?
        .filter_map(|entry| entry.ok())
        .collect();

    // Sort examples alphabetically
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for entry in entries {
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let filename = path.file_name().unwrap().to_string_lossy();
            let content = fs::read_to_string(&path)?;

            // Extract example title and description from comments
            let mut title = filename.to_string();
            let mut description = String::new();

            let mut in_comment_block = false;
            for line in content.lines().take(20) {
                // Only check first 20 lines
                let line = line.trim();

                if line.starts_with("//!") {
                    // Module doc comment
                    let comment = line[3..].trim();
                    if !comment.is_empty() {
                        if description.is_empty() {
                            title = comment.to_string(); // First doc comment becomes title
                        } else {
                            description.push_str(comment);
                            description.push('\n');
                        }
                    }
                } else if line.starts_with("/**") {
                    in_comment_block = true;
                } else if line.starts_with("*/") {
                    in_comment_block = false;
                } else if in_comment_block && line.starts_with("*") {
                    let comment = line[1..].trim();
                    if !comment.is_empty() {
                        description.push_str(comment);
                        description.push('\n');
                    }
                }
            }

            // If no doc comments found, create a title from the filename
            if title == filename.to_string() {
                title = filename
                    .replace(".rs", "")
                    .split('_')
                    .map(|word| {
                        let mut c = word.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
            }

            // Extract API usage information instead of showing the full code
            let usage_info = extract_example_usage(&content);

            writeln!(
                file,
                r##"    <article class="example" id="example-{id}">
        <h3>{title}</h3>
        {description}
        <div class="usage-info">
            {usage}
        </div>
    </article>"##,
                id = filename.replace(".rs", ""),
                title = title,
                description = if !description.is_empty() {
                    format!("<p>{}</p>", description.replace("\n", "</p><p>"))
                } else {
                    String::new()
                },
                usage = usage_info
            )?;
        }
    }

    writeln!(file, "</section>")?;
    Ok(())
}

/// Extracts usage information from example code
fn extract_example_usage(source: &str) -> String {
    let mut usage_info = String::new();
    let mut in_usage_section = false;
    let mut usage_comment_buffer = String::new();
    let mut common_indent = usize::MAX;

    // Look for usage examples in comments
    for line in source.lines() {
        let trimmed = line.trim();

        // Look for special usage documentation markers
        if trimmed.starts_with("// USAGE:") || trimmed.starts_with("//! USAGE:") {
            in_usage_section = true;
            usage_info.push_str("<h4>Usage</h4>\n");
            common_indent = usize::MAX; // Reset indentation detection
            continue;
        }

        // Collect usage information from comments
        if in_usage_section && (trimmed.starts_with("//") || trimmed.starts_with("//!")) {
            let comment_start = if trimmed.starts_with("//!") { 3 } else { 2 };

            // Extract the actual comment content (preserving whitespace)
            let comment = if line.len() > comment_start {
                &line[line.find("//").unwrap() + comment_start..]
            } else {
                ""
            };

            // Detect common indentation to normalize it later
            if !comment.trim().is_empty() {
                let leading_spaces = comment.len() - comment.trim_start().len();
                common_indent = common_indent.min(leading_spaces);
            }

            usage_comment_buffer.push_str(comment);
            usage_comment_buffer.push('\n');

            // End of usage section
            if comment.trim().is_empty() {
                in_usage_section = false;

                if !usage_comment_buffer.is_empty() {
                    // Normalize indentation
                    let normalized = if common_indent < usize::MAX {
                        usage_comment_buffer
                            .lines()
                            .map(|line| {
                                if line.len() > common_indent {
                                    line[common_indent..].to_string()
                                } else {
                                    line.to_string()
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("\n")
                    } else {
                        usage_comment_buffer.clone()
                    };

                    usage_info.push_str("<pre><code class=\"language-rust\">\n");
                    usage_info.push_str(&normalized);
                    usage_info.push_str("</code></pre>\n");
                    usage_comment_buffer.clear();
                }
            }
        }
    }

    // Add any remaining buffer content if section didn't end with an empty line
    if in_usage_section && !usage_comment_buffer.is_empty() {
        // Normalize indentation
        let normalized = if common_indent < usize::MAX {
            usage_comment_buffer
                .lines()
                .map(|line| {
                    if line.len() > common_indent {
                        line[common_indent..].to_string()
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            usage_comment_buffer.clone()
        };

        usage_info.push_str("<pre><code class=\"language-rust\">\n");
        usage_info.push_str(&normalized);
        usage_info.push_str("</code></pre>\n");
    }

    // If no explicit usage section, try to find the main function or entry point
    if usage_info.is_empty() {
        let mut found_fn_main = false;
        let mut main_fn_usage = String::new();
        let mut in_main = false;
        let mut brackets_count = 0;
        let mut code_lines: Vec<String> = Vec::new();
        let mut common_indent = usize::MAX;

        for line in source.lines() {
            if line.contains("fn main") {
                found_fn_main = true;
                in_main = true;
                if line.contains('{') {
                    brackets_count += 1;
                }
                continue;
            }

            if in_main {
                if line.contains('{') {
                    brackets_count += line.matches('{').count();
                }
                if line.contains('}') {
                    brackets_count -= line.matches('}').count();
                    if brackets_count <= 0 {
                        in_main = false;
                    }
                }

                // Extract key function calls and usage patterns
                let trimmed = line.trim();
                if !trimmed.starts_with("//")
                    && trimmed.contains('(')
                    && !trimmed.contains("println!")
                    && !trimmed.contains("assert")
                {
                    // Preserve the original line with its indentation
                    code_lines.push(line.to_string());

                    // Calculate leading whitespace for normalization
                    let leading_spaces = line.len() - line.trim_start().len();
                    if leading_spaces > 0 {
                        common_indent = common_indent.min(leading_spaces);
                    }
                }
            }
        }

        if found_fn_main && !code_lines.is_empty() {
            // Normalize indentation by removing common prefix
            if common_indent < usize::MAX {
                for line in &code_lines {
                    if line.len() > common_indent {
                        main_fn_usage.push_str(&line[common_indent..]);
                    } else {
                        main_fn_usage.push_str(line);
                    }
                    main_fn_usage.push('\n');
                }
            } else {
                for line in &code_lines {
                    main_fn_usage.push_str(line);
                    main_fn_usage.push('\n');
                }
            }

            usage_info.push_str("<h4>Example Usage</h4>\n");
            usage_info.push_str("<pre><code class=\"language-rust\">\n");
            usage_info.push_str(&main_fn_usage);
            usage_info.push_str("</code></pre>\n");
        }
    }

    if usage_info.is_empty() {
        usage_info = "<p>No usage information available.</p>".to_string();
    }

    usage_info
}
