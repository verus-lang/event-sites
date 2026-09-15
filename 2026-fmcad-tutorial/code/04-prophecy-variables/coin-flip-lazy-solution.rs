use vstd::prelude::*;
use vstd::proph::*;
use vstd::modes::*;

verus!{

// We stub out the random_bool function to avoid needing to pull
// in the 'rand' dependency.
fn random_bool() -> bool {
    true // chosen by fair dice roll.
         // guaranteed to be random.
}


struct CoinFlip {
    result: Option<bool>,
    prophecied_result: Tracked<ProphecyGhost<bool>>,
}

impl CoinFlip {
    #[verifier::prophetic]
    spec fn value(self) -> bool {
        match self.result {
            Some(r) => r,
            None => self.prophecied_result.value(),
        }
    }

    fn new() -> CoinFlip {
        CoinFlip {
            result: None,
            prophecied_result: Tracked(ProphecyGhost::<bool>::new()),
        }
    }

    fn get(&mut self) -> (b: bool)
        ensures
            b == old(self).value(),
            final(self).value() == old(self).value(),
    {
        match self.result {
            Some(r) => r,
            None => {
                let r = random_bool();
                self.result = Some(r);

                proof {
                    // Resolve the existing prophecy variable
                    let tracked mut proph_var = ProphecyGhost::<bool>::new();
                    tracked_swap(&mut proph_var, &mut self.prophecied_result);
                    proph_var.resolve(r);
                }

                r
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
