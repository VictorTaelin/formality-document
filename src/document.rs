extern crate formality;

// Document type on Formality
pub static FORMALITY_TYPES : &[u8] = b"
    data Uint : Type
    | O : (x : Uint) -> Uint
    | I : (x : Uint) -> Uint
    | E : Uint

    data List<A : Type> : Type
    | cons : (x : A, xs : List) -> List
    | nil  : List

    data Element : Type
    | circle : (x : Uint, y : Uint, r : Uint) -> Element
    | square : (x : Uint, y : Uint, r : Uint) -> Element

    let Document List<Element>

    Type
";

// Element type on Rust
#[derive(Debug)]
pub enum Element {
    Circle{x : u32, y : u32, r : u32},
    Square{x : u32, y : u32, r : u32}
}

// Document type on Rust
pub type Document = Vec<Element>;

// Reads a Formality document into a Rust document
pub fn fdoc_to_doc(_fdoc : &formality::term::Term) -> Document {
    panic!("TODO");
}
