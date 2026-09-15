use vstd::prelude::*;
use vstd::proph::*;

verus!{

// We stub out the random_bool function to avoid needing to pull
// in the 'rand' dependency.
fn random_bool() -> bool {
    true
}


struct CoinFlip {
    result: bool,
}

impl CoinFlip {
    spec fn value(self) -> bool {
        ???
    }

    fn new() -> CoinFlip {
        CoinFlip { result: random_bool() }
    }

    fn get(&self) -> (b: bool)
        ensures b == self.value()
    {
        self.result
    }
}


fn main() {
    let c = CoinFlip::new();
    let x1 = c.get();
    let x2 = c.get();
    assert(x1 == x2);
}

  
}
