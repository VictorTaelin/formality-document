extern crate formality;
use formality::syntax;

pub mod document;

fn main() {
    // Tests printing the value of `Document`
    let formality_types = syntax::term_from_ascii(document::FORMALITY_TYPES.to_vec()).unwrap().1;
    println!("Hello, world! {}", formality_types.get(&b"Document".to_vec()).unwrap());
}
