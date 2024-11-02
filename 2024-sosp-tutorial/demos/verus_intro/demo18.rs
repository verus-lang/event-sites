// vim: set filetype=verus laststatus=0:

use vstd::prelude::*; verus! {

fn view_of_vec() {
  let v: Vec<u64> = vec![1,2,3];
  let ghost s: Seq<u64> = v.view();
  assert(s.contains(2));
  assert(v@.contains(2));
  assert(forall|i:int| 0 <= i < s.len() ==> s[i] <= 3);
  assert(forall|i:int| 0 <= i < v@.len() ==> v@[i] <= 3);
}

fn find_elt(elt: u64, elts: Vec<u64>) -> (result: Option<usize>)
  ensures
    match result {
      Some(i) => i < elts@.len() && elts@[i as int] == elt,
      None => !elts@.contains(elt),
    },
{
  //-2- let mut i = 3;
  /*+2+*/ let mut i = 0;
  while i < elts.len() 
    invariant
      /*+1+*/ 0 <= i <= elts@.len(),
      //-1- forall |j| 0 <= j < elts.len() ==> elts@[j] != elt,
      /*+3+*/ forall |j| 0 <= j < i ==> elts@[j] != elt,
  {
    if elts[i] == elt {
      return Some(i);
    }
    i += 1;
  }
  None
}

}
