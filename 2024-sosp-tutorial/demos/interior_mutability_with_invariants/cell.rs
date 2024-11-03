use vstd::prelude::*;

mod cell_lib;
use cell_lib::*;

verus!{

fn main() {
    // Create a cell that can only have even values
    let cell = Cell::<u64, spec_fn(u64) -> bool>::new(
        24,
        Ghost(|v: u64| v % 2 == 0));

    let cell1: &Cell::<u64, _> = &cell;
    let cell2: &Cell::<u64, _> = &cell;

    cell1.set(24);

    cell2.set(26);

    let x = cell1.get();
    assert(x % 2 == 0);     // this is ALL we know about 'x'
}

}
