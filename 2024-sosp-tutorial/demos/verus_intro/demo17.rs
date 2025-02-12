// vim: set filetype=verus laststatus=0:
use vstd::prelude::*;
verus! {

struct Point {
  x: i64,
}

impl Point {
  fn move_x(self, dx: i64) -> (r: Point)
    requires i64::MIN <= self.x + dx < i64::MAX,
    ensures true // change me
  {
    Point { x: self.x + dx }
  }

  spec fn center(self) -> bool {
    self.x == 0
  }
}

fn move_point() {
  let p1 = Point { x: 10 };
  let p2 = p1.move_x(-10);
  assert(p2.center());
}

} // verus!
