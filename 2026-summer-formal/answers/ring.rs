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

spec fn small_mod(i: int, n: int) -> int {
    if i < 0 { i + n } else if i < n { i } else { i - n }
}

impl View for RingBuffer128 {
    type V = Seq<u32>;

    closed spec fn view(&self) -> Seq<u32> {
        Seq::new(
            small_mod(self.tail - self.head, 128) as nat,
            |i: int| self.data[small_mod(self.head + i, 128)],
        )
    }
}

impl RingBuffer128 {
    spec fn inv(&self) -> bool {
        &&& self.head < 128
        &&& self.tail < 128
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

        assert(next & 127 == next % 128) by(bit_vector);
    }
}

struct RingBuffer256 {
    head: usize,
    tail: usize,
    data: [u32; 256],
}

impl View for RingBuffer256 {
    type V = Seq<u32>;

    closed spec fn view(&self) -> Seq<u32> {
        Seq::new(
            small_mod(self.tail - self.head, 256) as nat,
            |i: int| self.data[small_mod(self.head + i, 256)],
        )
    }
}

impl RingBuffer256 {
    spec fn inv(&self) -> bool {
        &&& self.head < 256
        &&& self.tail < 256
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
                large_i <= small@.len(),
                small_i == small_mod(large_i + small.head, 128),
                forall|i: int| 0 <= i < large_i ==> #[trigger] data[i] == small@[i],
            decreases small@.len() - large_i
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
                small.inv(),
                large_i <= small@.len(),
                small_i == small_mod(large_i + small.head, 128),
                forall|i: int| 0 <= i < large_i ==> #[trigger] data[i] == small@[i],
            decreases small@.len() - large_i
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

} // verus!
