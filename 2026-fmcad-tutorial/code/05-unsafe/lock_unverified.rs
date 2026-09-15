use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

struct Mutex {
  lock: AtomicBool,
}

impl Mutex {
  fn new() -> Self {
    Mutex { lock: AtomicBool::new(false) }
  }

  fn lock(&self) {
    loop {
      let success = self.lock.compare_exchange(false, true, SeqCst, SeqCst);
      if success.is_ok() { break; }
    }
  }

  fn unlock(&self) {
    self.lock.store(false, SeqCst);
  }
}

fn main() {
}
