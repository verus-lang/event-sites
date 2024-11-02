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
  ta.v_align == VAlign::Top &&
  ta.h_align == HAlign::Left
}

// ===

//- proof fn try_to_move_to_top(ta1: TextAlign) -> TextAlign
//- {
//-   let mut ta2 = ta1;
//-   ta2.v_align = VAlign::Top;
//-   ta2
//- }

/*+*/ spec fn try_to_move_to_top(ta1: TextAlign) -> TextAlign
/*+*/ {
/*+*/   TextAlign { h_align: ta1.h_align, v_align: VAlign::Top }
/*+*/ }

proof fn ta_lemma(ta1: TextAlign)
  requires ta1.h_align == HAlign::Left
{
  let ta2 = try_to_move_to_top(ta1);
  assert(top_left(ta2));
}

} // verus
