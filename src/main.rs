use std::fs::read;

fn main() {
    println!("Hello, world!");
}

enum ParseState {
    Base,
    LineComment,
    MultiLineComment,
    EndingMultiLineComment,
    StringLiteral(char),
    StringLiteralEscaped(char),
}

fn process(inp: &str) -> String {
    let mut output = String::with_capacity(inp.len());
    let mut parse_state = ParseState::Base;

    for (i, ch) in inp.char_indices() {
        use ParseState::*;
        match parse_state {
            LineComment => {
                if starts_with_newline(&inp[i..]) {
                    parse_state = Base;
                    output.push(ch);
                }
            }
            MultiLineComment => {
                if multiline_comment_end(&inp[i..]) {
                    parse_state = EndingMultiLineComment;
                }
            }
            EndingMultiLineComment => {
                parse_state = Base;
            }
            StringLiteral(literal_end) => {
                if ch == literal_end {
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
                if starts_string_literal(&ch) {
                    parse_state = StringLiteral(ch);
                    output.push(ch);
                } else if multiline_comment_start(&inp[i..]) {
                    parse_state = MultiLineComment;
                } else if line_comment_start(&inp[i..]) {
                    parse_state = LineComment;
                } else {
                    output.push(ch);
                }
            }
        }
    }
    output
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

fn starts_string_literal(ch: &char) -> bool {
    ['\"', '\''].contains(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_strip_comments_inside_literals() {
        let inp = "I am code \"with a // funky /* literal */\"";
        assert_eq!(&process(inp), inp);
    }

    #[test]
    fn it_supports_escaped_quotes_inside_literals() {
        let inp = r#"I am code " with \" an /* even */ funkier // literal " "#;
        assert_eq!(&process(inp), inp);
    }

    #[test]
    fn it_supports_escaping_the_escape_character_inside_string_literals() {
        let inp = r#"I am code "with a literal \\"//and a comment"#;
        let expected = r#"I am code "with a literal \\""#;
        assert_eq!(&process(inp), expected);
    }

    #[test]
    fn it_supports_single_and_double_quotes_and_they_dont_close_each_other() {
        let inp = r#"'single /* quote */ -> " <- // still literal '"#;
        assert_eq!(&process(inp), inp);
        let inp = r#""double /* quote */ -> ' <- // still literal ""#;
        assert_eq!(&process(inp), inp);
    }

    #[test]
    fn string_literals_can_contain_newlines_and_unclosed_literals_run_till_eof() {
        let inp = "' <- all\n\n\n\r\nof // this /* is */ \ninside\n a literal";
        assert_eq!(&process(inp), inp);
    }

    #[test]
    fn it_strips_line_comments() {
        let inp = "I am code! //And I am a comment! /* still */\n";
        let expected = "I am code! \n";
        assert_eq!(&process(inp), expected);

        let inp = "I am code! //And I am a comment! /* still */\nCode again //But not this\n";
        let expected = "I am code! \nCode again \n";
        assert_eq!(&process(inp), expected);
    }

    #[test]
    fn it_strips_multiline_comments() {
        let inp = "I am code /* And I\n\r\n\n am not */ and I am too.";
        let expected = "I am code  and I am too.";
        assert_eq!(&process(inp), expected);
    }

    #[test]
    fn it_handles_windows_newlines_and_leaves_them_intact() {
        let inp = "I am code\r\n//I am a comment\r\n";
        let expected = "I am code\r\n\r\n";
        assert_eq!(&process(inp), expected);
    }

    #[test]
    fn unclosed_multiline_comments_will_strip_until_eof() {
        let inp = "code/*\n\n\nA lot of commentary\r\netc";
        let expected = "code";
        assert_eq!(&process(inp), expected);
    }

    #[test]
    fn trailing_newline_is_optional_and_preserved() {
        let inp = "code//comment";
        let expected = "code";
        assert_eq!(&process(inp), expected);

        let inp = "code//comment\r\n";
        let expected = "code\r\n";
        assert_eq!(&process(inp), expected);
    }

    #[test]
    fn it_handles_multibyte_characters() {
        let inp = "🌴🌴🌴/*🦎🦎🦎*/🌴🌴//🦎🦎🦎";
        let expected = "🌴🌴🌴🌴🌴";
        assert_eq!(&process(inp), expected);
    }
}
