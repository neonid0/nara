// extraction utils
//
//
//
// iterators for extracting tokens from a string
// I used Nom parser combinator solutions in these methods
fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or(s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];

    (remainder, extracted)
}

fn take_while_restrict(
    accept: impl Fn(char) -> bool,
    s: &str,
    error_msg: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, s);

    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((remainder, extracted))
    }
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c.is_whitespace(), s)
}

pub(crate) fn extract_whitespace_restrict(s: &str) -> Result<(&str, &str), String> {
    take_while_restrict(
        |c| c == ' ' || c == '\t',
        s,
        "expected whitespace".to_string(),
    )
}

pub(crate) fn extract_paranthesis(s: &str) -> Result<(&str, &str), String> {
    s.chars()
        .next()
        .filter(|&c| c == '(')
        .map(|_| {
            let (remainder, extracted) = take_while(|c| c != ')', &s[1..]);
            if let Some(after_paren) = remainder.strip_prefix(')') {
                Ok((after_paren.trim_start(), extracted))
            } else {
                Err("expected closing parenthesis".to_string())
            }
        })
        .unwrap_or_else(|| Err("expected opening parenthesis".to_string()))
}

pub(crate) fn extract_semicolon(s: &str) -> (&str, &str) {
    if let Some(after_semi) = s.strip_prefix(';') {
        (after_semi.trim_start(), ";")
    } else {
        (s, "")
    }
}

// didn't use this function in the code, but it might be useful later if I change the way
// semicolons are handled
#[allow(dead_code)]
pub(crate) fn extract_semicolon_restrict(s: &str) -> Result<(&str, &str), String> {
    if let Some(after_semi) = s.strip_prefix(';') {
        Ok((after_semi.trim_start(), ";"))
    } else {
        Err("expected semicolon".to_string())
    }
}

// block expression statements extraction
pub(crate) fn sequence<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    mut s: &str,
) -> Result<(&str, Vec<T>), String> {
    let mut items = Vec::new();

    while let Ok((new_s, item)) = parser(s) {
        s = new_s;
        items.push(item);

        let (new_s, _) = extract_whitespace(s);
        s = new_s;
    }

    Ok((s, items))
}

pub(crate) fn extract_params(s: &str) -> Result<(&str, Vec<String>), String> {
    let (s, params) = extract_paranthesis(s)?;

    let params = params
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    Ok((s, params))
}

// extract related tokens (eg. fn, val, mut)
pub(crate) fn tag<'b>(starting_text: &str, s: &'b str) -> Result<&'b str, String> {
    s.strip_prefix(starting_text)
        .ok_or_else(|| format!("expected {}", starting_text))
}

pub(crate) fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_starts_with_alphabetic {
        Ok(take_while(|c| c.is_ascii_alphanumeric(), s))
    } else {
        Err("expected identifier".to_string())
    }
}

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while_restrict(|c| c.is_ascii_digit(), s, "expected digits".to_string())
}

// there should be a better solution
pub(crate) fn extract_float(s: &str) -> Result<(&str, String), String> {
    let (s, integer_part) = extract_digits(s)?;

    if let Some(after_dot) = s.strip_prefix('.') {
        let (s, fractional_part) = take_while_restrict(
            |c| c.is_ascii_digit(),
            after_dot,
            "expected digits after decimal point".to_string(),
        )?;

        let float_str = format!("{}.{}", integer_part, fractional_part);
        Ok((s, float_str))
    } else {
        Err("expected decimal point".to_string())
    }
}

#[allow(clippy::while_let_on_iterator)]
pub(crate) fn extract_string_literal(s: &str) -> Result<(&str, String), String> {
    // Check if string starts with double quote
    if !s.starts_with('"') {
        return Err("expected opening double quote".to_string());
    }

    let mut chars = s[1..].chars();
    let mut result = String::new();
    let mut consumed = 1; // for opening quote

    // We need while let here because we call chars.next() again for escape sequences
    while let Some(ch) = chars.next() {
        consumed += ch.len_utf8();
        match ch {
            '"' => {
                // Found closing quote
                return Ok((&s[consumed..], result));
            }
            '\\' => {
                // Handle escape sequences
                if let Some(escaped) = chars.next() {
                    consumed += escaped.len_utf8();
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                } else {
                    return Err("unexpected end of string after backslash".to_string());
                }
            }
            _ => result.push(ch),
        }
    }

    Err("unclosed string literal".to_string())
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum FStringPart {
    Text(String),
    Interpolation(String),
}

#[allow(clippy::while_let_on_iterator)]
pub(crate) fn extract_fstring(s: &str) -> Result<(&str, Vec<FStringPart>), String> {
    // Check if string starts with f"
    if !s.starts_with("f\"") {
        return Err("expected f-string starting with f\"".to_string());
    }

    let mut chars = s[2..].chars();
    let mut parts = Vec::new();
    let mut current_text = String::new();
    let mut consumed = 2; // for 'f"'

    // We need while let here because we call chars.next() multiple times for escape sequences and interpolations
    while let Some(ch) = chars.next() {
        consumed += ch.len_utf8();
        match ch {
            '"' => {
                // End of f-string
                if !current_text.is_empty() {
                    parts.push(FStringPart::Text(current_text));
                }
                return Ok((&s[consumed..], parts));
            }
            '{' => {
                // Start of interpolation
                if !current_text.is_empty() {
                    parts.push(FStringPart::Text(current_text.clone()));
                    current_text.clear();
                }

                // Find matching }
                let mut expr = String::new();
                let mut depth = 1;
                while let Some(ch) = chars.next() {
                    consumed += ch.len_utf8();
                    if ch == '{' {
                        depth += 1;
                        expr.push(ch);
                    } else if ch == '}' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                        expr.push(ch);
                    } else {
                        expr.push(ch);
                    }
                }

                if depth != 0 {
                    return Err("unclosed interpolation in f-string".to_string());
                }

                parts.push(FStringPart::Interpolation(expr));
            }
            '\\' => {
                // Handle escape sequences
                if let Some(escaped) = chars.next() {
                    consumed += escaped.len_utf8();
                    match escaped {
                        'n' => current_text.push('\n'),
                        't' => current_text.push('\t'),
                        'r' => current_text.push('\r'),
                        '\\' => current_text.push('\\'),
                        '"' => current_text.push('"'),
                        '{' => current_text.push('{'),
                        '}' => current_text.push('}'),
                        _ => {
                            current_text.push('\\');
                            current_text.push(escaped);
                        }
                    }
                } else {
                    return Err("unexpected end of f-string after backslash".to_string());
                }
            }
            _ => current_text.push(ch),
        }
    }

    Err("unclosed f-string literal".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("\n   \n  1"), ("1", "\n   \n  "))
    }

    #[test]
    fn extract_spaces_restrict() {
        assert_eq!(extract_whitespace_restrict(" x"), Ok(("x", " ")))
    }

    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_ident("abcdEFGH stop"), Ok((" stop", "abcdEFGH")))
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("foobar()"), Ok(("()", "foobar")))
    }

    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(
            extract_ident("123abc"),
            Err("expected identifier".to_string()),
        );
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("val", "val x"), Ok(" x"))
    }

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), Ok(("-20", "10")));
    }

    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(extract_digits("abcd"), Err("expected digits".to_string()));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
    }

    #[test]
    fn do_not_extract_spaces1_when_input_does_not_start_with_them() {
        assert_eq!(
            extract_whitespace_restrict("blah"),
            Err("expected whitespace".to_string()),
        );
    }

    #[test]
    fn extract_simple_fstring() {
        assert_eq!(
            extract_fstring("f\"hello\""),
            Ok(("", vec![FStringPart::Text("hello".to_string())]))
        );
    }

    #[test]
    fn extract_fstring_with_interpolation() {
        assert_eq!(
            extract_fstring("f\"Hello {name}!\""),
            Ok((
                "",
                vec![
                    FStringPart::Text("Hello ".to_string()),
                    FStringPart::Interpolation("name".to_string()),
                    FStringPart::Text("!".to_string()),
                ]
            ))
        );
    }

    #[test]
    fn extract_fstring_with_expression() {
        assert_eq!(
            extract_fstring("f\"Result: {10 + 20}\""),
            Ok((
                "",
                vec![
                    FStringPart::Text("Result: ".to_string()),
                    FStringPart::Interpolation("10 + 20".to_string()),
                ]
            ))
        );
    }

    #[test]
    fn extract_fstring_with_escaped_braces() {
        assert_eq!(
            extract_fstring("f\"test \\{not_interpolated\\}\""),
            Ok((
                "",
                vec![FStringPart::Text("test {not_interpolated}".to_string())]
            ))
        );
    }

    #[test]
    fn extract_fstring_unclosed() {
        assert!(extract_fstring("f\"hello").is_err());
    }

    #[test]
    fn extract_fstring_unclosed_interpolation() {
        assert!(extract_fstring("f\"hello {name\"").is_err());
    }
}
