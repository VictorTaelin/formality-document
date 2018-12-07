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
    let defs = &build_defs(Some(b"
        let pretty_circle
            Element.circle(uint(20), uint(20), uint(10))

        let pretty_square
            Element.square(uint(40), uint(40), uint(10))

        let main
            Document.cons(pretty_circle,
            Document.cons(pretty_square,
            Document.nil))
    "));
    let get = |name| get_term(name, defs); // convenience to get terms
    let apply = |fun, args| apply(fun, args, defs); // convenience to apply terms 
    let document = term_to_document(&get_term_reduced(b"main", defs));
    println!("{:?}", document);

    // Tests `serialize` trait from `Document`
    let serialized = serde_json::to_string(&document).unwrap();
    println!("serialized = {}", serialized);

    // Tests `Deserialize` trait from `Document`
    let deserialized : Document = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    // Builds a demo app, gets its fields, computes some states, renders docs
    let app         = get(b"demo_app");
    let inistate    = apply(get(b"get_app_local_inistate"), vec![app.clone()]);
    let transact    = apply(get(b"get_app_local_transact"), vec![app.clone()]);
    let render      = apply(get(b"get_app_render"), vec![app.clone()]);
    let local_event = get(b"demo_local_event");
    let state_0     = inistate.clone();
    let state_1     = apply(transact.clone(), vec![local_event.clone(), state_0.clone()]);
    let state_2     = apply(transact.clone(), vec![local_event.clone(), state_1.clone()]);
    let f_doc_0     = apply(render.clone(), vec![state_0.clone()]);
    let f_doc_1     = apply(render.clone(), vec![state_1.clone()]);
    let f_doc_2     = apply(render.clone(), vec![state_2.clone()]);
    println!("state 0 = {}", state_0);
    println!("state 1 = {}", state_1);
    println!("state 2 = {}", state_2);
    println!("f.doc 0 = {:?}", term_to_document(&f_doc_0));
    println!("f.doc 1 = {:?}", term_to_document(&f_doc_1));
    println!("f.doc 2 = {:?}", term_to_document(&f_doc_2));
}
