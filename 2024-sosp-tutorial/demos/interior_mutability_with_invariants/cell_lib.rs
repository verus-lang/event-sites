use vstd::prelude::*;
use vstd::cell::{PCell, PointsTo};
use vstd::invariant::*;

// (this should probably be added to vstd, it's nicer than the current Cell utility)

verus!{

pub trait CellPredicate<V>: Sized {
    spec fn inv(self, v: V) -> bool;
}

impl<V> CellPredicate<V> for spec_fn(V) -> bool {
    open spec fn inv(self, v: V) -> bool {
        self(v)
    }
}

struct InternalCellPred<T, Pred> { t: T, p: Pred }

impl<T, Pred: CellPredicate<T>> InvariantPredicate<(Pred, PCell<T>), PointsTo<T>> for InternalCellPred<T, Pred> {
    closed spec fn inv(k: (Pred, PCell<T>), perm: PointsTo<T>) -> bool {
        let (pred, pcell) = k;
        {
            &&& perm@.value.is_Some()
            &&& pred.inv(perm@.value.get_Some_0())
            &&& pcell.id() === perm@.pcell
        }
    }
}

#[verifier::reject_recursive_types(T)]
pub struct Cell<T, Pred: CellPredicate<T>> {
    pred: Ghost<Pred>,
    pcell: PCell<T>,
    perm_inv: Tracked<LocalInvariant<(Pred, PCell<T>), PointsTo<T>, InternalCellPred<T, Pred>>>,
}

impl<T, Pred: CellPredicate<T>> Cell<T, Pred> {
    #[verifier::type_invariant]
    closed spec fn wf(&self) -> bool {
        self.perm_inv@.constant() === (self.pred@, self.pcell)
    }

    pub open spec fn inv(&self, val: T) -> bool {
        self.pred().inv(val)
    }

    pub closed spec fn pred(&self) -> Pred {
        self.pred@
    }

    pub fn new(val: T, Ghost(pred): Ghost<Pred>) -> (cell: Self)
        requires
            pred.inv(val),
        ensures
            cell.pred() == pred,
    {
        let (pcell, Tracked(perm)) = PCell::new(val);
        let tracked perm_inv = LocalInvariant::new((pred, pcell), perm, 0);
        Cell { pred: Ghost(pred), pcell, perm_inv: Tracked(perm_inv) }
    }
}

impl<T, Pred: CellPredicate<T>> Cell<T, Pred> {
    pub fn replace(&self, val: T) -> (old_val: T)
        requires
            self.inv(val),
        ensures
            self.inv(old_val),
    {
        proof {
            use_type_invariant(self);
        }
        let r;
        open_local_invariant!(self.perm_inv.borrow() => perm => {
            r = self.pcell.replace(Tracked(&mut perm), val);
        });
        r
    }

    pub fn set(&self, val: T)
        requires
            self.inv(val),
    {
        proof {
            use_type_invariant(self);
        }
        open_local_invariant!(self.perm_inv.borrow() => perm => {
            self.pcell.replace(Tracked(&mut perm), val);
        });
    }
}

impl<T: Copy, Pred: CellPredicate<T>> Cell<T, Pred> {
    pub fn get(&self) -> (val: T)
        ensures
            self.inv(val),
    {
        proof {
            use_type_invariant(self);
        }
        let r;
        open_local_invariant!(self.perm_inv.borrow() => perm => {
            r = *self.pcell.borrow(Tracked(&perm));
        });
        r
    }
}

fn main() { }

}
