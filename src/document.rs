extern crate formality;

use self::formality::term::*;
use self::formality::term::Term::*;

// Document type on Formality
pub static FORMALITY_HEADER : &[u8] = b"
    data Uint : Type
    | O : (x : Uint) -> Uint
    | I : (x : Uint) -> Uint
    | Z : Uint

    data List<A : Type> : Type
    | cons : (x : A, xs : List) -> List
    | nil  : List

    data Element : Type
    | circle : (x : Uint, y : Uint, r : Uint) -> Element
    | square : (x : Uint, y : Uint, r : Uint) -> Element

    let Document List<Element>

    let inc(n : Uint) => Uint{
        case n -> Uint
        | O(n) => I(n)
        | I(n) => O(inc(n))
        | Z    => Z
    }

    let id(n : Uint) =>
        case n -> Uint
        | O(n) => Uint.O(fold(n))
        | I(n) => Uint.I(fold(n))
        | Z    => Uint.Z

    let uint(n : (P : Type, S : (x : P) -> P, Z : P) -> P) =>
        id(n(Uint, inc, 32(Uint, Uint.O, Uint.Z)))
";

// Element type on Rust
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Element {
    Circle{x : u32, y : u32, r : u32},
    Square{x : u32, y : u32, r : u32}
}
use self::Element::{*};

// Document type on Rust
pub type Document = Vec<Element>;

// Interprets a normalized Formality term as a Rust document
pub fn term_to_document(term : &Term) -> Option<Document> {
    //println!("Converting fdoc to doc");
    fn build_document(term : &Term) -> Document {
        fn go(term : &Term, doc : &mut Document) {
            //println!("Parsing list of elements: {}", term);
            match term {
                New{idt: _, ctr: _, ref bod} => {
                    let (fun, args) = get_fun_args(bod);
                    //println!("Found ctor {} with {} args", fun, args.len());
                    match fun {
                        Var{idx} => {
                            if *idx == 1 {
                                //println!("That's a cons node.");
                                doc.push(build_element(&args[0]));
                                go(&args[1], doc);
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        let mut doc = Vec::new();
        go(term, &mut doc);
        doc
    }
    fn build_element(elem : &Term) -> Element {
        //println!("Building element: {}", elem);
        match elem {
            New{idt: _, ctr: _, ref bod} => {
                let (fun, args) = get_fun_args(bod);
                //println!("Found ctor {} with {} args", fun, args.len());
                match fun {
                    Var{idx} => {
                        match idx {
                            // Square
                            0 => {
                                //println!("That's a square.");
                                let x = build_uint(args[0]);
                                let y = build_uint(args[1]);
                                let r = build_uint(args[2]);
                                Square{x, y, r}
                            },
                            // Circle
                            1 => {
                                //println!("That's a circle.");
                                let x = build_uint(args[0]);
                                let y = build_uint(args[1]);
                                let r = build_uint(args[2]);
                                Circle{x, y, r}
                            },
                            _ => panic!("TODO error")
                        }
                    },
                    _ => panic!("TODO error")
                }
            },
            _ => panic!("TODO error")
        }
    }
    fn build_uint(term : &Term) -> u32 {
        //println!("Building uint: {}", term);
        fn go(term : &Term, add : u32) -> u32 {
            //println!("Building uint / go: {}, {}", term, add);
            match term {
                New{idt: _, ctr: _, ref bod} => {
                    let (fun, args) = get_fun_args(bod);
                    match fun {
                        Var{idx} => {
                            match idx {
                                0 => 0,
                                1 => add + go(args[0], add << 1),
                                2 => go(args[0], add << 1),
                                _ => panic!("TODO error")
                            }
                        },
                        _ => panic!("TODO error")
                    }
                },
                _ => panic!("TODO error")
            }
        }
        go(term, 1)
    }
    Some(build_document(term))
}

// Converts a Formality string to a Rust document
pub fn code_to_document(doc_code : &[u8]) -> Option<Document> {
    // Builds source code with proper headers
    let mut code = FORMALITY_HEADER.to_vec();
    code.extend_from_slice(b"\n");
    code.extend_from_slice(&mut doc_code.clone());
    code.extend_from_slice(b"\nType");

    // Parses it to get a list of definitions
    let parsed = formality::syntax::term_from_ascii(code);
    let (_, defs) = match parsed { Ok(res) => Some(res), Err(_) => None }?;

    // Gets and evaluates `main`
    let term = &defs.get(&b"main".to_vec())?;
    let (_, norm) = formality::compiler::eval(&term, &defs);

    // Converts to a document
    term_to_document(&norm)
}

