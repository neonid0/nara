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
        .unwrap_or_else(|| s.len());

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

// may separate for handling newlines and spaces
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
            if remainder.starts_with(')') {
                Ok((remainder[1..].trim_start(), extracted))
            } else {
                Err("expected closing parenthesis".to_string())
            }
        })
        .unwrap_or_else(|| Err("expected opening parenthesis".to_string()))
}

pub(crate) fn extract_semicolon(s: &str) -> (&str, &str) {
    if s.starts_with(';') {
        (&s[1..].trim_start(), ";")
    } else {
        (&s[..], "")
    }
}

// didn't use this function in the code, but it might be useful later if I change the way
// semicolons are handled
pub(crate) fn extract_semicolon_restrict(s: &str) -> Result<(&str, &str), String> {
    if s.starts_with(';') {
        Ok((&s[1..].trim_start(), ";"))
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

pub(crate) fn extract_params<'a>(s: &str) -> Result<(&str, Vec<String>), String> {
    let (s, params) = extract_paranthesis(s)?;

    let params = params
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    Ok((s, params))
}

// extract related tokens (eg. fn, val, mut)
pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        Err(format!("expected {}", starting_text))
    }
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

    if s.starts_with('.') {
        let (s, fractional_part) = take_while_restrict(
            |c| c.is_ascii_digit(),
            &s[1..],
            "expected digits after decimal point".to_string(),
        )?;

        let float_str = format!("{}.{}", integer_part, fractional_part);
        Ok((s, float_str))
    } else {
        Err("expected decimal point".to_string())
    }
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
}
