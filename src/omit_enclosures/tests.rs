/// `cargo test --features omit_enclosures`
#[cfg(test)]
mod tests {
    use crate::omit_enclosures::manipulate;

    #[test]
    fn manipulate_1() {
        let source = "<div><span>Remove me</span><p>Keep me</p><span>And me</span></div>";
        let omits = &["span"];
        let expect = "<div>Remove me<p>Keep me</p>And me</div>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_2() {
        let source = "<div><span>Remove me</span><p>Keep me</p><span>And me</span></div>";
        let omits = &["p"];
        let expect = "<div><span>Remove me</span>Keep me<span>And me</span></div>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_3() {
        let source = "<div><span>Remove me</span><p>Keep me</p><span>And me</span></div>";
        let omits = &["span", "p"];
        let expect = "<div>Remove meKeep meAnd me</div>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_4() {
        let source = "<div><span>Remove me</span><p>Keep me</p><span>And me</span></div>";
        let omits = &["span", "i"];
        let expect = "<div>Remove me<p>Keep me</p>And me</div>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn manipulate_5() {
        let source = "<div><span>Remove me</span><p>Keep me</p><span>And me</span></div>";
        let omits = &["span", "p", "div"];
        let expect = "Remove meKeep meAnd me";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn html_outline_1() {
        let source =
            "<body><div><span>Remove me</span><p>Keep me</p><span>And me</span></div></body>";
        let omits = &["span"];
        let expect = "<body><div>Remove me<p>Keep me</p>And me</div></body>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn html_outline_2() {
        let source = "<html><body><div><span>Remove me</span><p>Keep me</p><span>And me</span></div></body></html>";
        let omits = &["span"];
        let expect = "<html><body><div>Remove me<p>Keep me</p>And me</div></body></html>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn html_outline_3() {
        let source = "<html><head></head><body><div><span>Remove me</span><p>Keep me</p><span>And me</span></div></body></html>";
        let omits = &["span"];
        let expect =
            "<html><head></head><body><div>Remove me<p>Keep me</p>And me</div></body></html>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn attrs_preserved_1() {
        let source = "<div id=\"mydiv\" class=\"mydiv\"><span style=\"color: yellow;\">Remove me</span><p tabindex=\"10\">Keep me</p><span>And me</span></div>";
        let omits = &[];
        let expect = "<div id=\"mydiv\" class=\"mydiv\"><span style=\"color: yellow;\">Remove me</span><p tabindex=\"10\">Keep me</p><span>And me</span></div>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }

    #[test]
    fn attrs_preserved_2() {
        let source =
            "<div id=\"mydiv\" class=\"mydiv\"  tabindex=\"1\"><span>Remove me</span></div>";
        let omits = &["span"];
        let expect = "<div id=\"mydiv\" class=\"mydiv\" tabindex=\"1\">Remove me</div>";

        let result = manipulate(source, omits);
        assert_eq!(result, expect);
    }
}
