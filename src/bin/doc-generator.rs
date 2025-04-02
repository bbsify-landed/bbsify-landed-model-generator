use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// A markdown documentation generator for examples in the model-generator project.
///
/// This binary generates markdown files containing documentation
/// for the examples in the project, suitable for GitHub Pages.
///
/// Usage:
///   doc-generator [--promote]
///
/// Options:
///   --promote  Place markdown files next to the actual example files in addition to the docs directory
fn main() -> io::Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let promote = args.iter().any(|arg| arg == "--promote");

    // Define the output directory
    let output_dir = PathBuf::from("target/markdown-docs");
    fs::create_dir_all(&output_dir)?;

    // Get package info for the header
    let cargo_toml = fs::read_to_string("Cargo.toml")?;
    let package_name = extract_package_name(&cargo_toml);

    println!("Generating markdown documentation for examples...");

    // Process examples
    let examples_dir = PathBuf::from("examples");
    if examples_dir.exists() {
        generate_examples_markdown(&output_dir, &examples_dir, &package_name, promote)?;
    } else {
        println!("No examples directory found.");
    }

    println!(
        "Markdown documentation generated at: {}",
        output_dir.display()
    );

    if promote {
        println!("Markdown files also placed next to example files in the examples directory.");
    }

    Ok(())
}

fn generate_examples_markdown(
    output_dir: &Path,
    examples_dir: &Path,
    package_name: &str,
    promote: bool,
) -> io::Result<()> {
    // Create index file for all examples
    let mut index_file = File::create(output_dir.join("index.md"))?;

    writeln!(index_file, "# {} Examples", package_name)?;
    writeln!(
        index_file,
        "\nThis page contains examples demonstrating various features of the {} library.\n",
        package_name
    )?;

    let mut entries: Vec<_> = fs::read_dir(examples_dir)?
        .filter_map(|entry| entry.ok())
        .collect();

    // Sort examples alphabetically
    entries.sort_by_key(|a| a.file_name());

    // Add examples list to index
    writeln!(index_file, "## Available Examples\n")?;

    for entry in &entries {
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
            let filename = path.file_name().unwrap().to_string_lossy();
            let example_name = filename.replace(".rs", "");

            // Extract title from the file
            let content = fs::read_to_string(&path)?;
            let title = extract_example_title(&content, &example_name);

            writeln!(index_file, "- [{}](./{}.md)", title, example_name)?;
        }
    }

    // Create a directory for example markdown files
    let examples_output_dir = output_dir.join("examples");
    fs::create_dir_all(&examples_output_dir)?;

    // Generate individual markdown file for each example
    for entry in entries {
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
            let filename = path.file_name().unwrap().to_string_lossy();
            let example_name = filename.replace(".rs", "");
            let content = fs::read_to_string(&path)?;

            // Generate markdown content
            let md_content = generate_markdown_content(&path, &content, package_name)?;

            // Create markdown file in the examples output directory
            let mut example_file =
                File::create(examples_output_dir.join(format!("{}.md", example_name)))?;
            example_file.write_all(md_content.as_bytes())?;

            // If promote flag is set, also place the markdown file next to the example file
            if promote {
                let example_md_path = path.with_extension("md");
                let mut promoted_file = File::create(example_md_path)?;
                promoted_file.write_all(md_content.as_bytes())?;
            }
        }
    }

    // If promote flag is set, also copy the index file to the examples directory
    if promote {
        let promoted_index_path = examples_dir.join("index.md");
        fs::copy(output_dir.join("index.md"), promoted_index_path)?;
    }
    Ok(())
}

fn generate_markdown_content(path: &Path, content: &str, package_name: &str) -> io::Result<String> {
    let filename = path.file_name().unwrap().to_string_lossy();
    let example_name = filename.replace(".rs", "");

    // Extract title and description
    let title = extract_example_title(content, &example_name);
    let description = extract_example_description(content);

    let mut md_content = String::new();

    // Write header
    md_content.push_str(&format!("# {}\n\n", title));
    md_content.push_str("[Back to Examples Index](./index.md)\n\n");

    // Write description if available
    if !description.is_empty() {
        md_content.push_str(&format!("{}\n\n", description));
    }

    // Extract and write usage information
    let usage_info = extract_example_usage(content);
    if !usage_info.is_empty() {
        md_content.push_str("## Usage\n\n");
        md_content.push_str(&format!("{}\n\n", usage_info));
    }

    // Add full source code
    md_content.push_str("## Complete Source Code\n\n");
    md_content.push_str("```rust\n");
    md_content.push_str(content);
    md_content.push_str("```\n");

    // Add footer
    md_content.push_str("\n---\n\n");
    md_content.push_str(&format!("Generated for {} library", package_name));

    Ok(md_content)
}

fn extract_example_title(content: &str, default_name: &str) -> String {
    // Extract title from comments
    for line in content.lines().take(20) {
        let line = line.trim();

        if let Some(comment) = line.strip_prefix("//!") {
            // Module doc comment
            let comment = comment.trim();
            if !comment.is_empty() {
                return comment.to_string();
            }
        } else if line.starts_with("/**") || line.starts_with("/*") {
            // Look for the first non-empty line in the block comment
            for block_line in content
                .lines()
                .skip_while(|l| !l.trim().starts_with("/*"))
                .take(10)
            {
                let trimmed = block_line.trim();
                if trimmed.starts_with("*") && !trimmed.starts_with("*/") {
                    let comment = trimmed[1..].trim();
                    if !comment.is_empty() {
                        return comment.to_string();
                    }
                }
            }
        }
    }

    // If no doc comments found, create a title from the filename
    default_name
        .split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn extract_example_description(content: &str) -> String {
    let mut description = String::new();
    let mut in_comment_block = false;
    let mut found_title = false;

    for line in content.lines().take(30) {
        let line = line.trim();

        if let Some(comment) = line.strip_prefix("//!") {
            // Module doc comment
            let comment = comment.trim();
            if !comment.is_empty() {
                if !found_title {
                    // Skip the first doc comment as it becomes the title
                    found_title = true;
                    continue;
                }
                description.push_str(comment);
                description.push('\n');
            }
        } else if line.starts_with("/**") {
            in_comment_block = true;
            found_title = false;
        } else if line.starts_with("*/") {
            in_comment_block = false;
        } else if in_comment_block && line.starts_with("*") {
            let comment = line[1..].trim();
            if !comment.is_empty() {
                if !found_title {
                    // Skip the first line as it becomes the title
                    found_title = true;
                    continue;
                }
                description.push_str(comment);
                description.push('\n');
            }
        }
    }

    description.trim().to_string()
}

/// Extracts usage information from example code as markdown
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

                    usage_info.push_str("```rust\n");
                    usage_info.push_str(&normalized);
                    usage_info.push_str("```\n");
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

        usage_info.push_str("```rust\n");
        usage_info.push_str(&normalized);
        usage_info.push_str("```\n");
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
                    if brackets_count == 0 {
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

            usage_info.push_str("```rust\n");
            usage_info.push_str(&main_fn_usage);
            usage_info.push_str("```\n");
        }
    }

    usage_info.trim().to_string()
}

fn extract_package_name(cargo_toml: &str) -> String {
    extract_toml_value(cargo_toml, "name")
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
