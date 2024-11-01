use vstd::prelude::*;
verus! {

spec fn max_spec(a: nat, b: nat) -> nat {
  if a > b { a } else { b }
}

proof fn max_spec_quant() {
  assert(
    forall|a: nat, b: nat|
      (max_spec(a,b) == a || max_spec(a,b) == b) &&
      (max_spec(a,b) >= a && max_spec(a,b) >= b)
  );
  /*+*/ assert(max_spec(3, 4) >= 3);
  assert(
    exists|a: nat, b: nat| max_spec(a, b) >= a
  );
}

} // verus
