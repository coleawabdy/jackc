use regex::Regex;

lazy_static! {
    static ref RE_ANNOTATE: Regex = Regex::new(
        r#"(?mx)
		(?P<cs>//.*$)
		|
		(?s)(?P<cm>/\*.*\*/)
		|
		(?-s)(?P<str>".*?")
		|
		(?P<ws>\s+)
		|
		(?P<none>\S+)"#
    )
    .expect("Failed to build annotation regex");
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Annotation {
    Comment,
    String,
    Whitespace,
}

#[allow(dead_code)]
fn annotate(source: &str) -> Vec<(&str, Option<Annotation>)> {
    let mut annotations = Vec::<(&str, Option<Annotation>)>::new();

    // Map capture groups to specific annotation
    for caps in RE_ANNOTATE.captures_iter(source) {
        if let Some(m) = caps.name("cs") {
            annotations.push((m.as_str(), Some(Annotation::Comment)));
        } else if let Some(m) = caps.name("cm") {
            annotations.push((m.as_str(), Some(Annotation::Comment)));
        } else if let Some(m) = caps.name("str") {
            annotations.push((m.as_str(), Some(Annotation::String)));
        } else if let Some(m) = caps.name("ws") {
            annotations.push((m.as_str(), Some(Annotation::Whitespace)));
        } else if let Some(m) = caps.name("none") {
            annotations.push((m.as_str(), None));
        }
    }

    annotations
}

#[allow(dead_code)]
pub fn clean(source: &str) -> String {
    let mut cleaned_src = String::new();
    cleaned_src.reserve(source.len());

    let mut last_annotation: Option<Annotation> = None;
    for annotation in annotate(source) {
        cleaned_src += match annotation.1 {
            None | Some(Annotation::String) => {
                last_annotation = annotation.1;
                annotation.0
            }
            Some(Annotation::Comment) => "",
            Some(Annotation::Whitespace) => {
                if last_annotation != Some(Annotation::Whitespace) {
                    last_annotation = Some(Annotation::Whitespace);
                    " "
                } else {
                    ""
                }
            }
        }
    }

    // Remove trailing whitespace
    if last_annotation == Some(Annotation::Whitespace) {
        cleaned_src.remove(cleaned_src.len() - 1);
    }

    cleaned_src
}

#[cfg(test)]
mod tests {
    use super::{annotate, clean, Annotation};

    #[test]
    fn annotate_string() {
        assert_eq!(
            annotate("\"TEST STRING\""),
            vec![("\"TEST STRING\"", Some(Annotation::String))]
        );
        assert_eq!(
            annotate("\"STR1\"\"STR2\""),
            vec![
                ("\"STR1\"", Some(Annotation::String)),
                ("\"STR2\"", Some(Annotation::String))
            ]
        );
        assert_eq!(annotate("\"UNTERMINATED"), vec![("\"UNTERMINATED", None)]);
        assert_eq!(
            annotate("\"STRING\nNEWLINE\""),
            vec![
                ("\"STRING", None),
                ("\n", Some(Annotation::Whitespace)),
                ("NEWLINE\"", None)
            ]
        );
    }

    #[test]
    fn annotate_comment() {
        assert_eq!(
            annotate("// SINGLE LINE COMMENT\n"),
            vec![
                ("// SINGLE LINE COMMENT", Some(Annotation::Comment)),
                ("\n", Some(Annotation::Whitespace))
            ]
        );
        assert_eq!(
            annotate("/* MULTI\n LINE \n COMMENT */"),
            vec![("/* MULTI\n LINE \n COMMENT */", Some(Annotation::Comment))]
        )
    }

    #[test]
    fn annotate_whitespace() {
        assert_eq!(
            annotate(" \n\n"),
            vec![(" \n\n", Some(Annotation::Whitespace))]
        );
        assert_eq!(
            annotate("class Main {\n"),
            vec![
                ("class", None),
                (" ", Some(Annotation::Whitespace)),
                ("Main", None),
                (" ", Some(Annotation::Whitespace)),
                ("{", None),
                ("\n", Some(Annotation::Whitespace))
            ]
        )
    }

    #[test]
    fn annotate_composite() {
        assert_eq!(
            annotate("\"// COMMENT\""),
            vec![("\"// COMMENT\"", Some(Annotation::String))]
        );
        assert_eq!(
            annotate("/*\"INNTER STRING\"*/"),
            vec![("/*\"INNTER STRING\"*/", Some(Annotation::Comment))]
        );
    }

    #[test]
    fn remove_single_line_comments() {
        assert_eq!(clean("// SINGLE LINE COMMENT"), "");
        assert_eq!(
            clean("NON COMMENT\n// SINGLE LINE COMMENT\n"),
            "NON COMMENT"
        );
        assert_eq!(clean("NOT A COMMENT"), "NOT A COMMENT");
        assert_eq!(
            clean("NOT A COMMENT\n// SINGLE LINE COMMENT\nADDITIONAL NOT A COMMENT"),
            "NOT A COMMENT ADDITIONAL NOT A COMMENT"
        );
    }

    #[test]
    fn remove_multi_line_comments() {
        assert_eq!(
            clean(
                "/* MULTI LINE COMMENT */
        "
            ),
            ""
        );
        assert_eq!(
            clean(
                " /* MULTI\nLINE\nCOMMENT\n*/
    "
            ),
            ""
        );
        assert_eq!(
            clean(
                "NON COMMENT\n /* MULTI LINE COMMENT\n*/
    "
            ),
            "NON COMMENT"
        );
        assert_eq!(
            clean(
                "BEFORE NON COMMENT\n /* MULTI\n LINE\n COMMENT\n*/
    \nAFTER NON COMMENT"
            ),
            "BEFORE NON COMMENT AFTER NON COMMENT"
        );
    }

    #[test]
    fn compress_whitespace() {
        assert_eq!(clean("  "), "");
        assert_eq!(clean("\t \t"), "");
        assert_eq!(clean("\t\n\t\n"), "");
        assert_eq!(clean("\t\tclass Main {};\t\t\n"), " class Main {};");
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
