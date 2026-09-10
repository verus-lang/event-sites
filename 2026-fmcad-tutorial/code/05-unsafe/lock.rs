#![verifier::loop_isolation(false)]
#![verifier::exec_allows_no_decreases_clause]

use vstd::prelude::*;
use vstd::atomic::*;
use vstd::invariant::*;
use vstd::simple_pptr::*;
use vstd::atomic;
use vstd::modes::*;
use std::sync::Arc;

verus!{

struct Lock<T> {
    pub atomic: PAtomicBool,
    pub ptr: PPtr<T>, // Verus wrapper around *mut T
    pub inv: Tracked<AtomicInvariant<
        (AtomicCellId, PPtr<T>),
        (atomic::PermissionBool, Option<PointsTo<T>>),
        LockInv
    >>,
}

// The `LockInv` type implements InvariantPredicate, which lets us specify the invariant
// that always holds on the ghost state in the AtomicInvariant
ghost struct LockInv { }
impl<T> InvariantPredicate<(AtomicCellId, PPtr<T>), (atomic::PermissionBool, Option<PointsTo<T>>)> for LockInv {
    open spec fn inv(
        cell_id_and_ptr: (AtomicCellId, PPtr<T>),
        permissions: (atomic::PermissionBool, Option<PointsTo<T>>),
    ) -> bool {
        let (atomic_cell_id, ptr) = cell_id_and_ptr;
        let (permission_bool, opt_points_to) = permissions;

        permission_bool.id() == atomic_cell_id
        && match opt_points_to {
            None => {
                // When there's no PointsTo, the lock must be taken, thus
                // the boolean value is 'true'.
                permission_bool.value() == true
            }
            Some(points_to) => {
                // If we have the PointsTo, the lock must be unlocked, thus
                // the boolean value is 'false'.
                permission_bool.value() == false
                  && points_to.pptr() == ptr
                  && points_to.is_init()
            }
        }
    }
}

impl<T> Lock<T> {
    spec fn well_formed(self) -> bool {
        self.inv@.constant() == (self.atomic.id(), self.ptr)
    }

    /// Create a fresh lock initially storing the given value.
    fn new(t: T) -> (lock: Self)
        ensures lock.well_formed()
    {
        let (atomic, Tracked(atomic_perm)) = PAtomicBool::new(false);

        let (ptr, Tracked(points_to)) = PPtr::new(t);

        Lock { atomic, ptr: ptr,
            inv: Tracked(AtomicInvariant::new(
                (atomic.id(), ptr),
                (atomic_perm, Some(points_to)),
                42 // any number is ok here; this parameter is not used for this example
            ))
        }
    }

    /// Acquire the lock, obtaining the permission to access the desired value.
    fn acquire(&self) -> (points_to: Tracked<PointsTo<T>>)
        requires self.well_formed(),
        ensures points_to@.pptr() == self.ptr && points_to.is_init()
    {
        loop {
            let tracked points_to_opt = None;
            let res;
            open_atomic_invariant!(self.inv.borrow() => permissions => {
                let tracked (mut atomic_permission, mut points_to_inv) = permissions;

                // Atomic compare_exchange (i.e., CAS)
                res = self.atomic.compare_exchange(Tracked(&mut atomic_permission), false, true);

                proof {
                    tracked_swap(&mut points_to_opt, &mut points_to_inv);
                    permissions = (atomic_permission, points_to_inv);
                }
            });
            if res.is_ok() {
                return Tracked(points_to_opt.tracked_unwrap());
            }
        }
    }

    /// Release the lock, giving up the permission to access the desired value.
    fn release(&self, points_to: Tracked<PointsTo<T>>)
        requires
            self.well_formed(),
            points_to@.pptr() == self.ptr && points_to.is_init(),
    {
        open_atomic_invariant!(self.inv.borrow() => permissions => {
            let tracked (mut atomic_permission, _) = permissions;

            // Atomic store
            self.atomic.store(Tracked(&mut atomic_permission), false);

            proof {
                permissions = (atomic_permission, Some(points_to.get()));
            }
        });
    }
}

fn main() {
    let lock = Lock::<u64>::new(0);

    let lock_shared1 = Arc::new(lock);
    let lock_shared2 = lock_shared1.clone();
    vstd::thread::spawn(move || {
        // Take the lock
        let Tracked(permission) = lock_shared1.acquire();

        // Write 20 into the data
        PPtr::write(lock_shared1.ptr, Tracked(&mut permission), 20);

        // Release the lock
        lock_shared1.release(Tracked(permission));
    });

    vstd::thread::spawn(move || {
        // Take the lock
        let Tracked(permission) = lock_shared2.acquire();

        // Read the data
        let x = PPtr::read(lock_shared2.ptr, Tracked(&permission));

        // Release the lock
        lock_shared2.release(Tracked(permission));
    });
}

}

