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
// Note: it might be better to heap-allocate the data with "data: Box<[T; 128]>",
// but for simplicity we just inline the array directly into the ring buffer.
struct RingBuffer128<T> {
    head: usize,
    tail: usize,
    data: [Option<T>; 128],
}

impl<T> View for RingBuffer128<T> {
    type V = Seq<T>;

    closed spec fn view(&self) -> Seq<T> {
        // TODO
        arbitrary()
    }
}

#[verifier::external_body]
fn array_of_none<T>() -> [Option<T>; 128]
{
    // Verus doesn't yet support [const { ... }; N] array initializer syntax,
    // so we wrap this in an unverified "external_body" function
    [const { None }; 128]
}

impl<T> RingBuffer128<T> {
    spec fn inv(&self) -> bool {
        // TODO
        true
    }

    fn new() -> (r: RingBuffer128<T>)
        ensures
            r.inv(),
            r@.len() == 0,
    {
        RingBuffer128 { head: 0, tail: 0, data: array_of_none() }
    }

    fn push(&mut self, value: T)
        requires
            old(self).inv(),
            old(self)@.len() + 1 < 128,
        ensures
            final(self).inv(),
            final(self)@ == old(self)@.push(value),
    {
        let mut tail = self.tail;
        self.data[tail] = Some(value);
        tail += 1;
        if tail == 128 {
            tail = 0;
        }
        self.tail = tail;
    }

    fn pop(&mut self) -> (value: T)
        requires
            old(self).inv(),
            old(self)@.len() > 0,
        ensures
            final(self).inv(),
            final(self)@ == old(self)@.skip(1),
            value == old(self)@[0],
    {
        let mut head = self.head;
        let value = self.data[head].take().unwrap();
        head += 1;
        if head == 128 {
            head = 0;
        }
        self.head = head;
        value
    }
}

} // verus!
