use std::sync::RwLock;
use std::sync::Arc;
use std::thread::spawn;
use std::sync::RwLockWriteGuard;

fn main() {
    let lock = RwLock::<u64>::new(0);
    let shared_lock = Arc::new(lock);

    // Thread A
    let shared_lock1 = shared_lock.clone();
    spawn(move || {
        let mut handle: RwLockWriteGuard<_> = shared_lock1.write().unwrap();
        *handle += 1;
    });

    // Thread B
    let shared_lock2 = shared_lock.clone();
    spawn(move || {
        let mut handle = shared_lock2.write().unwrap();
        *handle += 1;
    });

    let read_handle = shared_lock.read().unwrap();
    let final_value = *read_handle;

    assert!(final_value == 2);
}
