enum ParseState {
    Base,
    LineComment,
    MultiLineComment,
    EndingMultiLineComment,
    StringLiteral(&'static str),
    StringLiteralEscaped(&'static str),
}

fn multiline_comment_start(rest: &str) -> bool {
    rest.starts_with("/*")
}

fn multiline_comment_end(rest: &str) -> bool {
    rest.starts_with("*/")
}

fn line_comment_start(rest: &str) -> bool {
    rest.starts_with("//")
}

fn starts_with_newline(inp: &str) -> bool {
    inp.starts_with("\n") || inp.starts_with("\r\n")
}

fn escapes_literal_end(ch: char) -> bool {
    ch == '\\'
}

fn string_literals(rest: &str) -> Option<&'static str> {
    if rest.starts_with("'") {
        Some("'")
    } else if rest.starts_with("\"") {
        Some("\"")
    } else if rest.starts_with("r#\"") {
        Some("\"#")
    } else {
        None
    }
}

pub fn strip_comments(input: &str) -> String {
    use ParseState::*;
    let mut output = String::with_capacity(input.len());
    let mut parse_state = Base;

    for (i, ch) in input.char_indices() {
        match parse_state {
            LineComment => {
                if starts_with_newline(&input[i..]) {
                    parse_state = Base;
                    output.push(ch);
                }
            }
            MultiLineComment => {
                if multiline_comment_end(&input[i..]) {
                    parse_state = EndingMultiLineComment;
                }
            }
            EndingMultiLineComment => {
                parse_state = Base;
            }
            StringLiteral(literal_end) => {
                if input[i..].starts_with(literal_end) {
                    parse_state = Base;
                } else if escapes_literal_end(ch) {
                    parse_state = StringLiteralEscaped(literal_end);
                }
                output.push(ch);
            }
            StringLiteralEscaped(literal_end) => {
                parse_state = StringLiteral(literal_end);
                output.push(ch);
            }
            Base => {
                if let Some(literal_end) = string_literals(&input[i..]) {
                    parse_state = StringLiteral(literal_end);
                    output.push(ch);
                } else if multiline_comment_start(&input[i..]) {
                    remove_trailing_newline_and_whitespace(&mut output);
                    parse_state = MultiLineComment;
                } else if line_comment_start(&input[i..]) {
                    remove_trailing_newline_and_whitespace(&mut output);
                    parse_state = LineComment;
                } else {
                    output.push(ch);
                }
            }
        }
    }
    output
}

/// removes a single newline followed by any number of non-newline whitespace from the end
fn remove_trailing_newline_and_whitespace(input: &mut String) {
    let trimmed = input.trim_end_matches(|c: char| c.is_whitespace() && !['\n', '\r'].contains(&c));
    let remove_until = if trimmed.ends_with("\r\n") {
        '\r'
    } else if trimmed.ends_with("\n") {
        '\n'
    } else {
        return;
    };
    while let Some(c) = input.pop() {
        if c == remove_until {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_strip_comments_inside_literals() {
        let inp = "I am code \"with a // funky /* literal */\"";
        assert_eq!(&strip_comments(inp), inp);
    }

    #[test]
    fn it_supports_rust_raw_string_literals() {
        let inp =
            "r#\"This is a /* literal */ and // should not be stripped \"#//but this is a comment";
        let expected = "r#\"This is a /* literal */ and // should not be stripped \"#";
        assert_eq!(&strip_comments(inp), expected);
    }

    #[test]
    fn it_supports_escaped_quotes_inside_literals() {
        let inp = r#"I am code " with \" an /* even */ funkier // literal " "#;
        assert_eq!(&strip_comments(inp), inp);
    }

    #[test]
    fn it_supports_escaping_the_escape_character_inside_string_literals() {
        let inp = r#"I am code "with a literal \\"//and a comment"#;
        let expected = r#"I am code "with a literal \\""#;
        assert_eq!(&strip_comments(inp), expected);
    }

    #[test]
    fn it_supports_single_and_double_quotes_and_they_dont_close_each_other() {
        let inp = r#"'single /* quote */ -> " <- // still literal '"#;
        assert_eq!(&strip_comments(inp), inp);
        let inp = r#""double /* quote */ -> ' <- // still literal ""#;
        assert_eq!(&strip_comments(inp), inp);
    }

    #[test]
    fn string_literals_can_contain_newlines_and_unclosed_literals_run_till_eof() {
        let inp = "' <- all\n\n\n\r\nof // this /* is */ \ninside\n a literal";
        assert_eq!(&strip_comments(inp), inp);
    }

    #[test]
    fn it_strips_line_comments() {
        let inp = "I am code! //And I am a comment! /* still */\n";
        let expected = "I am code! \n";
        assert_eq!(&strip_comments(inp), expected);

        let inp = "I am code! //And I am a comment! /* still */\nCode again //But not this\n";
        let expected = "I am code! \nCode again \n";
        assert_eq!(&strip_comments(inp), expected);
    }

    #[test]
    fn it_removes_lines_with_only_whitespace_and_comments_entirely() {
        let inp =
            "I am code\n\t //And I am only commentary\nCode again\r\n\t /*long\ncommentary\n*/";
        let expected = "I am code\nCode again";
        assert_eq!(&strip_comments(inp), expected);
    }

    #[test]
    fn it_leaves_empty_lines_untouched() {
        let inp = "\t . \n\t \r\n\n\n\n\n\r\n\t";
        assert_eq!(&strip_comments(inp), inp);
    }

    #[test]
    fn it_strips_multiline_comments() {
        let inp = "I am code /* And I\n\r\n\n am not */ and I am too.";
        let expected = "I am code  and I am too.";
        assert_eq!(&strip_comments(inp), expected);
    }

    #[test]
    fn it_handles_windows_newlines_and_leaves_them_intact() {
        let inp = "I am code\r\n//I am a comment\r\n";
        let expected = "I am code\r\n";
        assert_eq!(&strip_comments(inp), expected);
    }

    #[test]
    fn unclosed_multiline_comments_will_strip_until_eof() {
        let inp = "code/*\n\n\nA lot of commentary\r\netc";
        let expected = "code";
        assert_eq!(&strip_comments(inp), expected);
    }

    #[test]
    fn trailing_newline_is_optional_and_preserved() {
        let inp = "code//comment";
        let expected = "code";
        assert_eq!(&strip_comments(inp), expected);

        let inp = "code//comment\r\n";
        let expected = "code\r\n";
        assert_eq!(&strip_comments(inp), expected);
    }

    #[test]
    fn it_handles_multibyte_characters() {
        let inp = "🌴🌴🌴/*🦎🦎🦎*/🌴🌴//🦎🦎🦎";
        let expected = "🌴🌴🌴🌴🌴";
        assert_eq!(&strip_comments(inp), expected);
    }
}
