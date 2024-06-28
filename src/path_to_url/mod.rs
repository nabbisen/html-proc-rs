use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, ParseOpts};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

use std::path::Path;

use crate::core::utils::reset_document_outline;

mod tests;

/// URL HTTP protocol
pub enum HttpProtocol {
    Http,
    Https,
}
/// conversion target tag
pub enum ConvertTag {
    A,
    Img,
    Audio,
    Video,
    Source,
    Link,
    Script,
}
// conversion target attr of each tag
const CONVERT_TAG_ATTRS: [(&str, &str); 7] = [
    ("a", "href"),
    ("img", "src"),
    ("audio", "src"),
    ("video", "src"),
    ("source", "src"),
    ("link", "href"),
    ("script", "src"),
];

/// conversion options
pub struct ConvertOptions {
    /// HTTP protocol
    http_protocol: HttpProtocol,
    /// FQDN (host name, domain)
    fqdn: String,
    /// port number. Default: 443 on HTTPS, 80 on HTTP
    port: u16,
    /// path base. Default: /
    current_path: String,
    /// target tags
    tags: Vec<ConvertTag>,
}
impl ConvertOptions {
    pub fn new(fqdn: &str) -> Self {
        ConvertOptions {
            http_protocol: HttpProtocol::Https,
            fqdn: fqdn.to_owned(),
            port: 443 as u16,
            current_path: String::from("/"),
            tags: Vec::from_iter([ConvertTag::A]),
        }
    }
    pub fn set_http_protocol(mut self, http_protocol: HttpProtocol) -> Self {
        self.http_protocol = http_protocol;
        self
    }
    pub fn set_fqdn(mut self, fqdn: &str) -> Self {
        self.fqdn = fqdn.to_owned();
        self
    }
    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    pub fn set_current_path(mut self, current_path: &str) -> Self {
        self.current_path = current_path.to_owned();
        self
    }
    pub fn set_tags(mut self, tags: Vec<ConvertTag>) -> Self {
        self.tags = tags;
        self
    }
}

/// internal options made of specified options
struct ActualConvertOptions<'a> {
    url_prefix: &'a str,
    current_path: &'a str,
    tags: &'a Vec<&'a str>,
}

/// convert paths in html to urls
///
/// [feature entry point]
///
/// ```rust
/// use htmlproc::path_to_url::{convert, ConvertOptions};
///
/// let source: &str = "<a href=\"/somewhere\">link</a>";
/// let fqdn: &str = "some.domain";
/// let options: ConvertOptions = ConvertOptions::new(fqdn);
/// let expect: &str = "<a href=\"https://some.domain/somewhere\">link</a>";
///
/// let result = convert(source, &options);
/// assert_eq!(result, expect);
/// ```
///
pub fn convert(html: &str, options: &ConvertOptions) -> String {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    let url_prefix = match (&options.http_protocol, options.port) {
        (HttpProtocol::Https, 443) => format!("https://{}", options.fqdn),
        (HttpProtocol::Http, 80) => format!("http://{}", options.fqdn),
        _ => format!(
            "{}://{}",
            match &options.http_protocol {
                HttpProtocol::Http => "http",
                _ => "https",
            },
            options.fqdn
        ),
    };
    let mut tags = Vec::<&str>::new();
    options.tags.iter().for_each(|x| match x {
        ConvertTag::A => tags.push("a"),
        ConvertTag::Img => tags.push("img"),
        ConvertTag::Audio => tags.push("audio"),
        ConvertTag::Video => tags.push("video"),
        ConvertTag::Source => tags.push("source"),
        ConvertTag::Link => tags.push("link"),
        ConvertTag::Script => tags.push("script"),
    });
    let actual_option = ActualConvertOptions {
        url_prefix: &url_prefix,
        current_path: &options.current_path,
        tags: &tags,
    };

    let mut output = String::new();
    scan(&dom.document, &actual_option, &mut output);

    reset_document_outline(output.as_str(), html)
}

/// scan to convert recursively
fn scan(handle: &Handle, options: &ActualConvertOptions, output: &mut String) {
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
            let tag_name = name.local.as_ref();

            let is_convert_tag = options.tags.contains(&tag_name);

            // start tag
            output.push('<');
            output.push_str(tag_name);

            let attrs = attrs
                .clone()
                .into_inner()
                .iter()
                // tag attrs
                .map(|x| {
                    let attr_name = x.name.local.to_string();
                    let mut attr_value = x.value.to_string();
                    // path conversion
                    if is_convert_tag
                        && CONVERT_TAG_ATTRS.contains(&(tag_name, attr_name.as_str()))
                        && !attr_value.contains("//")
                    {
                        // absolute path
                        if attr_value.starts_with('/') {
                            attr_value = format!("{}{}", options.url_prefix, attr_value);
                        // relative path
                        } else {
                            let base_path = Path::new(options.current_path);
                            let source_path = Path::new(&attr_value);
                            let joined = base_path.join(source_path);
                            let path = joined.display();
                            attr_value = format!("{}{}", options.url_prefix, path);
                        }
                    };
                    format!(" {}=\"{}\"", attr_name, attr_value)
                })
                .collect::<Vec<_>>()
                .join("");
            output.push_str(attrs.as_str());

            output.push('>');

            for child in node.children.borrow().iter() {
                scan(child, options, output);
            }

            // end tag
            output.push_str("</");
            output.push_str(tag_name);
            output.push('>');
        }
        NodeData::Text { ref contents } => {
            output.push_str(&contents.borrow());
        }
        _ => {}
    }
}
