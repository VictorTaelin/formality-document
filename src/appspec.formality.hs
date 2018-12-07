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

data LocalEvent : Type
| MouseClick : (x : Uint, y : Uint) -> LocalEvent
| Keypress   : (k : Uint)           -> LocalEvent

data LocalReducer<State : Type> : Type
| new : (state : State, transact : (event : LocalEvent, state : State) -> State) -> LocalReducer

data App<State : Type> : Type
| new : (local_reducer : LocalReducer<State>, render : (state : State) -> Document) -> App

let get_app_local_state(State : Type, app : App<State>) =>
    case app -> State
    | new(local_reducer, render) => 
      case local_reducer -> State
      | new(state, transact) => state

let get_app_local_transact(State : Type, app : App<State>) =>
    case app -> () => (event : LocalEvent, state : State) -> State
    | new(local_reducer, render) =>
      case local_reducer -> () => (event : LocalEvent, state : State) -> State
      | new(state, transact) => transact

let get_app_render(State : Type, app : App<State>) =>
    case app -> () => (state : State) -> Document
    | new(local_reducer, render) => render

let compute_local_state
    ( State : Type
    , local_reducer : LocalReducer<State>
    , local_events : List<LocalEvent>) =>
    case local_reducer        -> State
    | new(state, transact)    =>
        case local_events     -> State
        | cons(event, state)  => fold(state)
        | nil                 => state

let inc(n : Uint) =>
    case n -> Uint
    | O(n) => Uint.I(n)
    | I(n) => Uint.O(fold(n))
    | Z    => Uint.Z

let id(n : Uint) =>
    case n -> Uint
    | O(n) => Uint.O(fold(n))
    | I(n) => Uint.I(fold(n))
    | Z    => Uint.Z

let uint(n : (P : Type, S : (x : P) -> P, Z : P) -> P) =>
    let fuse_inc(n : Uint) => Uint{
        case n -> Uint
        | O(n) => I(n)
        | I(n) => O(fuse_inc(n))
        | Z    => Z
    }

    id(n(Uint, fuse_inc, 32(Uint, Uint.O, Uint.Z)))

-- Demo App:
-- - Renders a circle on the middle of the screen
-- - Size increases every time the user clicks or presses a key
let DemoLocalState      Uint
let DemoLocalReducer    LocalReducer<DemoLocalState>
let DemoApp             App<DemoLocalState>
let demo_local_event    LocalEvent.MouseClick(uint(0), uint(0))
let demo_local_state    uint(16)
let demo_local_transact (e : LocalEvent, s : DemoLocalState) => id(inc(s))
let demo_local_reducer  DemoLocalReducer.new(demo_local_state, demo_local_transact)
let demo_render         (s : DemoLocalState) => Document.cons(Element.circle(uint(16), uint(16), s), Document.nil)
let demo_app            DemoApp.new(demo_local_reducer, demo_render)

let main get_app_local_state(DemoLocalState, demo_app)
