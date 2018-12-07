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

data App : Type
| new : 
  ( LocalState     : Type
  , local_inistate : LocalState
  , local_transact : (event : LocalEvent, state : LocalState) -> LocalState
  , render         : (state : LocalState) -> Document)
  -> App

let get_app_LocalState(app : App) =>
  case app -> Type
  | new(LocalState, local_inistate, local_transact, render) => LocalState

let get_app_local_inistate(app : App) =>
  case app -> () => get_app_LocalState(self)
  | new(LocalState, local_inistate, local_transact, render) => local_inistate

let get_app_local_transact(app : App) =>
  case app -> () => (event : LocalEvent, state : get_app_LocalState(self)) -> get_app_LocalState(self)
  | new(LocalState, local_inistate, local_transact, render) => local_transact

let get_app_render(app : App) =>
  case app -> () => (state : get_app_LocalState(self)) -> Document
  | new(LocalState, local_inistate, local_transact, render) => render

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
let demo_local_inistate uint(16)
let demo_local_transact (e : LocalEvent, s : DemoLocalState) => id(inc(s))
let demo_render         (s : DemoLocalState) => Document.cons(Element.circle(uint(16), uint(16), s), Document.nil)
let demo_app            App.new(DemoLocalState, demo_local_inistate, demo_local_transact, demo_render)
let demo_local_event    LocalEvent.MouseClick(uint(0), uint(0))

let main get_app_local_inistate(demo_app)
