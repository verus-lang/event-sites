use vstd::prelude::*;

// The purpose of this file is to export some `expensive_function` to be used by the
// 'memoize' exercise. The specific function doesn't matter (the one here, GCD, isn't really
// that expensive.)

verus!{

spec fn gcd(a: nat, b: nat) -> nat
    decreases a, b
{
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else if a > b {
        gcd(b, a % b)
    } else {
        gcd(a, b % a)
    }
}

fn compute_gcd(a: u64, b: u64) -> (g: u64)
    ensures g == gcd(a as nat, b as nat)
{
    let mut a1 = a;
    let mut b1 = b;
    loop
        invariant gcd(a1 as nat, b1 as nat) == gcd(a as nat, b as nat)
    {
        if a1 == 0 {
            return b1;
        } else if b1 == 0 {
            return a1;
        } else if a1 > b1 {
            let m = a1 % b1;
            a1 = b1;
            b1 = m;
        } else {
            b1 = b1 % a1;
        }
    }
}

pub type Args = u64;
pub type Output = u64;

pub closed spec fn func(args: &Args) -> Output {
    gcd(*args as nat, (*args ^ 0x110d) as nat) as u64
}

pub fn expensive_function(args: &Args) -> (out: Output)
    ensures out == func(args)
{
    compute_gcd(*args, *args ^ 0x110d)
}

}
