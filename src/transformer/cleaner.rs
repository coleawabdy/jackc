#[allow(dead_code)]
pub fn clean(source: &str) -> String {
    #[derive(PartialEq, Copy, Clone)]
    enum Context {
        None,
        Comment(bool),
        String,
    }

    let mut ret = String::new();
    ret.reserve(source.len());

    let mut context = Context::None;
    let mut skip_next = false;
    let mut was_last_whitespace = false;
    for (index, c) in source.chars().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

		let was_comment = matches!(context, Context::Comment(_));

        context = match context {
            Context::None => match c {
                '/' => {
                    let next_char = source.chars().nth(index + 1);

                    match next_char {
                        Some('*') | Some('/') => {
                            skip_next = true;
                            Context::Comment(next_char == Some('*'))
                        }
                        None | Some(_) => context,
                    }
                }
                '"' => Context::String,
                _ => context,
            },
            Context::Comment(true) => {
                if c == '*' && source.chars().nth(index + 1).unwrap_or_default() == '/' {
                    skip_next = true;
                    Context::None
                } else {
                    context
                }
            }
            Context::Comment(false) => {
                if c == '\n' {
                    Context::None
                } else {
                    context
                }
            }
            Context::String => {
                if c == '"' {
                    Context::None
                } else {
                    context
                }
            }
        };

        let was_comment = match context {
            Context::Comment(_) => true,
            _ => was_comment,
        };

        if c.is_whitespace() && !was_last_whitespace && !was_comment {
            ret.push(match c {
                '\n' => c,
                _ => ' ',
            });
            was_last_whitespace = true;
        } else if !was_comment {
            ret.push(c);
            was_last_whitespace = false;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use crate::transformer::cleaner::clean;

    #[test]
    fn remove_single_line_comments() {
        assert_eq!(clean("// SINGLE LINE COMMENT"), "");
        assert_eq!(
            clean("NON COMMENT\n// SINGLE LINE COMMENT\n"),
            "NON COMMENT\n"
        );
        assert_eq!(clean("NOT A COMMENT"), "NOT A COMMENT");
        assert_eq!(
            clean("NOT A COMMENT\n// SINGLE LINE COMMENT\nADDITIONAL NOT A COMMENT"),
            "NOT A COMMENT\nADDITIONAL NOT A COMMENT"
        );
    }
}
