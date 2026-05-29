// This file requires the "-V new-mut-ref" command-line option to verus

use vstd::prelude::*;

verus! {

fn main() {
    let mut r = RingBuffer128::new();
    r.push(100);
    r.push(200);
    r.push(300);
    let x = r.pop();
    assert(x == 100);
    assert(r@ =~= seq![200, 300]);
}

// Fixed-capacity FIFO queue, pushing at the tail and popping from the head.
struct RingBuffer128<T> {
    head: usize,
    tail: usize,
    data: Vec<T>,
}

spec fn small_mod(i: int, n: int) -> int {
    if i < 0 { i + n } else if i < n { i } else { i - n }
}

impl<T> View for RingBuffer128<T> {
    type V = Seq<T>;

    closed spec fn view(&self) -> Seq<T> {
        Seq::new(
            small_mod(self.tail - self.head, 128) as nat,
            |i: int| self.data[small_mod(self.head + i, 128)],
        )
    }
}

impl<T: Default> RingBuffer128<T> {
    spec fn inv(&self) -> bool {
        &&& self.head < 128
        &&& self.tail < 128
        &&& self.data.len() == 128
    }

    fn new() -> (r: RingBuffer128<T>)
        ensures
            r.inv(),
            r@.len() == 0,
    {
        let mut v = Vec::new();
        for i in 0..128
            invariant
                v.len() == i,
        {
            v.push(Default::default());
        }
        RingBuffer128 { head: 0, tail: 0, data: v }
    }

    fn push(&mut self, value: T)
        requires
            self.inv(),
            self@.len() + 1 < 128,
        ensures
            final(self).inv(),
            final(self)@ == old(self)@.push(value),
    {
        let mut tail = self.tail;
        self.data[tail] = value;
        tail += 1;
        if tail == 128 {
            tail = 0;
        }
        self.tail = tail;
    }

    fn pop(&mut self) -> (value: T)
        requires
            self.inv(),
            self@.len() > 0,
        ensures
            final(self).inv(),
            final(self)@ == old(self)@.skip(1),
            value == old(self)@[0],
    {
        let mut head = self.head;
        let mut value = Default::default();
        std::mem::swap(&mut self.data[head], &mut value);
        head += 1;
        if head == 128 {
            head = 0;
        }
        self.head = head;
        value
    }
}

} // verus!
