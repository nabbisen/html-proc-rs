use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, ParseOpts};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

use std::default::Default;

use crate::consts::SELF_CLOSING_TAGS;

mod tests;

/// omits specific enclosures of tags in html
///
/// [feature entry point]
///
/// ```rust
/// use htmlproc::omit_enclosure::manipulate;
///
/// let source: &str = "<div>outside <span>inside</span>\n</div>";
/// let target_tags: &[&str;1] = &["span"];
/// let expect: &str = "<div>outside inside\n</div>";
///
/// let result = manipulate(source, target_tags);
/// assert_eq!(result, expect);
/// ```
///
pub fn manipulate(html: &str, omit_tags: &[&str]) -> String {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    let mut omit_tags = Vec::from(omit_tags);
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

    let mut output = String::new();
    scan(&dom.document, omit_tags, &mut output);

    output
}

/// scan to manipulate dom recursively
fn scan(handle: &Handle, omit_tags: &[&str], output: &mut String) {
    let node = handle;

    match &node.data {
        NodeData::Document => {
            for child in node.children.borrow().iter() {
                scan(child, omit_tags, output);
            }
        }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let name_local = name.local.as_ref();
            if !omit_tags.contains(&name_local) {
                // start tag
                output.push('<');
                output.push_str(name_local);

                let attrs = attrs
                    .clone()
                    .into_inner()
                    .iter()
                    .map(|x| format!(" {}=\"{}\"", x.name.local, x.value))
                    .collect::<Vec<String>>()
                    .join("");
                output.push_str(attrs.as_str());

                output.push('>');
            }

            for child in node.children.borrow().iter() {
                scan(child, omit_tags, output);
            }

            if !SELF_CLOSING_TAGS.contains(&name_local) && !omit_tags.contains(&name_local) {
                // end tag
                output.push_str("</");
                output.push_str(name_local);
                output.push('>');
            }
        }
        NodeData::Text { ref contents } => {
            output.push_str(&contents.borrow());
        }
        _ => {}
    }
}
