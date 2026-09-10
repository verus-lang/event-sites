#![allow(unused_imports)]

use vstd::prelude::*;
use vstd::simple_pptr::*;

verus!{

fn main() {
    unsafe {
        // ALLOCATE
        // p: PPtr<u64>, points_to: PointsTo<u64>
        let (p, Tracked(mut points_to)) = PPtr::<u64>::empty();

        assert(points_to.mem_contents() === MemContents::Uninit);
        assert(points_to.pptr() == p);

        // unsafe { *p = 5; }
        PPtr::write(p, Tracked(&mut points_to), 5); 

        assert(points_to.mem_contents() === MemContents::Init(5));
        assert(points_to.pptr() == p);

        // let x = unsafe { *p };
        let x = PPtr::read(p, Tracked(&points_to)); 

        assert(x == 5);

        // DEALLOCATE
        let y = PPtr::into_inner(p, Tracked(points_to));

        assert(y == 5);
    }
}

}
