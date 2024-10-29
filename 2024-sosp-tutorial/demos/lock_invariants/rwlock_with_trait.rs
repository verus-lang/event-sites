use vstd::prelude::*;
use vstd::rwlock::*;

verus!{

struct FixedParity {
    pub parity: int,
}

impl RwLockPredicate<u64> for FixedParity {
    open spec fn inv(self, v: u64) -> bool {
        v % 2 == self.parity
    }
}

fn main() {
    // Create a lock that can only store even integers
    let lock_even = RwLock::<u64, FixedParity>::new(20, Ghost(FixedParity { parity: 0 }));

    // Create a lock that can only store odd integers
    let lock_odd = RwLock::<u64, FixedParity>::new(23, Ghost(FixedParity { parity: 1 }));

    let read_handle_even = lock_even.acquire_read();
    let val_even = *read_handle_even.borrow();
    assert(val_even % 2 == 0);
    read_handle_even.release_read();

    let read_handle_odd = lock_odd.acquire_read();
    let val_odd = *read_handle_odd.borrow();
    assert(val_odd % 2 == 1);
    read_handle_odd.release_read();

    let (even_value, write_handle_even) = lock_even.acquire_write();
    write_handle_even.release_write(7);
}

}
