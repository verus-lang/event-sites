// vim: set filetype=verus laststatus=0:

use vstd::prelude::*; verus! {

spec fn max_correct(a: nat, b: nat, r: nat) -> bool {
    &&& r == a || r == b
    &&& r >= a && r >= b
}

fn max(a: u64, b: u64) -> (r: u64)
  ensures max_correct(
    a as nat, b as nat, r as nat)
{
  let r = if a >= b {
    a
  } else {
    b
  };
  assert(r >= a && r >= b);
  return r;
}

} // verus!

fn main() {
  println!("{}", max(4, 7));
}

