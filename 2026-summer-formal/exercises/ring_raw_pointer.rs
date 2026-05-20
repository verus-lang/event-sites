use vstd::prelude::*;
use vstd::layout::{size_of, align_of};
use vstd::raw_ptr::{PointsTo, PointsToRaw};

verus! {

// Assume that u32 and i32 are 4 bytes in size and 4-byte aligned in memory
global layout u32 is size == 4, align == 4;
global layout i32 is size == 4, align == 4;

fn main() {
    let mut r = RingBuffer5::new();
    r.push(100);
    r.push(200);
    r.push(300);
/*TODO, after pop is finished
    let x = r.pop();
    assert(x == 100);
    assert(r@ =~= seq![200, 300]);
*/
}

spec fn small_mod5(i: int) -> int {
    // TODO: replace this was something that uses + and - rather than %
    // and is good enough to calculate i % 5 for i that might slightly overflow 5 or underflow 0
    i % 5
}

// Fixed-capacity FIFO queue, pushing at the tail and popping from the head.
// Each element must be 4 bytes in size.
struct RingBuffer5<T> {
    head: usize,
    tail: usize,
    // pointer to 0th element of data
    data_ptr: *mut T,
    // free and alloc map index 0, 1, 2, 3, 4 to PointsToRaw or PointsTo<T>
    free: Tracked<Map<int, PointsToRaw>>,
    alloc: Tracked<Map<int, PointsTo<T>>>,
}

impl<T> View for RingBuffer5<T> {
    type V = Seq<T>;

    closed spec fn view(&self) -> Seq<T> {
        Seq::new(
            small_mod5(self.tail - self.head) as nat,
            |i: int| self.alloc[small_mod5(self.head + i)].value(),
        )
    }
}

// Build Set {start + 0, start + 1, start + 2, start + 3}
spec fn range_set4(start: int) -> Set<int> {
    Set::range(start, start + 4)
}

// Split a single large 20-byte PointsToRaw into 5 4-byte pieces,
// and put these 5 4-byte pieces into the free Map.
// Do this by repeatedly spittling off 4 bytes at a time,
// using range_set4 to define the set of the 4 addresses of each of the 4 bytes.
proof fn make_free_map(
    addr: int,
    tracked data: PointsToRaw,
) -> (tracked free: Map<int, PointsToRaw>)
    requires
        data.is_range(addr, 20),
    ensures
        forall|i: int|
            #![trigger free.contains_key(i)]
            #![trigger free[i]]
            0 <= i < 5 ==> {
                &&& free.contains_key(i)
                &&& free[i].is_range(addr + 4 * i, 4)
                &&& free[i].provenance() == data.provenance()
            },
{
    let tracked (data0, data) = data.split(range_set4(addr + 0));
    let tracked (data1, data) = data.split(range_set4(addr + 4));
    let tracked (data2, data) = data.split(range_set4(addr + 8));
    let tracked (data3, data) = data.split(range_set4(addr + 12));
    let tracked (data4, data) = data.split(range_set4(addr + 16));
    let tracked mut free = Map::tracked_empty();
    free.tracked_insert(0, data0);
    free.tracked_insert(1, data1);
    free.tracked_insert(2, data2);
    free.tracked_insert(3, data3);
    free.tracked_insert(4, data4);
    free
}

/*
// More general recursive alternative to make_free_map
proof fn make_free_map_rec(
    addr: int,
    n: int,
    tracked data: PointsToRaw,
) -> (tracked free: Map<int, PointsToRaw>)
    requires
        0 <= n,
        data.is_range(addr, 4 * n),
    ensures
        forall|i: int|
            #![trigger free.contains_key(i)]
            #![trigger free[i]]
            0 <= i < n ==> {
                &&& free.contains_key(i)
                &&& free[i].is_range(addr + 4 * i, 4)
                &&& free[i].provenance() == data.provenance()
            },
    decreases TODO
{
    TODO
}
*/

impl<T> RingBuffer5<T> {
    spec fn inv(&self) -> bool {
        &&& self.head < 5
        &&& self.tail < 5
        &&& self.data_ptr.addr() + 4 * 5 <= usize::MAX + 1
        &&& self.data_ptr.addr() % 4 == 0
        &&& forall|i: int|
            #![trigger self.free.contains_key(i)]
            #![trigger self.free[i]]
            0 <= i < 5 && !(0 <= small_mod5(i - self.head) < self@.len()) ==> {
                &&& #[trigger] self.free.contains_key(i)
                &&& self.free[i].is_range(self.data_ptr.addr() + 4 * i, 4)
                &&& self.free[i].provenance() == self.data_ptr@.provenance
            }
        &&& forall|i: int|
            #![trigger self.alloc.contains_key(i)]
            #![trigger self.alloc[i]]
            0 <= i < 5 && 0 <= small_mod5(i - self.head) < self@.len() ==> {
                &&& self.alloc.contains_key(i)
                &&& self.alloc[i].is_init()
                &&& self.alloc[i].ptr()@.addr == self.data_ptr.addr() + 4 * i
                &&& self.alloc[i].ptr()@.provenance == self.data_ptr@.provenance
            }
    }

    fn new() -> (r: RingBuffer5<T>)
        ensures
            r.inv(),
            r@.len() == 0,
    {
        // 4 * 5 bytes, 4-byte alignment
        let (data_ptr, Tracked(data), Tracked(dealloc)) = vstd::raw_ptr::allocate(4 * 5, 4);

        let mut free = Tracked(make_free_map(data_ptr.addr() as int, data));
        let mut alloc = Tracked(Map::tracked_empty());

        RingBuffer5 { head: 0, tail: 0, data_ptr: data_ptr as *mut T, free, alloc }
    }

    fn push(&mut self, value: T)
        requires
            size_of::<T>() == 4,
            align_of::<T>() == 4,
            self.inv(),
            self@.len() + 1 < 5,
        ensures
            final(self).inv(),
            final(self)@ == old(self)@.push(value),
    {
        let mut tail = self.tail;

        // Pointer-based code implementing self.data[tail] = value
        let addr: usize = self.data_ptr as usize + 4 * tail;
        let element_ptr: *mut T = self.data_ptr.with_addr(addr) as *mut T;
        let tracked free_element: PointsToRaw = self.free.tracked_remove(tail as int);
        let tracked mut alloc_element: PointsTo<T> = free_element.into_typed(addr);
        vstd::raw_ptr::ptr_mut_write(element_ptr, Tracked(&mut alloc_element), value);
        proof {
            self.alloc.tracked_insert(tail as int, alloc_element);
        }

        tail += 1;
        if tail == 5 {
            tail = 0;
        }
        self.tail = tail;
    }

/*
    fn pop(&mut self) -> (value: T)
        requires
            size_of::<T>() == 4,
            align_of::<T>() == 4,
            self.inv(),
            self@.len() > 0,
        ensures
            final(self).inv(),
            final(self)@ == old(self)@.skip(1),
            value == old(self)@[0],
    {
        let mut head = self.head;

        // Pointer-based code implementing value = self.data[head],
        let addr: usize = self.data_ptr as usize + 4 * head;
        let element_ptr: *mut T = self.data_ptr.with_addr(addr) as *mut T;
/*
TODO:
- use tracked_remove to remove alloc_element: PointsTo<T> from self.alloc
- use vstd::raw_ptr::ptr_mut_read to read value at address element_ptr
- use alloc_element.into_raw() to convert PointsTo<T> to free_element: PointsToRaw
- use tracked_insert to insert free_element into self.free
See corresponding operations in "push" above as a starting point

Note: ptr_mut_read has signature:
    ptr_mut_read<T>(ptr: *const T, Tracked(perm): Tracked<&mut PointsTo<T>>) -> (v: T)
For more details, see:
- https://verus-lang.github.io/verus/verusdoc/vstd/raw_ptr/fn.ptr_mut_read.html
- https://verus-lang.github.io/verus/verusdoc/src/vstd/raw_ptr.rs.html
*/

        head += 1;
        if head == 5 {
            head = 0;
        }
        self.head = head;
        value
    }
*/
}

} // verus!
