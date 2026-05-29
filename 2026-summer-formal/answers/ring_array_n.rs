use vstd::prelude::*;

verus! {

fn main() {
    let mut r = RingBuffer::<128>::new();
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
struct RingBuffer<const N: usize> {
    head: usize,
    tail: usize,
    data: [u32; N],
}

spec fn small_mod(i: int, n: usize) -> int {
    if i < 0 { i + n } else if i < n { i } else { i - n }
}

impl<const N: usize> View for RingBuffer<N> {
    type V = Seq<u32>;

    closed spec fn view(&self) -> Seq<u32> {
        Seq::new(
            small_mod(self.tail - self.head, N) as nat,
            |i: int| self.data[small_mod(self.head + i, N)],
        )
    }
}

impl<const N: usize> RingBuffer<N> {
    spec fn inv(&self) -> bool {
        &&& self.head < N
        &&& self.tail < N
    }

    fn new() -> (r: RingBuffer<N>)
        requires
            0 < N,
        ensures
            r.inv(),
            r@.len() == 0,
    {
        RingBuffer { head: 0, tail: 0, data: [0; N] }
    }

    fn push(&mut self, value: u32)
        requires
            self.inv(),
            self@.len() + 1 < N,
        ensures
            final(self).inv(),
            final(self)@ == old(self)@.push(value),
    {
        let mut tail = self.tail;
        self.data[tail] = value;
        tail += 1;
        if tail == N {
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
        if head == N {
            head = 0;
        }
        self.head = head;
        value
    }
}

impl<const N: usize> RingBuffer<N> {
    #[verifier::loop_isolation(false)]
    fn new_from_smaller1<const M: usize>(small: &RingBuffer<M>) -> (large: RingBuffer<N>)
        requires
            M <= N,
            small.inv(),
        ensures
            large.inv(),
            large@ == small@,
    {
        let mut data = [0; N];
        let mut large_i = 0;
        let mut small_i = small.head;
        while small_i != small.tail
            invariant
                large_i <= small@.len(),
                small_i == small_mod(large_i + small.head, M),
                forall|i: int| 0 <= i < large_i ==> #[trigger] data[i] == small@[i],
            decreases small@.len() - large_i
        {
            data[large_i] = small.data[small_i];
            large_i += 1;
            small_i += 1;
            if small_i == M {
                small_i = 0;
            }
        }
        RingBuffer { head: 0, tail: large_i, data }
    }

    #[verifier::loop_isolation(true)]
    fn new_from_smaller2<const M: usize>(small: &RingBuffer<M>) -> (large: RingBuffer<N>)
        requires
            M <= N,
            small.inv(),
        ensures
            large.inv(),
            large@ == small@,
    {
        let mut data = [0; N];
        let mut large_i = 0;
        let mut small_i = small.head;
        while small_i != small.tail
            invariant
                M <= N,
                small.inv(),
                large_i <= small@.len(),
                small_i == small_mod(large_i + small.head, M),
                forall|i: int| 0 <= i < large_i ==> #[trigger] data[i] == small@[i],
            decreases small@.len() - large_i
        {
            data[large_i] = small.data[small_i];
            large_i += 1;
            small_i += 1;
            if small_i == M {
                small_i = 0;
            }
        }
        RingBuffer { head: 0, tail: large_i, data }
    }
}

} // verus!
