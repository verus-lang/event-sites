// vim: set filetype=verus laststatus=0:

use vstd::prelude::*;
verus! {

enum HAlign {
  Left,
  Center,
  Right,
}

enum VAlign {
  Top,
  Middle,
  Bottom,
}

struct TextAlign {
  h_align: HAlign,
  v_align: VAlign,
}

spec fn top_left(ta: TextAlign) -> bool {
  &&& ta.v_align == VAlign::Top
  &&& ta.h_align == HAlign::Left
}

// ===

proof fn top_and_left(ta: TextAlign)
  requires
    ta.v_align == VAlign::Top,
    /*+*/ ta.h_align == HAlign::Left,
  ensures
    top_left(ta),
{
  //+- assert(ta.v_align == VAlign::Top);
  //+- assert(ta.h_align == HAlign::Left);
}

} // verus!
