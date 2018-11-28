extern crate formality_document as fd;
use fd::document::*;
use fd::formality::syntax;

extern crate serde_json;

fn main() {
    // Tests printing the value of `Document`
    let formality_types = syntax::term_from_ascii(FORMALITY_TYPES.to_vec()).unwrap().1;
    println!("Hello, world! {}", formality_types.get(&b"Document".to_vec()).unwrap());

    // Tests an example Document in Rust
    let document : Document = vec![
        Element::Circle{x: 50, y: 50, r: 20},
        Element::Square{x: 50, y: 50, r: 20}
    ];
    println!("{:?}", document);

    // Tests `serialize` trait from `Document`
    let serialized = serde_json::to_string(&document).unwrap();
    println!("serialized = {}", serialized);

    // Tests `Deserialize` trait from `Document`
    let deserialized: Document = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
