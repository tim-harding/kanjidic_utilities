use quick_xml::{Reader, events::Event};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KdpError {
    #[error("Error parsing XML file")]
    Parse(#[from] quick_xml::Error)
}

pub fn stuff(xml: &str) {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(event) => match event {
                Event::Start(e) => println!("Start {:?}", e),
                Event::Text(e) => println!("Text {:?}", e),
                Event::Eof => break,
                Event::End(_) => todo!(),
                Event::Empty(_) => todo!(),
                Event::Comment(_) => todo!(),
                Event::CData(_) => todo!(),
                Event::Decl(_) => todo!(),
                Event::PI(_) => todo!(),
                Event::DocType(_) => todo!(),
            }
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        buf.clear();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
