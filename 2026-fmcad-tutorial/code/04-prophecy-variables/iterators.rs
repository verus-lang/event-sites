use vstd::prelude::*;
use vstd::std_specs::iter::IteratorSpec;

verus!{

    fn double(x: &u64) -> (out: u64)
        requires x < 0x8000_0000_0000_0000
        ensures out == *x * 2
    {
        x * 2
    }

    fn test_iterators() {
        let v = vec![1, 2, 3, 4, 5, 6];

        let w = v.iter()
                 .skip(1)
                 .take(4)
                 .map(double)
                 .collect::<Vec<u64>>();

        assert(w@ =~= seq![4, 6, 8, 10]);
    }

    fn test_iterators2() {
        let v = vec![1, 2, 3, 4, 5, 6];

        let iter1 = v.iter();                 assert(iter1.remaining() == seq![&1, &2, &3, &4, &5, &6]);
        let iter2 = iter1.skip(1);            assert(iter2.remaining() == seq![&2, &3, &4, &5, &6]);
        let iter3 = iter2.take(4);            assert(iter3.remaining() == seq![&2, &3, &4, &5]);
        let iter4 = iter3.map(double);
        let w = iter4.collect::<Vec<u64>>();  assert(iter4.remaining() == seq![4, 6, 8, 10]);

        assert(w@ =~= seq![4, 6, 8, 10]);
    }



}
