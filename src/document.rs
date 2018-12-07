extern crate formality;

use self::formality::term::*;
use self::formality::term::Term::*;

// Document type on Formality
pub static FORMALITY_HEADER : &[u8] = include_bytes!("appspec.formality.hs");

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

// Builds a Formality Defs using the AppSpec headers plus some extra code (possibly a DApp)
pub fn build_defs(extra_code : Option<&[u8]>) -> formality::term::Defs { 
    // Builds source code with proper headers
    let mut code = FORMALITY_HEADER.to_vec();
    if let Some(extra_code) = extra_code {
        code.extend_from_slice(b"\n");
        code.extend_from_slice(&mut extra_code.clone());
    };
    code.extend_from_slice(b"\nType");

    // Parses it to get a list of definitions
    let parsed = formality::syntax::term_from_ascii(code);
    let (_, defs) = (match parsed { Ok(res) => Some(res), Err(_) => None }).unwrap();

    defs
}

// Gets a term from a Formality Defs
pub fn get_term(name : &[u8], defs : &formality::term::Defs) -> formality::term::Term {
    defs.get(name).unwrap().clone()
}

// Gets a term from a Formality Defs in normal form
pub fn get_term_reduced(name : &[u8], defs : &formality::term::Defs) -> formality::term::Term {
    formality::term::reduced(&get_term(name, defs), defs, true)
}

// Applies a term to a list of term arguments, returns the normal form (interpreted)
pub fn apply(fun : Term, args : Vec<Term>, defs : &formality::term::Defs) -> Term {
    let mut fun = fun.clone();
    for i in 0..args.len() {
        let arg = args[i].clone();
        fun = formality::term::Term::App{
            fun: Box::new(fun),
            arg: Box::new(arg)
        };
    }
    reduced(&fun, defs, true)
}
