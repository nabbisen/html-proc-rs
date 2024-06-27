/// `cargo test --features path_to_url`
#[cfg(test)]
mod tests {
    use crate::path_to_url::{convert, ConvertOptions, ConvertTag};

    #[test]
    fn convert_a_1() {
        let source = "<a href=\"/somewhere\">link</a>";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn);
        let expect = "<a href=\"https://some.domain/somewhere\">link</a>";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn convert_a_2() {
        let source = "<a href=\"/somewhere\">link</a>";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn).set_current_path("/some/where");
        let expect = "<a href=\"https://some.domain/somewhere\">link</a>";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn convert_a_3() {
        let source = "<a href=\"../somewhere\">link</a>";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn).set_current_path("/some/where");
        let expect = "<a href=\"https://some.domain/some/where/../somewhere\">link</a>";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn convert_img_1() {
        let source = "<img src=\"/some.file\">";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn).set_tags(Vec::from_iter([ConvertTag::Img]));
        let expect = "<img src=\"https://some.domain/some.file\">";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn convert_audio_1() {
        let source = "<audio src=\"/some.file\"></audio>";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn).set_tags(Vec::from_iter([ConvertTag::Audio]));
        let expect = "<audio src=\"https://some.domain/some.file\"></audio>";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn convert_video_1() {
        let source = "<video src=\"/some.file\"></video>";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn).set_tags(Vec::from_iter([ConvertTag::Video]));
        let expect = "<video src=\"https://some.domain/some.file\"></video>";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn convert_source_1() {
        let source = "<audio><source src=\"/some.file\"></audio>";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn).set_tags(Vec::from_iter([ConvertTag::Source]));
        let expect = "<audio><source src=\"https://some.domain/some.file\"></audio>";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn convert_link_1() {
        let source = "<link href=\"/some.file\">";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn).set_tags(Vec::from_iter([ConvertTag::Link]));
        let expect = "<link href=\"https://some.domain/some.file\">";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn convert_script_1() {
        let source = "<script src=\"/some.file\"></script>";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn).set_tags(Vec::from_iter([ConvertTag::Script]));
        let expect = "<script src=\"https://some.domain/some.file\"></script>";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn document_outline_1() {
        let source = "<html><head></head><body><a href=\"/somewhere\">link</a></body></html>";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn);
        let expect = "<html><head></head><body><a href=\"https://some.domain/somewhere\">link</a></body></html>";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn skipped_1() {
        let source = "<a href=\"../somewhere\">link</a>";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn).set_tags(Vec::from_iter([ConvertTag::Img]));
        let expect = "<a href=\"../somewhere\">link</a>";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }

    #[test]
    fn skipped_2() {
        let source = "<a href=\"../somewhere\">link</a>
<img src=\"/some.file\">
<audio src=\"/some.file\"></audio>
<video src=\"/some.file\"></video>
<audio><source src=\"/some.file\"></audio>
<link href=\"/some.file\">
<script src=\"/some.file\"></script>
";
        let fqdn = "some.domain";
        let options = ConvertOptions::new(fqdn).set_tags(Vec::new());
        let expect = "<a href=\"../somewhere\">link</a>
<img src=\"/some.file\">
<audio src=\"/some.file\"></audio>
<video src=\"/some.file\"></video>
<audio><source src=\"/some.file\"></audio>
<link href=\"/some.file\">
<script src=\"/some.file\"></script>
";

        let result = convert(source, &options);
        assert_eq!(result, expect);
    }
}
