use std::fs::read;

fn main() {
    println!("Hello, world!");
}

fn process(inp: &str) -> String {
    let mut output = String::with_capacity(inp.len());
    let mut reading_line_comment = false;
    let mut reading_multiline_comment = false;
    let mut take = 0;
    let mut escape_until: Option<char> = None;
    for (i, ch) in inp.char_indices() {
        println!(
            "{i}, {ch}, lc: {}, mlc: {}, t: {}, rest: {}",
            reading_line_comment,
            reading_multiline_comment,
            take,
            &inp[i..]
        );
        if take > 0 {
            take -= 1;
        } else if let Some(escape) = escape_until {
            if ch == escape {
                escape_until = None;
            }
            output.push(ch);
        } else if starts_escape(&ch) {
            escape_until = Some(ch);
            output.push(ch);
        } else if reading_line_comment {
            if starts_with_newline(&inp[i..]) {
                reading_line_comment = false;
                output.push(ch);
            }
        } else if reading_multiline_comment {
            if inp[i..].starts_with("*/") {
                reading_multiline_comment = false;
                take = 1;
            }
        } else if inp[i..].starts_with("/*") {
            reading_multiline_comment = true;
        } else if inp[i..].starts_with("//") {
            reading_line_comment = true;
        } else {
            output.push(ch);
        }
    }
    output
}

fn starts_with_newline(inp: &str) -> bool {
    inp.starts_with("\n") || inp.starts_with("\r\n")
}

fn starts_escape(ch: &char) -> bool {
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
