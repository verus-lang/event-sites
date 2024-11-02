
use vstd::prelude::*; verus! {

fn smallest_elt(elts: Vec<u64>) -> (result: u64)
  requires
    elts@.len() > 0
  ensures
    forall|i:int| 0 <= i < elts@.len() ==> elts@[i] >= result,
    exists|i:int| 0 <= i < elts@.len() && elts@[i] == result,
{
  let mut i = 1;
  let mut smallest = elts[0];
  while i < elts.len() 
    invariant
      1 <= i <= elts@.len(),
      forall |j| 0 <= j < i ==> elts@[j] >= smallest,
      exists|j:int| 0 <= j < i && elts@[j] == smallest,
  {
    if elts[i] < smallest {
      smallest = elts[i];
    }
    i += 1;
  }
  smallest
}

fn main() {
}

}
