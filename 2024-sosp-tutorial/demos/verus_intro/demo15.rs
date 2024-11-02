// vim: set filetype=verus laststatus=0:
use vstd::prelude::*;
verus! {

spec fn max_spec(a: nat, b: nat) -> nat {
  if a > b { a } else { b }
}

proof fn max_spec_quant()
  ensures
    forall|a: nat, b: nat|
      (max_spec(a,b) == a || max_spec(a,b) == b) &&
      (max_spec(a,b) >= a && max_spec(a,b) >= b),
      exists|a: nat, b: nat| max_spec(a, b) >= a,
{
  /*+*/ assert(max_spec(3, 4) >= 3);
}

} // verus
