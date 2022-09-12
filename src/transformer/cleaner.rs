#[allow(dead_code)]
pub fn clean(source: &str) -> String {
    #[derive(PartialEq, Copy, Clone)]
    enum Context {
        None,
        Comment(bool),
        String,
        Whitespace(bool),
    }

    let mut ret = String::new();
    ret.reserve(source.len());

    let mut context = Context::None;
    let mut skip_next = false;
    for (index, c) in source.chars().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        let next_context = match context {
            Context::None | Context::Whitespace(_) => match c {
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
                c => {
                    if c.is_whitespace() {
                        let existing_newline = matches!(context, Context::Whitespace(true));
                        Context::Whitespace(c == '\n' || existing_newline)
                    } else {
                        Context::None
                    }
                }
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

        match (context, next_context) {
            (Context::Whitespace(mut newline), next) => {
                newline = match next {
                    Context::Whitespace(true) => true,
                    _ => newline,
                };

                if !matches!(next, Context::Whitespace(_))
                    || source.chars().nth(index + 1).is_none()
                {
                    ret.push(if newline { '\n' } else { ' ' });
                }
                match next {
                    Context::None | Context::String => ret.push(c),
                    _ => {}
                }
            }
            (Context::None, Context::None) | (Context::String, _) | (_, Context::String) => {
                ret.push(c)
            }
            _ => {}
        }

        context = next_context;
    }

    ret
}

#[cfg(test)]
mod tests {
    use crate::transformer::cleaner::clean;

    #[test]
    fn remove_single_line_comments() {
        //assert_eq!(clean("// SINGLE LINE COMMENT"), "");
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

    #[test]
    fn remove_multi_line_comments() {
        assert_eq!(clean("/* MULTI LINE COMMENT */"), "");
        assert_eq!(clean("/* MULTI\nLINE\nCOMMENT\n*/"), "");
        assert_eq!(
            clean("NON COMMENT\n/* MULTI LINE COMMENT\n*/"),
            "NON COMMENT\n"
        );
        assert_eq!(
            clean("BEFORE NON COMMENT\n/* MULTI\n LINE\n COMMENT\n*/\nAFTER NON COMMENT"),
            "BEFORE NON COMMENT\n\nAFTER NON COMMENT"
        );
    }

    #[test]
    fn compress_whitespace() {
        assert_eq!(clean("  "), " ");
        assert_eq!(clean("\t \t"), " ");
        assert_eq!(clean("\t\n\t\n"), "\n");
        assert_eq!(clean("\t\tclass Main {};\t\t\n"), " class Main {};\n");
    }

    #[test]
    fn ignore_string_constants() {
        assert_eq!(clean("\"  \t\""), "\"  \t\"");
        assert_eq!(
            clean("\"// SINGLE LINE COMMENT\""),
            "\"// SINGLE LINE COMMENT\""
        )
    }
}
