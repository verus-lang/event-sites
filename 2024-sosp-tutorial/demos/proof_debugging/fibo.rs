// vim: set filetype=verus laststatus=0:

use vstd::prelude::*;

verus! {

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

exec fn fibo(n: u64) -> (result: Option<u64>)
  ensures
    result is Some <==> fibo_spec(n as nat) <= u64::MAX,
    result matches Some(r) ==> fibo_spec(n as nat) == r,
{
  if n == 0 {
    return Some(0);
  }
  let mut prev: u64 = 0;
  let mut cur: u64 = 1;
  let mut i: u64 = 1;
  while i < n
    invariant
      0 < i <= n,
      prev == fibo_spec((i - 1) as nat),
      cur == fibo_spec(i as nat),
  {
    i = i + 1;
    proof {
      lemma_fibo_is_monotonic(i as nat, n as nat);
    }
    assert(cur + prev == fibo_spec(i as nat));
    match cur.checked_add(prev) {
      Some(new_cur) => {
        prev = cur;
        cur = new_cur;
      },
      None => {
        assert(fibo_spec(n as nat) > u64::MAX);
        return None;
      },
    }
  }
  Some(cur)
}

// fn main() {
//   assert(fibo_spec(16) <= u64::MAX) by (compute_only);
//   let f = fibo(16);
//   assert(fibo_spec(16) == 987) by (compute_only);
//   assert(f == Some(987u64));
// }

} // verus!

fn main() {
  for i in 0..256 {
    println!("fibo({}) == {:?}", i, fibo(i));
  }
}
