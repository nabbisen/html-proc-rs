use crate::omit_enclosure::manipulate;

pub fn reset_document_outline(output: &str, html: &str) -> String {
    let mut omit_tags = Vec::<&str>::new();
    // omits document outline tags if necessary
    if !html.contains("<html>") {
        omit_tags.push("html")
    };
    if !html.contains("<head>") {
        omit_tags.push("head")
    };
    if !html.contains("<body>") {
        omit_tags.push("body")
    };
    let omit_tags = omit_tags.as_slice();
    manipulate(output, omit_tags)
}
