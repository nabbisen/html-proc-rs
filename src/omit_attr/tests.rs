/// `cargo test --features omit_attr`
#[cfg(test)]
mod tests {
    use crate::omit_attr::manipulate;

    #[test]
    fn manipulate_attr_1() {
        let source = "<span style=\"remove: me;\">Content</span>";
        let omits = &["style"];
        let expect = "<span>Content</span>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_attr_2() {
        let source = "<span style=\"remove: me;\" class=\"keep-me\">Content</span>";
        let omits = &["style"];
        let expect = "<span class=\"keep-me\">Content</span>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_attr_3() {
        let source = "<span style=\"remove: me;\" class=\"keep-me\">Content</span>";
        let omits = &["*.style"];
        let expect = "<span class=\"keep-me\">Content</span>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_tag_attr_1() {
        let source = "<span style=\"remove: me;\" class=\"keep-me\">Content</span>";
        let omits = &["span.style"];
        let expect = "<span class=\"keep-me\">Content</span>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_tag_attr_2() {
        let source = "<span style=\"remove: me;\" class=\"keep-me\">Content</span>";
        let omits = &["a.style"];
        let expect = "<span style=\"remove: me;\" class=\"keep-me\">Content</span>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_tag_attr_3() {
        let source = "<span style=\"remove: me;\" class=\"keep-me\">Content</span>";
        let omits = &["span.style", "span.class"];
        let expect = "<span>Content</span>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_attr_and_tag_attr_1() {
        let source = "<span style=\"remove: me;\" class=\"keep-me\">Content</span>";
        let omits = &["*.style", "span.class"];
        let expect = "<span>Content</span>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_attr_and_tag_attr_2() {
        let source = "<main id=\"preserved\"><div style=\"remove: me;\" class=\"keep-me\" z-index=\"1\">Content</div></main>";
        let omits = &["style", "*.class", "div.z-index"];
        let expect = "<main id=\"preserved\"><div>Content</div></main>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }
}
