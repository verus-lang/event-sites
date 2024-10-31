use vstd::prelude::*;
use state_machines_macros::tokenized_state_machine;

tokenized_state_machine!{
    VerusSyncAgreement2 {
        //// The state

        fields {
            #[sharding(variable)]
            pub value1: u64,

            #[sharding(variable)]
            pub value2: u64,
        }

        /// The operations

        init!{
            initialize(initial_value: u64) {
                init value1 = initial_value;
                init value2 = initial_value;
            }
        }

        transition!{
            update_both(new_value: u64) {
                update value1 = new_value;
                update value2 = new_value;
            }
        }

        property!{
            both_agree() {
                assert pre.value1 == pre.value2;
            }
        }

        //// The space-reaching invariant

        #[invariant]
        pub spec fn the_space_reaching_invariant(&self) -> bool {
            self.value1 == self.value2
        }

        //// Invariant proofs (these are trivial)

        #[inductive(initialize)]
        fn initialize_inductive(post: Self, initial_value: u64) { }
       
        #[inductive(update_both)]
        fn update_both_inductive(pre: Self, post: Self, new_value: u64) { }
    }
}

fn main() { }
