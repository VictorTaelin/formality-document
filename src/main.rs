extern crate formality;
extern crate formality_document as fd;
use fd::document::*;

extern crate serde_json;

fn main() {
    // Prints an example Document in Rust
    let document : Document = vec![
        Element::Circle{x: 30, y: 30, r: 20},
        Element::Square{x: 60, y: 60, r: 20}
    ];
    println!("{:?}", document);

    // Prints a Document generated from Formality code
    let document : Document = code_to_document(b"
        let pretty_circle
            Element.circle(uint(20), uint(20), uint(10))

        let pretty_square
            Element.square(uint(40), uint(40), uint(10))

        let main
            Document.cons(pretty_circle,
            Document.cons(pretty_square,
            Document.nil))
    ").unwrap();
    println!("{:?}", document);

    // Tests `serialize` trait from `Document`
    let serialized = serde_json::to_string(&document).unwrap();
    println!("serialized = {}", serialized);

    // Tests `Deserialize` trait from `Document`
    let deserialized : Document = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
