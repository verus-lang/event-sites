use vstd::prelude::*;
use vstd::rwlock::*;
use vstd::thread::spawn;
use std::sync::Arc;

verus!{

fn main() {
    // Create a lock to store a u64
    let lock = RwLock::<u64, spec_fn(u64) -> bool>::new(
        // Initial value
        12,
        // Lock invariant: number must be even
        Ghost(|v: u64| v % 2 == 0)
    );

    // Put the lock in an Arc; we can clone the Arc to share it
    // between threads.
    let shared_lock = Arc::new(lock);

    // Thread 1 - take a write lock

    let shared_lock1 = shared_lock.clone();
    spawn(move || {
        let lock: &RwLock::<u64, _> = &*shared_lock1;

        // We can add 2 to the lock (this preserves evenness)
        let (mut val, write_handle) = lock.acquire_write();
        val = val.wrapping_add(2);
        write_handle.release_write(val);
    });

    // Thread 2 - take a read lock

    let shared_lock2 = shared_lock.clone();
    spawn(move || {
        let lock: &RwLock::<u64, _> = &*shared_lock2;

        // We can use the lock invariant to check the value is even
        let read_handle = lock.acquire_read();
        let val1: &u64 = read_handle.borrow();
        assert(*val1 % 2 == 0);
        read_handle.release_read();
    });
}

}
