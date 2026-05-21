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
// Note: it might be better to heap-allocate the data with "data: Box<[u32; 128]>",
// but for simplicity we just inline the array directly into the ring buffer.
struct RingBuffer128 {
    head: usize,
    tail: usize,
    data: [u32; 128],
}

impl View for RingBuffer128 {
    type V = Seq<u32>;

    closed spec fn view(&self) -> Seq<u32> {
        // TODO
        Seq::new(
            0, // length of sequence
            |i: int| arbitrary(), // value of the sequence at index i
        )
    }
}

impl RingBuffer128 {
    spec fn inv(&self) -> bool {
        // TODO
        true
    }

    fn new() -> (r: RingBuffer128)
        ensures
            r.inv(),
            r@.len() == 0,
    {
        RingBuffer128 { head: 0, tail: 0, data: [0; 128] }
    }

    fn push(&mut self, value: u32)
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

    fn pop(&mut self) -> (value: u32)
        requires
            self.inv(),
            self@.len() > 0,
        ensures
            final(self).inv(),
            final(self)@ == old(self)@.skip(1),
            value == old(self)@[0],
    {
        let mut head = self.head;
        let value = self.data[head];
        head += 1;
        if head == 128 {
            head = 0;
        }
        self.head = head;
        value
    }
}

/*
impl RingBuffer128 {
    fn push_via_bitwise_and(&mut self, value: u32)
        requires
            self.inv(),
            self@.len() + 1 < 128,
        ensures
            final(self).inv(),
            final(self)@ == old(self)@.push(value),
    {
        let tail = self.tail;
        self.data[tail] = value;
        let next = tail + 1;
        self.tail = next & 127;

        // TODO
        assert(true) by(bit_vector);
    }
}
*/

/*
struct RingBuffer256 {
    head: usize,
    tail: usize,
    data: [u32; 256],
}

impl View for RingBuffer256 {
    type V = Seq<u32>;

    closed spec fn view(&self) -> Seq<u32> {
        // TODO
        arbitrary()
    }
}

impl RingBuffer256 {
    spec fn inv(&self) -> bool {
        // TODO
        true
    }

    fn new() -> (r: RingBuffer256)
        ensures
            r.inv(),
            r@.len() == 0,
    {
        RingBuffer256 { head: 0, tail: 0, data: [0; 256] }
    }

    #[verifier::loop_isolation(false)]
    fn new_from_smaller1(small: &RingBuffer128) -> (large: RingBuffer256)
        requires
            small.inv(),
        ensures
            large.inv(),
            large@ == small@,
    {
        let mut data = [0; 256];
        let mut large_i = 0;
        let mut small_i = small.head;
        while small_i != small.tail
            invariant
                // TODO
                true,
                true,
                forall|i: int| #[trigger] data[i] == arbitrary::<u32>(),
            decreases 0int // TODO
        {
            data[large_i] = small.data[small_i];
            large_i += 1;
            small_i += 1;
            if small_i == 128 {
                small_i = 0;
            }
        }
        RingBuffer256 { head: 0, tail: large_i, data }
    }

    #[verifier::loop_isolation(true)]
    fn new_from_smaller2(small: &RingBuffer128) -> (large: RingBuffer256)
        requires
            small.inv(),
        ensures
            large.inv(),
            large@ == small@,
    {
        let mut data = [0; 256];
        let mut large_i = 0;
        let mut small_i = small.head;
        while small_i != small.tail
            invariant
                // TODO
                true,
                true,
                true,
                forall|i: int| #[trigger] data[i] == arbitrary::<u32>(),
            decreases 0int // TODO
        {
            data[large_i] = small.data[small_i];
            large_i += 1;
            small_i += 1;
            if small_i == 128 {
                small_i = 0;
            }
        }
        RingBuffer256 { head: 0, tail: large_i, data }
    }
}
*/

} // verus!
