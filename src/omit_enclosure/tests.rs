/// `cargo test`
#[cfg(test)]
mod tests {
    use crate::omit_enclosure::manipulate;

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
    fn manipulate_6() {
        let source = r#"<!DOCTYPE html><span style="caret-color: rgb(10, 10, 10); color: rgb(10, 10, 10); font-family: Arial, sans-serif; font-size: 14.036885px; font-style: normal; font-variant-caps: normal; font-weight: 400; letter-spacing: normal; orphans: auto; text-align: left; text-indent: 0px; text-transform: none; white-space: normal; widows: auto; word-spacing: 0px; -webkit-tap-highlight-color: rgba(0, 0, 0, 0.4); -webkit-text-stroke-width: 0px; background-color: rgb(254, 254, 254); text-decoration: none; display: inline !important; float: none;">To avoid polluting the<span class="Apple-converted-space"> </span></span><a href="https://github.com/" title="Click to open https://github.com/" style="font-family: Arial, sans-serif; font-size: 14.036885px; font-style: normal; font-variant-caps: normal; letter-spacing: normal; orphans: auto; text-align: left; text-indent: 0px; text-transform: none; white-space: normal; widows: auto; word-spacing: 0px; -webkit-tap-highlight-color: rgba(0, 0, 0, 0.4); -webkit-text-stroke-width: 0px; text-decoration: none; font-weight: bold; color: rgb(0, 102, 153);">some text<span class="Apple-converted-space"> </span></a><span style="caret-color: rgb(10, 10, 10); color: rgb(10, 10, 10); font-family: Arial, sans-serif; font-size: 14.036885px; font-style: normal; font-variant-caps: normal; font-weight: 400; letter-spacing: normal; orphans: auto; text-align: left; text-indent: 0px; text-transform: none; white-space: normal; widows: auto; word-spacing: 0px; -webkit-tap-highlight-color: rgba(0, 0, 0, 0.4); -webkit-text-stroke-width: 0px; background-color: rgb(254, 254, 254); text-decoration: none; display: inline !important; float: none;"></span><span style="caret-color: rgb(10, 10, 10); color: rgb(10, 10, 10); font-family: Arial, sans-serif; font-size: 14.036885px; font-style: normal; font-variant-caps: normal; font-weight: 400; letter-spacing: normal; orphans: auto; text-align: left; text-indent: 0px; text-transform: none; white-space: normal; widows: auto; word-spacing: 0px; -webkit-tap-highlight-color: rgba(0, 0, 0, 0.4); -webkit-text-stroke-width: 0px; text-decoration: none;">#50105</span><span style="caret-color: rgb(10, 10, 10); color: rgb(10, 10, 10); font-family: Arial, sans-serif; font-size: 14.036885px; font-style: normal; font-variant-caps: normal; font-weight: 400; letter-spacing: normal; orphans: auto; text-align: left; text-indent: 0px; text-transform: none; white-space: normal; widows: auto; word-spacing: 0px; -webkit-tap-highlight-color: rgba(0, 0, 0, 0.4); -webkit-text-stroke-width: 0px; background-color: rgb(254, 254, 254); text-decoration: none; display: inline !important; float: none;"><span class="Apple-converted-space"> </span>trailing text</span>"#;
        let omits = &["span"];
        let expect = r#"To avoid polluting the <a href="https://github.com/" title="Click to open https://github.com/" style="font-family: Arial, sans-serif; font-size: 14.036885px; font-style: normal; font-variant-caps: normal; letter-spacing: normal; orphans: auto; text-align: left; text-indent: 0px; text-transform: none; white-space: normal; widows: auto; word-spacing: 0px; -webkit-tap-highlight-color: rgba(0, 0, 0, 0.4); -webkit-text-stroke-width: 0px; text-decoration: none; font-weight: bold; color: rgb(0, 102, 153);">some text </a>#50105 trailing text"#;

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
