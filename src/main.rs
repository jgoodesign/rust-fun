// naively building some kind of data store that modifies data by passing events to reducer functions
// copying redux pattern
// trying to get handle on pattern matching, generic types, lifetimes, general typing issues

//our store struct
// lifetime 'a is required,
// we need to pass it down to the functions within the reducers Vec so that the compiler keeps them alive as long as the Store is alive
// accepts generic type T for state, means this should work with any state
struct Store<'a, T: 'a> {
    state: T,
    reducers: Vec<&'a Fn(Action, T) -> T>,
}

// T requires the Copy trait here in order fo rus to safely pass 'self.state' to the reducer function
// also tried to accomplish this without Copy and using Box but couldn't get it to work properly
// lifetime 'a is also required when impl the Store struct
impl<'a, T> Store<'a, T>
where
    T: Copy,
{
    fn init(s: T) -> Store<'a, T> {
        Store {
            state: s,
            reducers: vec![],
        }
    }

    fn add_reducer(&mut self, reducer: &'a Fn(Action, T) -> T) {
        self.reducers.push(reducer);
    }

    fn dispatch(&mut self, action: Action) {
        // could not get this to work with .iter().for_each
        // problem was getting the for_each closure to capture &self
        for reducer in &self.reducers {
            //this is why we need the Copy traits
            self.state = reducer(action, self.state);
        }
    }
}

//this needs Copy trait to work with Action
#[derive(Debug, Clone, Copy)]
enum EventType {
    Attack,
    Move,
}
// needs Copy trait to pass properly to reducer function
// I think Box should work here too?
#[derive(Debug, Clone, Copy)]
struct Action {
    event: EventType,
}
//needs Copy trait to staisfy Store where clause
#[derive(Debug, Clone, Copy)]
struct StoreData {
    a: i32,
    b: i32,
}

fn reducer(action: Action, state: StoreData) -> StoreData {
    // implicit return pattern matching!
    match action.event {
        EventType::Attack => StoreData {
            a: state.a + 2,
            //struct update syntax to copy over all other values
            //needs to go last, ignores any values already defined beforehand
            ..state
        },
        EventType::Move => StoreData {
            a: state.a + 4,
            ..state
        },
    }
}

fn main() {
    let x = add(5);
    println!("x is {}", x(5));
    let x_boxed = add_boxed(5);
    println!("x_boxed is {}", x_boxed(5));

    let def = StoreData { a: 1, b: 2 };

    let mut store = Store::init(def);

    println!("store state a is {}", store.state.a);
    Store::add_reducer(&mut store, &reducer);

    store.dispatch(Action {
        event: EventType::Attack,
    });

    println!("store state a is {}", store.state.a);

    store.dispatch(Action {
        event: EventType::Move,
    });

    println!("store state a is {}", store.state.a);
}

// higher order function working!
// no pointer
// 'impl' is required here in order to properly type the return closure, from docs:
/*
The other use of the impl keyword is in impl Trait syntax, which can be seen as a shorthand for "a concrete type that implements this trait".
Its primary use is working with closures, which have type definitions generated at compile time that can't be simply typed out.
*/
fn add(a: u32) -> impl Fn(u32) -> u32 {
    move |b: u32| -> u32 { a + b }
}

// same higher order function working with 'Box<Fn()>' type instead of 'impl Fn'
// from what I understand Box::new returns a function pointer to the closure and moves it to the heap
// this is probably required for more complicated closures
fn add_boxed(a: u32) -> Box<Fn(u32) -> u32> {
    Box::new(move |b: u32| -> u32 { a + b })
}
