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

    // Thread 1

    let shared_lock1 = shared_lock.clone();
    spawn(move || {
        let lock: &RwLock::<u64, _> = &*shared_lock1;

        // We can use the lock invariant to check the value is even
        let read_handle1 = lock.acquire_read();
        let val1: &u64 = read_handle1.borrow();
        assert(*val1 % 2 == 0);

        // If we acquire a second read-lock, we can confirm each handle
        // reads the same value:
        let read_handle2 = lock.acquire_read();
        let val2 = *read_handle2.borrow();

        proof { ReadHandle::lemma_readers_match(&read_handle1, &read_handle2); }
        assert(val1 == val2);

        read_handle1.release_read();
        read_handle2.release_read();
    });

    // Thread 2

    let shared_lock2 = shared_lock.clone();
    spawn(move || {
        let lock: &RwLock::<u64, _> = &*shared_lock2;

        // We can add 2 to the lock (this preserves evenness)
        let (mut val, write_handle) = lock.acquire_write();
        val = val.wrapping_add(2);
        write_handle.release_write(val);
    });

    // Thread 3 -- THE EVIL THREAD >:(

    let shared_lock3 = shared_lock.clone();
    spawn(move || {
        let lock: &RwLock::<u64, _> = &*shared_lock3;

        let (val, write_handle) = lock.acquire_write();
        let new_val = 13; // 13 isn't even!
        write_handle.release_write(new_val); // ERROR: invariant violated
    });
}

}
