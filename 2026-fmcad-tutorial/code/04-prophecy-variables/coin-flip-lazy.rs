use vstd::prelude::*;
use vstd::modes::*;

verus!{

// We stub out the random_bool function to avoid needing to pull
// in the 'rand' dependency.
fn random_bool() -> bool {
    true
}


struct CoinFlip {
    result: Option<bool>,
}

impl CoinFlip {
    spec fn value(self) -> bool {
        ???
    }

    fn new() -> CoinFlip {
        CoinFlip {
            result: None,
        }
    }

    fn get(&mut self) -> (b: bool)
        ensures
            b == old(self).value(),
            final(self).value() == old(self).value(),
    {
        match self.result {
            Some(r) => {
                return r;
            }
            None => {
                let r = random_bool();
                self.result = Some(r);
                return r;
            }
        }
    }
}

fn main() {
    let mut c = CoinFlip::new();
    let x1 = c.get();
    let x2 = c.get();
    assert(x1 == x2);
}

}
