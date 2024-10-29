#![allow(unused_imports)]

use vstd::atomic_ghost::*;
use vstd::modes::*;
use vstd::prelude::*;
use vstd::thread::*;
use vstd::{pervasive::*, *};
use state_machines_macros::tokenized_state_machine;
use std::sync::Arc;
use vstd::rwlock::{RwLock, RwLockPredicate};

verus! {

//// VerusSync transition system
tokenized_state_machine!(
    X {
        // Describe the state of the system
        fields {
            // Current value of the counter
            #[sharding(variable)]
            pub counter: int,

            // Has Thread A performed its increment yet?
            #[sharding(variable)]
            pub inc_a: bool,

            // Has Thread B performed its increment yet?
            #[sharding(variable)]
            pub inc_b: bool,
        }

        //// Describe the 'initial' state of the system

        init!{
            initialize() {
                init counter = 0;
                init inc_a = false;
                init inc_b = false;
            }
        }

        //// Describe the allowed transitions

        transition!{
            tr_inc_a() {
                require(!pre.inc_a);
                update counter = pre.counter + 1;
                update inc_a = true;
            }
        }

        transition!{
            tr_inc_b() {
                require(!pre.inc_b);
                update counter = pre.counter + 1;
                update inc_b = true;
            }
        }

        //// Describe some properties that we would like to hold.
        //// These properties must hold from any reachable state
        //// of the system.

        // Property 1: From any reachable state, it should be
        // possible to add 1 to the counter without overflowing a u32 value.
        property!{
            increment_will_not_overflow_u32() {
                assert 0 <= pre.counter < 0xffff_ffff;
            }
        }

        // Property 2: From any reachable state, if both Thread A
        // and Thread B have performed their increment,
        // then the counter value must be equal to 2.
        property!{
            finalize() {
                require(pre.inc_a);
                require(pre.inc_b);
                assert pre.counter == 2;
            }
        }

        // To prove that these properties hold for any reachable state,
        // we need to supply an inductive invariant.
        // This inductive invariant should:
        //  (1) Apply to any reachable state, and be provable by induction
        //  (2) Imply the above 2 properties.

        #[invariant]
        pub fn the_invariant(&self) -> bool {
            self.counter == (if self.inc_a { 1 as int } else { 0 }) + (if self.inc_b { 1 as int } else { 0 })
        }

        // In general, proof bodies for the inductiveness proofs go here.
        // However, you probably won't need them for this exercise:

        #[inductive(tr_inc_a)]
        fn tr_inc_a_preserves_the_invariant(pre: Self, post: Self) {
            /* proof here */
        }

        #[inductive(tr_inc_b)]
        fn tr_inc_b_preserves_the_invariant(pre: Self, post: Self) {
            /* proof here */
        }

        #[inductive(initialize)]
        fn initial_state_satisfies_inv(post: Self) {
            /* proof here */
        }
    }
);

pub ghost struct CounterPredicate {
    pub instance: X::Instance,
}

pub struct LockInterior {
    pub counter: u32,
    pub ghost_counter: Tracked<X::counter>,
}

impl RwLockPredicate<LockInterior> for CounterPredicate {
    open spec fn inv(self, lock_interior: LockInterior) -> bool {
        lock_interior.counter == lock_interior.ghost_counter@@.value
          && lock_interior.ghost_counter@@.instance == self.instance
    }
}

fn main() {
    // Initialize protocol
    let tracked (
        Tracked(instance),
        Tracked(counter_token),
        Tracked(inc_a_token),
        Tracked(inc_b_token),
    ) = X::Instance::initialize();

    // Initialize the counter to 0
    // Maintain an invariant that the counter token's value matches the actual u32 counter
    let lock = RwLock::<LockInterior, CounterPredicate>::new(
        LockInterior { counter: 0, ghost_counter: Tracked(counter_token) },
        Ghost(CounterPredicate { instance: instance }),
    );

    let shared_lock = Arc::new(lock);

    // Spawn threads

    // Thread A
    let shared_lock1 = shared_lock.clone();
    let join_handle1 = spawn(
        (move || -> (new_token: Tracked<X::inc_a>)
            ensures
                new_token@@.instance == instance && new_token@@.value == true,
            {
                // Our closure captures a few variables: shared_lock1, which gives
                // us access to the lock, and the `inc_a_token`, which gives us
                // the 'right' to perform one increment operation.
                let tracked mut token = inc_a_token;
                let lock: &RwLock<_, _> = &*shared_lock1;

                // Take the lock
                let (mut lock_interior, lock_handle) = lock.acquire_write();

                // Ghost increment
                proof {
                    // Prove that the increment operation won't overflow the u32.
                    instance.increment_will_not_overflow_u32(
                        &*lock_interior.ghost_counter.borrow());

                    // Update the token values
                    instance.tr_inc_a(
                        &mut *lock_interior.ghost_counter.borrow_mut(),
                        &mut token);
                }

                // Physical increment
                lock_interior.counter = lock_interior.counter + 1;

                // Release lock
                lock_handle.release_write(lock_interior);

                Tracked(token)
            }
        )
    );

    // Thread B
    let shared_lock2 = shared_lock.clone();
    let join_handle2 = spawn(
        (move || -> (new_token: Tracked<X::inc_b>)
            ensures
                new_token@@.instance == instance && new_token@@.value == true,
            {
                // Our closure captures a few variables: shared_lock2, which gives
                // us access to the lock, and the `inc_b_token`, which gives us
                // the 'right' to perform one increment operation.
                let tracked mut token = inc_b_token;
                let lock: &RwLock<_, _> = &*shared_lock2;

                // Take the lock
                let (mut lock_interior, lock_handle) = lock.acquire_write();

                // Ghost increment
                proof {
                    // Prove that the increment operation won't overflow the u32.
                    instance.increment_will_not_overflow_u32(
                        &*lock_interior.ghost_counter.borrow());

                    // Update the token values
                    instance.tr_inc_b(
                        &mut *lock_interior.ghost_counter.borrow_mut(),
                        &mut token);
                }

                // Physical increment
                lock_interior.counter = lock_interior.counter + 1;

                // Release lock
                lock_handle.release_write(lock_interior);

                Tracked(token)
            }
        )
    );

    // Join all threads. Note this lets us regain access to the `inc_a` and `inc_b` tokens.
    // In case of an unexpected thread system error, abort early.

    let tracked inc_a_token;
    match join_handle1.join() {
        Result::Ok(token) => {
            proof {
                inc_a_token = token.get();
            }
        },
        _ => {
            return;
        }
    };
    let tracked inc_b_token;
    match join_handle2.join() {
        Result::Ok(token) => {
            proof {
                inc_b_token = token.get();
            }
        },
        _ => {
            return;
        }
    };

    // Take a read lock and read the value of the counter one last time.

    let lock = &*shared_lock;
    let read_handle = lock.acquire_read();

    let readonly_lock_interior = read_handle.borrow();
    let final_value = readonly_lock_interior.counter;

    proof {
        instance.finalize(
            &*readonly_lock_interior.ghost_counter.borrow(),
            &inc_a_token,
            &inc_b_token,
        );

        //////// Our principle objective is to show that this is true:
        assert(final_value == 2);
    }

    read_handle.release_read();
}

}
