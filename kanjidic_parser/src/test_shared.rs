use roxmltree::Document;

lazy_static! {
    pub static ref DOC: Document<'static> = {
        let file = include_bytes!("../../assets/kanjidic2.xml");
        let xml = std::str::from_utf8(file).unwrap();
        let start = xml.find("<kanjidic2>").unwrap();
        let skipped = std::str::from_utf8(&xml.as_bytes()[start..]).unwrap();
        roxmltree::Document::parse(skipped).unwrap()
    };
}
