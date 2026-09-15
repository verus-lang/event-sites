#![verifier::loop_isolation(false)]
#![verifier::exec_allows_no_decreases_clause]

use vstd::prelude::*;
verus!{

spec fn fibonacci(n: nat) -> nat
    decreases n,
{
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci((n - 2) as nat) + fibonacci((n - 1) as nat)
    }
}

proof fn lemma_fibonacci_is_monotonic(i: nat, j: nat)
    requires
        i <= j,
    ensures
        fibonacci(i) <= fibonacci(j),
    decreases j - i,
{
    if j >= i + 1 {
        lemma_fibonacci_is_monotonic(i, (j - 1) as nat);
    }
}

exec fn fibonacci_impl(n: u64) -> (result: u64)
    requires
        fibonacci(n as nat) <= u64::MAX,
    ensures
        result == fibonacci(n as nat),
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
            cur == fibonacci(i as nat),
            prev == fibonacci((i - 1) as nat),
    {
        i = i + 1;
        proof {
            lemma_fibonacci_is_monotonic(i as nat, n as nat);
        }
        let new_cur = cur + prev;
        prev = cur;
        cur = new_cur;
    }
    cur
}



}

fn main() { }
