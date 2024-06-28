use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, ParseOpts};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

use crate::core::utils::reset_document_outline;

mod tests;

struct OmitOptions {
    attrs: Vec<String>,
    tag_attrs: Vec<(String, String)>,
}

/// omits specific attributes of tags in html
///
/// [feature entry point]
///
/// ```rust
/// use htmlproc::omit_attr::manipulate;
///
/// let source: &str = "<div id=\"preserved\"><span style=\"want: omitted;\" class=\"also: wanted;\" z-index=\"1\">Content</span></div>";
/// let omit_attrs = &["style", "*.class", "span.z-index"];
/// let expect: &str = "<div id=\"preserved\"><span>Content</span></div>";
///
/// let result = manipulate(source, omit_attrs);
/// assert_eq!(result, expect);
/// ```
///
pub fn manipulate(html: &str, omit_attrs: &[&str]) -> String {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    let options = options(omit_attrs);

    let mut output = String::new();
    scan(&dom.document, &options, &mut output);

    reset_document_outline(output.as_str(), html)
}

fn options(omit_attrs: &[&str]) -> OmitOptions {
    if omit_attrs
        .iter()
        .any(|x| 1 < x.chars().filter(|y| *y == '.').count())
    {
        panic!("Invalid omit_attrs: each item should have single or none of \".\"");
    }

    let attr_options = omit_attrs
        .iter()
        .filter(|&&x| !x.contains('.') || x.starts_with("*."))
        .map(|&x| {
            let ret = if x.starts_with("*.") { &x[2..] } else { x };
            ret.to_owned()
        })
        .collect();
    let tag_attr_options = omit_attrs
        .iter()
        .filter(|&&x| x.contains('.') && !x.starts_with("*."))
        .map(|x| {
            let splitted = x.split('.').collect::<Vec<&str>>();
            let tag_name = splitted[0];
            let attr_name = splitted[1];
            if tag_name.is_empty() || attr_name.is_empty() {
                panic!("Invalid tag-attr pair found: {} - {}", tag_name, attr_name);
            }
            (tag_name.to_owned(), attr_name.to_owned())
        })
        .collect();
    let options = OmitOptions {
        attrs: attr_options,
        tag_attrs: tag_attr_options,
    };
    options
}

/// scan to manipulate dom recursively
fn scan(handle: &Handle, options: &OmitOptions, output: &mut String) {
    let node = handle;

    match &node.data {
        NodeData::Document => {
            for child in node.children.borrow().iter() {
                scan(child, options, output);
            }
        }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let tag_name = name.local.to_string();

            // start tag
            output.push('<');
            output.push_str(tag_name.as_str());

            let attrs = attrs
                .clone()
                .into_inner()
                .iter()
                .filter(|x| {
                    let attr_name = x.name.local.to_string();
                    !options.attrs.contains(&attr_name)
                        && !options
                            .tag_attrs
                            .contains(&(tag_name.to_owned(), attr_name))
                })
                .map(|x| format!(" {}=\"{}\"", x.name.local, x.value))
                .collect::<Vec<String>>()
                .join("");
            output.push_str(attrs.as_str());

            output.push('>');

            for child in node.children.borrow().iter() {
                scan(child, options, output);
            }

            // end tag
            output.push_str("</");
            output.push_str(tag_name.as_str());
            output.push('>');
        }
        NodeData::Text { ref contents } => {
            output.push_str(&contents.borrow());
        }
        _ => {}
    }
}
