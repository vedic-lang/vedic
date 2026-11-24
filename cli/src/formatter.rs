use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::utilities::is_vedic_file;

#[derive(Debug, PartialEq)]
enum TokenKind {
    Atom,
    BlankLine,
    Comment,
    Symbol,
}

#[derive(Debug)]
struct Token {
    text: String,
    kind: TokenKind,
}

fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic()
        || c == '_'
        || ('\u{0900}'..='\u{0965}').contains(&c)
        || ('\u{0970}'..='\u{097F}').contains(&c)
}

fn is_identifier_continue(c: char) -> bool {
    is_identifier_start(c) || c.is_ascii_digit() || ('\u{0966}'..='\u{096F}').contains(&c)
}

fn is_number(c: char) -> bool {
    c.is_ascii_digit() || ('\u{0966}'..='\u{096F}').contains(&c)
}

fn tokenize(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut at = 0;

    while at < chars.len() {
        let c = chars[at];
        if c.is_whitespace() {
            if c == '\n' {
                at += 1;
                let mut next = at;
                while next < chars.len() && matches!(chars[next], ' ' | '\t' | '\r') {
                    next += 1;
                }
                if next < chars.len() && chars[next] == '\n' {
                    while at < chars.len() && chars[at].is_whitespace() {
                        at += 1;
                    }
                    tokens.push(Token {
                        text: String::new(),
                        kind: TokenKind::BlankLine,
                    });
                }
            } else {
                at += 1;
            }
            continue;
        }

        if c == '#' {
            let start = at;
            while at < chars.len() && chars[at] != '\n' {
                at += 1;
            }
            tokens.push(Token {
                text: chars[start..at].iter().collect(),
                kind: TokenKind::Comment,
            });
            continue;
        }

        if c == '\'' || c == '"' {
            let quote = c;
            let start = at;
            at += 1;
            while at < chars.len() {
                let current = chars[at];
                at += 1;
                if current == quote {
                    break;
                }
            }
            tokens.push(Token {
                text: chars[start..at].iter().collect(),
                kind: TokenKind::Atom,
            });
            continue;
        }

        if is_number(c) {
            let start = at;
            while at < chars.len() && is_number(chars[at]) {
                at += 1;
            }
            if at + 1 < chars.len() && chars[at] == '.' && is_number(chars[at + 1]) {
                at += 1;
                while at < chars.len() && is_number(chars[at]) {
                    at += 1;
                }
            }
            tokens.push(Token {
                text: chars[start..at].iter().collect(),
                kind: TokenKind::Atom,
            });
            continue;
        }

        if is_identifier_start(c) {
            let start = at;
            at += 1;
            while at < chars.len() && is_identifier_continue(chars[at]) {
                at += 1;
            }
            tokens.push(Token {
                text: chars[start..at].iter().collect(),
                kind: TokenKind::Atom,
            });
            continue;
        }

        let symbol = if at + 1 < chars.len() {
            let pair: String = chars[at..=at + 1].iter().collect();
            if matches!(pair.as_str(), "&&" | "||" | "==" | "!=" | "<=" | ">=") {
                at += 2;
                pair
            } else {
                at += 1;
                c.to_string()
            }
        } else {
            at += 1;
            c.to_string()
        };
        tokens.push(Token {
            text: symbol,
            kind: TokenKind::Symbol,
        });
    }

    tokens
}

fn is_operator(text: &str) -> bool {
    matches!(
        text,
        "=" | "=="
            | "!="
            | "<"
            | "<="
            | ">"
            | ">="
            | "+"
            | "-"
            | "*"
            | "/"
            | "%"
            | "&&"
            | "||"
            | "!"
    )
}

fn push_indent(line: &mut String, indent: usize) {
    if line.is_empty() {
        line.push_str(&" ".repeat(indent * 4));
    }
}

fn separate(line: &mut String) {
    if !line.is_empty() && !line.ends_with(char::is_whitespace) {
        line.push(' ');
    }
}

fn flush_line(line: &mut String, output: &mut String) {
    let trimmed = line.trim_end();
    if !trimmed.is_empty() {
        output.push_str(trimmed);
        output.push('\n');
    }
    line.clear();
}

pub fn format_source(source: &str) -> String {
    let tokens = tokenize(source);
    let mut output = String::new();
    let mut line = String::new();
    let mut indent = 0usize;
    let mut paren_depth = 0usize;
    let mut previous: Option<&str> = None;
    let mut after_close_brace = false;

    for (index, token) in tokens.iter().enumerate() {
        let next_is_comment = tokens
            .get(index + 1)
            .is_some_and(|next| next.kind == TokenKind::Comment);

        if after_close_brace {
            if token.text == "अथ" {
                separate(&mut line);
            } else {
                flush_line(&mut line, &mut output);
            }
            after_close_brace = false;
        }

        match token.kind {
            TokenKind::BlankLine => {
                flush_line(&mut line, &mut output);
                if !output.is_empty() && !output.ends_with("\n\n") {
                    output.push('\n');
                }
                previous = None;
            }
            TokenKind::Comment => {
                push_indent(&mut line, indent);
                if !line.is_empty() && !line.ends_with(char::is_whitespace) {
                    line.push_str("  ");
                }
                line.push_str(&token.text);
                flush_line(&mut line, &mut output);
                previous = None;
            }
            TokenKind::Atom => {
                push_indent(&mut line, indent);
                let needs_space = match previous {
                    Some(prev)
                        if token.text == "यदि" || token.text == "चक्र" || token.text == "पर्यन्त" =>
                    {
                        prev == "अथ"
                    }
                    Some(prev) => {
                        prev == "अथ"
                            || prev == ")"
                            || prev == "]"
                            || tokens
                                .get(index.wrapping_sub(1))
                                .is_some_and(|previous_token| {
                                    previous_token.kind == TokenKind::Atom
                                })
                    }
                    None => false,
                };
                if needs_space {
                    separate(&mut line);
                }
                line.push_str(&token.text);
                previous = Some(&token.text);
            }
            TokenKind::Symbol => {
                push_indent(&mut line, indent);
                match token.text.as_str() {
                    "{" => {
                        separate(&mut line);
                        line.push('{');
                        indent += 1;
                        flush_line(&mut line, &mut output);
                    }
                    "}" => {
                        flush_line(&mut line, &mut output);
                        indent = indent.saturating_sub(1);
                        push_indent(&mut line, indent);
                        line.push('}');
                        after_close_brace = true;
                    }
                    "(" => {
                        if matches!(previous, Some("यदि" | "चक्र" | "पर्यन्त"))
                        {
                            separate(&mut line);
                        }
                        line.push('(');
                        paren_depth += 1;
                    }
                    ")" => {
                        line.push(')');
                        paren_depth = paren_depth.saturating_sub(1);
                    }
                    "[" => line.push('['),
                    "]" => line.push(']'),
                    "." => line.push('.'),
                    "," => {
                        line.push(',');
                        if tokens
                            .get(index + 1)
                            .is_some_and(|next| next.text != "]" && next.text != ")")
                        {
                            line.push(' ');
                        }
                    }
                    ";" => {
                        line.push(';');
                        if paren_depth > 0 {
                            line.push(' ');
                        } else if next_is_comment {
                            line.push_str("  ");
                        } else {
                            flush_line(&mut line, &mut output);
                        }
                    }
                    op if is_operator(op) => {
                        let unary = matches!(op, "!" | "-")
                            && (previous.is_none()
                                || matches!(
                                    previous,
                                    Some(
                                        "(" | "["
                                            | "{"
                                            | ","
                                            | ";"
                                            | "="
                                            | "=="
                                            | "!="
                                            | "<"
                                            | "<="
                                            | ">"
                                            | ">="
                                            | "+"
                                            | "-"
                                            | "*"
                                            | "/"
                                            | "%"
                                            | "&&"
                                            | "||"
                                    )
                                ));
                        if !unary || !matches!(previous, None | Some("(" | "[" | "{")) {
                            separate(&mut line);
                        }
                        line.push_str(op);
                        line.push(' ');
                    }
                    _ => line.push_str(&token.text),
                }
                previous = Some(&token.text);
            }
        }
    }

    if !line.is_empty() {
        flush_line(&mut line, &mut output);
    }
    output
}

fn collect_vedic_files(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot read directory {}: {error}", directory.display()))?;
    let mut entries: Vec<_> = entries
        .map(|entry| entry.map_err(|error| format!("cannot read directory entry: {error}")))
        .collect::<Result<_, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("cannot inspect {}: {error}", path.display()))?;
        if file_type.is_dir() {
            collect_vedic_files(&path, files)?;
        } else if file_type.is_file() && is_vedic_file(&path) {
            files.push(path);
        }
    }
    Ok(())
}

pub fn format_target(target: &Path) -> Result<Vec<PathBuf>, String> {
    let metadata = fs::metadata(target)
        .map_err(|error| format!("cannot access {}: {error}", target.display()))?;
    let files = if metadata.is_file() {
        if !is_vedic_file(target) {
            return Err(format!(
                "{} is not a Vedic source file (.v, .ved, .veda, or .vedic)",
                target.display()
            ));
        }
        vec![target.to_path_buf()]
    } else if metadata.is_dir() {
        let mut files = Vec::new();
        collect_vedic_files(target, &mut files)?;
        files
    } else {
        return Err(format!("{} is not a file or directory", target.display()));
    };

    if files.is_empty() {
        return Err(format!(
            "no Vedic source files found in {}",
            target.display()
        ));
    }

    for file in &files {
        let source = fs::read_to_string(file)
            .map_err(|error| format!("cannot read {}: {error}", file.display()))?;
        let formatted = format_source(&source);
        if source != formatted {
            fs::write(file, formatted)
                .map_err(|error| format!("cannot write {}: {error}", file.display()))?;
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use super::format_source;
    use super::format_target;

    #[test]
    fn formats_blocks_and_operators() {
        let source = "मान x=1;यदि(x>0){वद(x+1);}अथ{वद(०);}";
        let expected = "मान x = 1;\nयदि (x > 0) {\n    वद(x + 1);\n} अथ {\n    वद(०);\n}\n";
        assert_eq!(format_source(source), expected);
    }

    #[test]
    fn preserves_strings_and_comments() {
        let source = "वद(\"# not a comment {x}\");# trailing comment\n# standalone\n";
        let expected = "वद(\"# not a comment {x}\");  # trailing comment\n# standalone\n";
        assert_eq!(format_source(source), expected);
    }

    #[test]
    fn preserves_a_single_blank_line_between_statements() {
        let source = "वद(१);\n\nवद(२);";
        assert_eq!(format_source(source), "वद(१);\n\nवद(२);\n");
    }

    #[test]
    fn keeps_for_loop_semicolons_inside_parentheses() {
        let source = "चक्र(मान i=०;i<३;i=i+१){वद(i);}";
        let expected = "चक्र (मान i = ०; i < ३; i = i + १) {\n    वद(i);\n}\n";
        assert_eq!(format_source(source), expected);
    }

    #[test]
    fn formatting_is_idempotent() {
        let source = "सूत्र f(x){फल -x;}वद(f(२));";
        let formatted = format_source(source);
        assert_eq!(format_source(&formatted), formatted);
    }

    #[test]
    fn formats_vedic_files_recursively_and_skips_other_files() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("vedic-format-test-{unique}"));
        let nested = root.join("nested");
        fs::create_dir_all(&nested).unwrap();
        let source_file = nested.join("example.ved");
        fs::write(&source_file, "मान x=१;").unwrap();
        fs::write(root.join("notes.txt"), "leave unchanged").unwrap();

        let formatted_files = format_target(&root).unwrap();

        assert_eq!(formatted_files, vec![PathBuf::from(&source_file)]);
        assert_eq!(fs::read_to_string(source_file).unwrap(), "मान x = १;\n");
        assert_eq!(
            fs::read_to_string(root.join("notes.txt")).unwrap(),
            "leave unchanged"
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn accepts_the_v_extension_used_by_the_vscode_language_package() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let file = std::env::temp_dir().join(format!("vedic-format-test-{unique}.v"));
        fs::write(&file, "वद(१);").unwrap();

        assert_eq!(format_target(&file).unwrap(), vec![file.clone()]);
        assert_eq!(fs::read_to_string(&file).unwrap(), "वद(१);\n");
        fs::remove_file(file).unwrap();
    }
}
