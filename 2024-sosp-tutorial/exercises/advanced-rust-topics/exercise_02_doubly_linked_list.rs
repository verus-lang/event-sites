// Verus tutorial - 'Advanced Topics' Exercise: doubly-linked list
//
// This file provides an implementation of a doubly-ended queue implemented
// as a doubly-linked list. The intent is to illustrate the use of Verus
// 'memory permissions' to verify a data structure that cannot be expressed
// through a 'normal' ownership discipline.
//
// Most of the implementation has been filled in for you. There are 2 blanks to fill in;
// search for 'EXERCISE' in this file.

#![verifier::loop_isolation(false)]

use vstd::prelude::*;

verus! {

mod doubly_linked_list {
    use vstd::prelude::*;
    use vstd::simple_pptr::*;
    use vstd::raw_ptr::MemContents;
    use vstd::assert_by_contradiction;
    use super::todo;

    ////// Definition of the core data structures:

    // Single node in the list
    struct Node<V> {
        prev: Option<PPtr<Node<V>>>,
        next: Option<PPtr<Node<V>>>,
        payload: V,
    }

    // Doubly-linked list
    // Contains head pointer, tail pointer
    // and in ghost code, tracks all the pointers and all the PointsTo permissions
    // to access the nodes
    pub struct DoublyLinkedList<V> {
        // physical data:
        head: Option<PPtr<Node<V>>>,
        tail: Option<PPtr<Node<V>>>,

        // ghost and tracked data:
        ghost_state: Tracked<GhostState<V>>,
    }

    pub tracked struct GhostState<V> {
        ghost length: int,
        tracked points_to_map: Map<int, PointsTo<Node<V>>>,
    }

    impl<V> DoublyLinkedList<V> {
        // Main spec definitions of the doubly-linked list

        /// The pointer to the i^th node
        spec fn ptr_at(&self, i: int) -> PPtr<Node<V>> {
            self.ghost_state@.points_to_map[i].pptr()
        }

        /// Pointer to the node of index (i-1), or None if i is 0.
        spec fn prev_of(&self, i: int) -> Option<PPtr<Node<V>>> {
            if i == 0 {
                None
            } else {
                Some(self.ptr_at(i - 1))
            }
        }

        /// Pointer to the node of index (i+1), or None if i is the last index.
        spec fn next_of(&self, i: int) -> Option<PPtr<Node<V>>> {
            if i + 1 == self.ghost_state@.length {
                None
            } else {
                Some(self.ptr_at((i + 1) as int))
            }
        }

        /// Node at index `i` is well-formed
        spec fn well_formed_node(&self, i: int) -> bool {
            &&& self.ghost_state@.points_to_map.dom().contains(i)
            &&& self.ghost_state@.points_to_map[i].mem_contents() matches MemContents::Init(node)
                  && node.prev == self.prev_of(i)
                  && node.next == self.next_of(i)
        }

        /// Linked list is well-formed
        pub closed spec fn well_formed(&self) -> bool {
            &&& self.ghost_state@.length >= 0
            // Every node from 0 .. len - 1 has an entry in the points_to_map
            &&& (forall|i: int| 0 <= i && i < self.ghost_state@.length ==> self.ghost_state@.points_to_map.dom().contains(i))
            // Every node from 0 .. len - 1 is well-formed
            &&& (forall|i: int| 0 <= i && i < self.ghost_state@.length ==> self.well_formed_node(i))
            // Head and tail pointers are correct
            &&& (if self.ghost_state@.length == 0 {
                // If the list is empty, then the `head` and `tail` pointers are both None
                self.head.is_none() && self.tail.is_none()
            } else {
                // If the list is non-empty, then `head` and `tail` pointers point to the
                // the first and last nodes.
                &&& self.head == Some(self.ptr_at(0))
                &&& self.tail == Some(self.ptr_at(self.ghost_state@.length - 1))
            })
        }

        /// Representation of this list as a sequence
        pub closed spec fn view(&self) -> Seq<V> {
            Seq::<V>::new(
                self.ghost_state@.length as nat,
                |i: int| { self.ghost_state@.points_to_map[i].value().payload },
            )
        }

        //// Interface of executable functions

        /// Construct a new, empty, doubly-linked list.
        pub fn new() -> (s: Self)
            ensures
                s.well_formed(),
                s@.len() == 0,
        {
            DoublyLinkedList {
                ghost_state: Tracked(GhostState {
                    length: 0,
                    points_to_map: Map::tracked_empty(),
                }),
                head: None,
                tail: None,
            }
        }

        /// Insert one node, assuming the linked list is empty.
        fn insert_node_into_empty_list(&mut self, v: V)
            requires
                old(self).well_formed(),
                old(self).ghost_state@.length == 0,
            ensures
                self.well_formed(),
                self@ =~= old(self)@.push(v),
        {
            // EXERCISE 1. Implement `insert_node_into_empty_list`.

            // Step 1. Allocate a fresh node.
            // You may want to see the docs for PPtr:
            // https://verus-lang.github.io/verus/verusdoc/vstd/simple_pptr/struct.PPtr.html

            // ...

            // Step 2. Update all the fields.

            // self.tail = ...;
            // self.head = ...;

            proof {
                // Step 3. Update all the ghost fields to make sure the invariants.
                // are maintained. This is a purely ghost step,
                // so it happens inside the `proof { ... }` block.
                //
                // Hints:
                //
                // 1. To modify the GhostState, you can get a mutable reference
                //    via `self.ghost_state.borrow_mut()`
                //
                //    e.g., `self.ghost_state.borrow_mut().field = xyz;`
                //      or, `self.ghost_state.borrow_mut().field.mutating_method(...);`
                //
                // 2. To manipulate a Map<...>, check the docs at:
                //    https://verus-lang.github.io/verus/verusdoc/vstd/map/struct.Map.html
                //    Look for the methods that start with the `tracked_` prefix.
            }
        }

        /// Insert a value to the end of the list
        pub fn push_back(&mut self, v: V)
            requires
                old(self).well_formed(),
            ensures
                self.well_formed(),
                self@ == old(self)@.push(v),
        {
            match self.tail {
                None => {
                    // Special case: list is empty
                    proof {
                        // Show that the `self.tail == None` implies the list is empty
                        assert_by_contradiction!(self.ghost_state@.length == 0,
                        {
                            assert(self.well_formed_node(self.ghost_state@.length)); // trigger
                        });
                    }
                    self.insert_node_into_empty_list(v);
                }
                Some(old_tail_ptr) => {
                    proof {
                        assert(self.well_formed_node(self.ghost_state@.length - 1)); // trigger
                    }

                    // Allocate a new node to go on the end. It's 'prev' field points
                    // to the old tail pointer.
                    let (new_tail_ptr, Tracked(new_tail_pointsto)) = PPtr::<Node<V>>::new(
                        Node::<V> { prev: Some(old_tail_ptr), next: None, payload: v },
                    );

                    // Update the 'next' pointer of the previous tail node
                    // This is all equivalent to `(*old_tail_ptr).next = new_tail_ptr;`
                    let tracked mut old_tail_pointsto: PointsTo<Node<V>> =
                        self.ghost_state.borrow_mut().points_to_map.tracked_remove(self.ghost_state@.length - 1);
                    let mut old_tail_node = old_tail_ptr.take(Tracked(&mut old_tail_pointsto));
                    old_tail_node.next = Some(new_tail_ptr);
                    old_tail_ptr.put(Tracked(&mut old_tail_pointsto), old_tail_node);
                    proof {
                        self.ghost_state.borrow_mut().points_to_map.tracked_insert(
                            self.ghost_state@.length - 1,
                            old_tail_pointsto,
                        );
                    }

                    // Update `self.tail`
                    self.tail = Some(new_tail_ptr);

                    proof {
                        // Put the new tail's PointsTo into the map
                        self.ghost_state.borrow_mut().points_to_map.tracked_insert(self.ghost_state@.length, new_tail_pointsto);
                        self.ghost_state@.length = self.ghost_state@.length + 1;

                        // Additional proof work to help the solver show that
                        // `self.well_formed()` has been restored.
                        if self.ghost_state@.length >= 3 {
                            assert(self.ptr_at(self.ghost_state@.length - 3)
                                == old(self).ptr_at(self.ghost_state@.length - 3));
                        }
                        assert(self.well_formed_node(self.ghost_state@.length - 2));
                        assert(self.well_formed_node(self.ghost_state@.length - 1));
                        assert(forall|i: int| 0 <= i < self.ghost_state@.length && old(self).well_formed_node(i)
                            ==> self.well_formed_node(i));
                        assert forall|i: int| 0 <= i && i < self.ghost_state@.length as int - 1
                            implies old(self)@[i] == self@[i]
                        by {
                            assert(old(self).well_formed_node(i));  // trigger
                        }
                        assert(self@ =~= old(self)@.push(v));

                        assert(self.well_formed());
                    }
                }
            }
        }

        /// Take a value from the end of the list. Requires the list to be non-empty.
        pub fn pop_back(&mut self) -> (v: V)
            requires
                old(self).well_formed(),
                old(self)@.len() > 0,
            ensures
                self.well_formed(),
                self@ == old(self)@.drop_last(),
                v == old(self)@[old(self)@.len() as int - 1],
        {
            assert(self.well_formed_node(self.ghost_state@.length - 1));

            // Deallocate the last node in the list and get the payload.
            // Note self.tail.unwrap() will always succeed because of the precondition `len > 0`
            let last_ptr = self.tail.unwrap();
            let tracked last_pointsto = self.ghost_state.borrow_mut().points_to_map.tracked_remove(
                self.ghost_state@.length - 1,
            );
            let last_node = last_ptr.into_inner(Tracked(last_pointsto));
            let v = last_node.payload;

            match last_node.prev {
                None => {
                    // If this was the *only* node in the list,
                    // we set both `head` and `tail` to None
                    self.tail = None;
                    self.head = None;
                    proof {
                        assert_by_contradiction!(self.ghost_state@.length == 1,
                        {
                            assert(old(self).well_formed_node(self.ghost_state@.length - 2)); // trigger
                        });
                    }
                },
                Some(penultimate_ptr) => {
                    assert(old(self)@.len() >= 2);
                    assert(old(self).well_formed_node(self.ghost_state@.length - 2));

                    // Otherwise, we need to set the 'tail' pointer to the (new) tail pointer,
                    // i.e., the pointer that was previously the second-to-last pointer.
                    self.tail = Some(penultimate_ptr);

                    // And we need to set the 'next' pointer of the new tail node to None.
                    let tracked mut penultimate_pointsto =
                        self.ghost_state.borrow_mut().points_to_map.tracked_remove(self.ghost_state@.length - 2);
                    let mut penultimate_node = penultimate_ptr.take(Tracked(&mut penultimate_pointsto));
                    penultimate_node.next = None;
                    penultimate_ptr.put(Tracked(&mut penultimate_pointsto), penultimate_node);
                    proof {
                        self.ghost_state.borrow_mut().points_to_map.tracked_insert(
                            self.ghost_state@.length - 2,
                            penultimate_pointsto,
                        );
                    }
                },
            }

            // Additional proof work to help the solver show that
            // `self.well_formed()` has been restored.
            proof {
                self.ghost_state@.length = self.ghost_state@.length - 1;
                if self.ghost_state@.length > 0 {
                    assert(self.well_formed_node(self.ghost_state@.length - 1));
                }
                assert(forall|i: int| 0 <= i < self@.len() && old(self).well_formed_node(i) ==> self.well_formed_node(i));
                assert forall|i: int| 0 <= i && i < self@.len() implies #[trigger] self@[i] == old(self)@.drop_last()[i] by {
                    assert(old(self).well_formed_node(i));  // trigger
                }
                assert(self@ =~= old(self)@.drop_last());

                assert(self.well_formed());
            }

            return v;
        }

        /// Insert a value to the front of the list
        pub fn push_front(&mut self, v: V)
            requires
                old(self).well_formed(),
            ensures
                self.well_formed(),
                self@ == seq![v].add(old(self)@),
        {
            match self.head {
                None => {
                    // Special case: list is empty
                    proof {
                        // Show that the `self.head == None` implies the list is empty
                        assert_by_contradiction!(self.ghost_state@.length == 0, {
                            assert(self.well_formed_node((self.ghost_state@.length - 1)));
                        });
                    }
                    self.insert_node_into_empty_list(v);
                    assert(self@ =~= seq![v].add(old(self)@));
                }
                Some(old_head_ptr) => {
                    proof {
                        assert(self.ghost_state@.length > 0);
                        assert(self.well_formed_node(0));
                    }

                    // Allocate a new node to go at the front. It's 'next' field points
                    // to the old head pointer.
                    let (new_head_ptr, Tracked(new_head_pointsto)) = PPtr::new(
                        Node::<V> { prev: None, next: Some(old_head_ptr), payload: v },
                    );

                    // Update the 'tail' pointer of the previous head node
                    // This is all equivalent to `(*old_head_ptr).next = new_head_ptr;`
                    let tracked mut old_head_pointsto =
                        self.ghost_state.borrow_mut().points_to_map.tracked_remove(0);
                    let mut old_head_node = old_head_ptr.take(Tracked(&mut old_head_pointsto));
                    old_head_node.prev = Some(new_head_ptr);
                    old_head_ptr.put(Tracked(&mut old_head_pointsto), old_head_node);
                    proof {
                        self.ghost_state.borrow_mut().points_to_map.tracked_insert(0, old_head_pointsto);
                    }

                    // Update `self.head`
                    self.head = Some(new_head_ptr);

                    proof {
                        // Put the new head's PointsTo into the map.
                        // This goes in at index 0, so we have to shift all the keys up by 1.
                        assert forall|j: int|
                            0 <= j && j < old(self)@.len() implies self.ghost_state@.points_to_map.dom().contains(
                            j,
                        ) by {
                            assert(old(self).well_formed_node(j));
                        }
                        self.ghost_state.borrow_mut().points_to_map.tracked_map_keys_in_place(
                            Map::<int, int>::new(
                                |j: int| 1 <= j && j <= old(self).view().len(),
                                |j: int| j - 1,
                            ),
                        );
                        self.ghost_state.borrow_mut().points_to_map.tracked_insert(0, new_head_pointsto);
                        self.ghost_state@.length = self.ghost_state@.length + 1;

                        // Additional proof work to help the solver show that
                        // `self.well_formed()` has been restored.
                        assert(self.well_formed_node(0));
                        assert(self.well_formed_node(1));
                        assert(forall|i: int|
                            1 <= i && i <= old(self).ghost_state@.length && old(self).well_formed_node(i - 1)
                                ==> #[trigger] self.well_formed_node(i));
                        assert forall|i: int| 1 <= i && i <= self.ghost_state@.length as int - 1
                            implies old(self)@[i - 1] == self@[i]
                        by {
                            assert(old(self).well_formed_node(i - 1));  // trigger
                        }
                        assert(self@ =~= seq![v].add(old(self)@));

                        assert(self.well_formed());
                    }
                }
            }
        }

        /// Take a value from the front of the list. Requires the list to be non-empty.
        pub fn pop_front(&mut self) -> (v: V)
            requires
                old(self).well_formed(),
                old(self).view().len() > 0,
            ensures
                self.well_formed(),
                self@ == old(self)@.subrange(1, old(self)@.len() as int),
                v == old(self)@[0],
        {
            assert(self.well_formed_node(0));

            // Deallocate the first node in the list and get the payload.
            // Note self.head.unwrap() will always succeed because of the precondition `len > 0`
            let first_ptr = self.head.unwrap();
            let tracked first_pointsto = self.ghost_state.borrow_mut().points_to_map.tracked_remove(0);
            let first_node = first_ptr.into_inner(Tracked(first_pointsto));
            let v = first_node.payload;

            match first_node.next {
                None => {
                    // If this was the *only* node in the list,
                    // we set both `head` and `tail` to None
                    self.tail = None;
                    self.head = None;
                    proof {
                        assert_by_contradiction!(self.ghost_state@.length == 1,
                        {
                            assert(old(self).well_formed_node(1)); // trigger
                        });
                    }
                }
                Some(second_ptr) => {
                    assert(old(self)@.len() >= 2);
                    assert(old(self).well_formed_node(1));

                    // Otherwise, we need to set the 'head' pointer to the (new) head pointer,
                    // i.e., the pointer that was previously the second pointer.
                    self.head = Some(second_ptr);

                    // And we need to set the 'tail' pointer of the new head node to None
                    let tracked mut second_pointsto = self.ghost_state.borrow_mut().points_to_map.tracked_remove(1);
                    let mut second_node = second_ptr.take(Tracked(&mut second_pointsto));
                    second_node.prev = None;
                    second_ptr.put(Tracked(&mut second_pointsto), second_node);
                    proof {
                        self.ghost_state.borrow_mut().points_to_map.tracked_insert(1, second_pointsto);

                        // Since we removed index 0, we need to shift all the keys down,
                        // 1 -> 0, 2 -> 1, etc.
                        assert forall |j: int|
                            1 <= j && j < old(self)@.len() implies self.ghost_state@.points_to_map.dom().contains(
                            j,
                        ) by {
                            assert(old(self).well_formed_node(j));
                        };
                        self.ghost_state.borrow_mut().points_to_map.tracked_map_keys_in_place(
                            Map::<int, int>::new(
                                |j: int| 0 <= j && j < old(self).view().len() - 1,
                                |j: int| j + 1,
                            ),
                        );
                    }
                }
            }

            // Additional proof work to help the solver show that
            // `self.well_formed()` has been restored.
            proof {
                self.ghost_state@.length = self.ghost_state@.length - 1;
                if self.ghost_state@.length > 0 {
                    assert(self.well_formed_node(0));
                }
                assert(forall|i: int| 0 <= i < self.view().len()
                    && old(self).well_formed_node(i + 1) ==> self.well_formed_node(i));
                assert forall|i: int| 0 <= i && i < self@.len()
                    implies #[trigger] self@[i] == old(self)@.subrange(1, old(self)@.len() as int)[i]
                by {
                    assert(old(self).well_formed_node(i + 1));  // trigger
                }
                assert(self@ =~= old(self)@.subrange(1, old(self)@.len() as int));

                assert(self.well_formed());
            }

            return v;
        }

        /// Get a reference to the i^th value in the list
        pub fn get<'a>(&'a self, i: usize) -> (v: &'a V)
            requires
                self.well_formed(),
                0 <= i < self@.len(),
            ensures
                *v == self@[i as int]
        {
            // EXERCISE 2. Implement `get`.
            // 
            // Hints:
            //
            // 1. You shouldn't need to mutate anything.
            //    This function operates on shared references!
            //
            // 2. To get a shared reference &GhostState, you can do `self.ghost_state.borrow()`
            //
            // 3. The precondition of `get` requires `i` to be in-range, so shouldn't need
            //    to do any error-handling! Also note that you can call `.unwrap()` on any
            //    `Option` value. Verus will allow this as long as it can prove the supplied
            //    optional value is not None.
            //
            // 4. Useful docs:
            //    - https://verus-lang.github.io/verus/verusdoc/vstd/simple_pptr/struct.PPtr.html
            //    - https://verus-lang.github.io/verus/verusdoc/vstd/map/struct.Map.html

            todo()
        }
    }
}

////// Example usage:

mod main {
    use super::doubly_linked_list::{DoublyLinkedList};

    pub fn run() {
        let mut t = DoublyLinkedList::<u32>::new();
        t.push_back(2);
        t.push_back(3);
        t.push_front(1);  // 1, 2, 3

        let elem0 = t.get(0);
        let elem1 = t.get(1);
        let elem2 = t.get(2);
        assert(*elem0 == 1);
        assert(*elem1 == 2);
        assert(*elem2 == 3);

        let x = t.pop_back();  // 3
        let y = t.pop_front();  // 1
        let z = t.pop_front();  // 2

        assert(x == 3);
        assert(y == 1);
        assert(z == 2);
    }
}

// Used as a placeholder for the exercises, where necessary
#[verifier::external_body]
fn todo<A>() -> A
    requires false
{
    todo!();
}

fn main() {
    main::run();
}

} // verus!
