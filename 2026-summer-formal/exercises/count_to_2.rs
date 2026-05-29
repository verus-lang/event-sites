use std::sync::Arc;
use vstd::prelude::*;
use vstd::resource::Loc;
use vstd::resource::frac::FracGhost;
use vstd::atomic::{PAtomicU32, PermissionU32};
use vstd::invariant::{AtomicInvariant, InvariantPredicate};
use vstd::shared::Shared;
use vstd::open_atomic_invariant;

verus! {

tracked struct CountState {
    tracked inv_frac_a: FracGhost<int>,
    tracked inv_frac_b: FracGhost<int>,
    tracked atomicu32_perm: PermissionU32,
}

impl InvariantPredicate<spec_fn(CountState) -> bool, CountState> for CountState {
    open spec fn inv(inv_fn: spec_fn(CountState) -> bool, v: CountState) -> bool {
        inv_fn(v)
    }
}

type AtomicInv<State> = AtomicInvariant<spec_fn(State) -> bool, State, State>;

fn main() {
    // Integer fractions
    let tracked mut inv_frac_a: FracGhost<int> = FracGhost::new(0);
    let tracked mut inv_frac_b: FracGhost<int> = FracGhost::new(0);
    let tracked thread_frac_a = inv_frac_a.split();
    let tracked thread_frac_b = inv_frac_b.split();

    // Atomic U32
    let (ptr, Tracked(atomicu32_perm)) = PAtomicU32::new(0);
    let ptr = Arc::new(ptr);
    let ptr_a = ptr.clone();
    let ptr_b = ptr.clone();

    // AtomicInv<CountState>
    let ghost inv_fn = |cs: CountState| {
        &&& cs.inv_frac_a.id() == thread_frac_a.id()
        &&& cs.inv_frac_b.id() == thread_frac_b.id()
        &&& cs.inv_frac_a.frac() == 0.5real
        &&& cs.inv_frac_b.frac() == 0.5real
        &&& cs.atomicu32_perm.id() == ptr.id()

        // TODO:
        &&& true
        &&& true

        &&& cs.atomicu32_perm.value() == cs.inv_frac_a@ + cs.inv_frac_b@
    };
    let tracked count_state = CountState { inv_frac_a, inv_frac_b, atomicu32_perm };
    let tracked atomic_inv: AtomicInv<CountState> = AtomicInvariant::new(inv_fn, count_state, 0);
    let tracked shared_atomic_inv = Shared::new(atomic_inv);
    let tracked shared_atomic_inv_a = shared_atomic_inv.clone();
    let tracked shared_atomic_inv_b = shared_atomic_inv.clone();

    // Thread a
    let thread_a = vstd::thread::spawn(|| -> (r: Tracked<FracGhost<int>>)
        ensures
            r.id() == thread_frac_a.id(),
            r.frac() == 0.5real,
            r@@ == 1,
        {
            let tracked mut thread_frac_a = thread_frac_a;
            let tracked shared_atomic_inv = shared_atomic_inv_a;
            let ptr = ptr_a;

            open_atomic_invariant!(shared_atomic_inv.borrow() => count_state => {
                ptr.fetch_add(Tracked(&mut count_state.atomicu32_perm), 1);
                proof {
                    count_state.inv_frac_a.combine(thread_frac_a);
                    count_state.inv_frac_a.update(1);
                    thread_frac_a = count_state.inv_frac_a.split();
                }
            });

            Tracked(thread_frac_a)
        }
    );

    // Thread b
    let thread_b = vstd::thread::spawn(|| -> (r: Tracked<FracGhost<int>>)
        ensures
            r.id() == thread_frac_b.id(),
            r.frac() == 0.5real,
            r@@ == 1,
        {
            let tracked mut thread_frac_b = thread_frac_b;
            let tracked shared_atomic_inv = shared_atomic_inv_b;
            let ptr = ptr_b;

            open_atomic_invariant!(shared_atomic_inv.borrow() => count_state => {
                ptr.fetch_add(Tracked(&mut count_state.atomicu32_perm), 1);
                proof {
                    count_state.inv_frac_b.combine(thread_frac_b);
                    count_state.inv_frac_b.update(1);
                    thread_frac_b = count_state.inv_frac_b.split();
                }
            });

            Tracked(thread_frac_b)
        }
    );

    // Parent joins both threads and loads n == 2
    if let (Ok(join_a), Ok(join_b)) = (thread_a.join(), thread_b.join()) {
        let Tracked(thread_frac_a) = join_a;
        let Tracked(thread_frac_b) = join_b;
        let n: u32;
        open_atomic_invariant!(shared_atomic_inv.borrow() => count_state => {
            n = ptr.load(Tracked(&mut count_state.atomicu32_perm));
            proof {
                count_state.inv_frac_a.agree(&thread_frac_a);
                count_state.inv_frac_b.agree(&thread_frac_b);
            }
        });
        assert(n == 2);
    }
}

} // verus!
