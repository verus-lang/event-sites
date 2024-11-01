use vstd::prelude::*;
verus! {

struct Point {
  x: int,
  y: int,
}

proof fn same_x() {
  let p1 = Point { x: 10, y: 20 };
  let p2 = Point { x: 10, y: 30 };
  assert(p1.x == p2.x);
}

proof fn collection_of_points() {
  let s = set![
    Point { x: 10, y: 20 },
    Point { x: 10, y: 30 },
  ];
  assert(s.len() == 2);
  assert(s has Point { x: 10, y: 20 });
}

// ===

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

} // verus!
