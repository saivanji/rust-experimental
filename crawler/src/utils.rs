pub fn no_leading_slash(input: &str) -> &str {
    if input.chars().count() == 0 || input.chars().nth(0).unwrap() != '/' {
        return input;
    }

    no_leading_slash(input.strip_prefix("/").unwrap_or(input))
}

pub fn no_trailing_slash(input: &str) -> &str {
    if input.chars().count() == 0 || input.chars().last().unwrap() != '/' {
        return input;
    }

    no_trailing_slash(input.strip_suffix("/").unwrap_or(input))
}
