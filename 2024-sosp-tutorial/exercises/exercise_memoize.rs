// Verus tutorial - 'Advanced Topics' Exercise: computation memoization
//
// This file provides an implementation of a store to memoize the results of some computation.
// This exercise illustrates how Verus can handle interior mutability using lock invariants.
//
// Most of the implementation has been filled in for you. There are 2 blanks to fill in;
// search for 'EXERCISE' in this file.

use vstd::prelude::*;
use vstd::rwlock::{RwLock, RwLockPredicate};
use vstd::hash_map::HashMapWithView;
use std::sync::Arc;

mod memoization_exercise_internals;

verus!{
broadcast use vstd::std_specs::hash::group_hash_axioms;

//////// Import a function. Our objective is to memoize the results of this function.

use memoization_exercise_internals::{Args, Output};

/// Pure functional specification of the function to be memoized.
pub closed spec fn func(args: &Args) -> Output {
    memoization_exercise_internals::func(args)
}

/// Executable version of the function to be memoized.
/// The `ensures` clause guarantees that this computation is deterministic,
/// i.e., it returns the same value every time it is called for the same arguments.
pub fn expensive_function(args: &Args) -> (out: Output)
    ensures out == func(args),
{
    memoization_exercise_internals::expensive_function(args)
}

//////// Here we define the main data structure.

pub struct Memoizer {
    lock: RwLock<HashMapWithView<Args, Output>, Pred>,
}

// This is where we define our "lock invariant". The 'Pred' type is kind of a dummy type, here;
// in more complex cases, we might put actual data in here, but we don't need to for this
// exercise. Thus for the purposes of this file, 'Pred' only serves as a way to connect
// the lock invariant to the lock.
//
// See the RwLock docs for more information:
// https://verus-lang.github.io/verus/verusdoc/vstd/rwlock/struct.RwLock.html
struct Pred { }
impl RwLockPredicate<HashMapWithView<Args, Output>> for Pred {
    open spec fn inv(self, v: HashMapWithView<Args, Output>) -> bool {
        // EXERCISE: fill me in
        true
    }
}

impl Memoizer {
    /// Construct a new 'memoizer' object
    pub fn new() -> Self {
        Memoizer {
            lock: RwLock::new(HashMapWithView::<Args, Output>::new(), Ghost(Pred{})),
        }
    }

    /// Get the result of applying `expensive_function` to the given arguments.
    /// This should call `expensive_function` only if necessary, and it should store
    /// the output for future use.
    pub fn get(&self, args: &Args) -> (out: Output)
        ensures out == func(args)
    {
        // EXERCISE: fill me in
        //
        // Useful docs:
        //  - RwLock: https://verus-lang.github.io/verus/verusdoc/vstd/rwlock/struct.RwLock.html
        //  - HashMapWithView: https://verus-lang.github.io/verus/verusdoc/vstd/hash_map/struct.HashMapWithView.html

        todo()
    }
}

/// Example usage. Note that this will compile only if `Memoizer: Sync`.
fn main() {
    let memoizer = Memoizer::new();
    let shared_memoizer = Arc::new(memoizer);

    let shared_memoizer1 = shared_memoizer.clone();
    vstd::thread::spawn(move || {
        let v = (*shared_memoizer1).get(&7);
        assert(v == func(&7));
    });

    let shared_memoizer2 = shared_memoizer.clone();
    vstd::thread::spawn(move || {
        let v = (*shared_memoizer2).get(&20);
        assert(v == func(&20));
    });
}

// Used as a placeholder for the exercises, where necessary
#[verifier::external_body]
fn todo<A>() -> A
    requires false
{
    todo!();
}

//////// BONUS EXERCISES (open-ended):
//
// 1. Make `Memoizer` generic over Args, Output, and the computation.
//
// 2. The current locking scheme is very coarse-grained. Come up with (and implement) and more
//    fine-grained scheme.

}
