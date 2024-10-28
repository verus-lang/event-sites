#![allow(unused_imports)]

use builtin::*;
use builtin_macros::*;
use vstd::simple_pptr::*;
use vstd::prelude::*;

verus!{

fn main() {
    unsafe {
        // ALLOCATE
        // p: PPtr<u64>, points_to: PointsTo<u64>
        let (p, Tracked(mut points_to)) = PPtr::<u64>::empty();

        assert(points_to.mem_contents() === MemContents::Uninit);
        assert(points_to.pptr() == p);

        // Equivalent to:
        // unsafe { *p = 5; }
        p.write(Tracked(&mut points_to), 5); 

        assert(points_to.mem_contents() === MemContents::Init(5));
        assert(points_to.pptr() == p);

        // Equivalent to:
        // let x = unsafe { *p };
        let x = p.read(Tracked(&points_to)); 

        assert(x == 5);

        // DEALLOCATE
        let y = p.into_inner(Tracked(points_to));

        assert(y == 5);
    }
}

}
