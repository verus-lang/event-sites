// vim: set filetype=verus laststatus=0:

use vstd::prelude::*; verus! {

spec fn fibo_spec(n: nat) -> nat
  decreases n
{
  if n == 0 { 0 } else if n == 1 { 1 }
  else {
    fibo_spec((n - 2) as nat) + fibo_spec((n - 1) as nat)
  }
}

proof fn lemma_fibo_is_monotonic(i: nat, j: nat)
  requires i <= j,
  ensures fibo_spec(i) <= fibo_spec(j),
  decreases j - i
{
  if i < 2 && j < 2 {
  } else if i == j {
  } else if i == j - 1 {
    reveal_with_fuel(fibo_spec, 2);
    lemma_fibo_is_monotonic(i, (j - 1) as nat);
  } else {
    lemma_fibo_is_monotonic(i, (j - 1) as nat);
    lemma_fibo_is_monotonic(i, (j - 2) as nat);
  }
}

fn fibo(n: u64) -> (result: u64)
  requires fibo_spec(n as nat) <= u64::MAX,
  ensures result == fibo_spec(n as nat)
{
  if n == 0 {
    return 0;
  }
  let mut prev: u64 = 0;
  let mut cur: u64 = 1;
  let mut i: u64 = 1;
  while i < n
    invariant
      0 < i <= n,
      prev == fibo_spec((i - 1) as nat),
      cur == fibo_spec(i as nat),
      fibo_spec(n as nat) <= u64::MAX,
  {
    i = i + 1;
    proof {
      lemma_fibo_is_monotonic(i as nat, n as nat);
    }
    assert(fibo_spec(i as nat) <= fibo_spec(n as nat));
    assert(cur + prev <= u64::MAX);
    let new_cur = cur + prev;
    prev = cur;
    cur = new_cur;
  }
  cur
}

fn main() {
  assert(fibo_spec(24) <= u64::MAX) by (compute);
  let f = fibo(24);
}

} // verus!

