(assert
 (not (=>
   (= max! (ite
     (> a! b!)
     a!
     b!
   ))
   (and
    (>= max! a!)
    (>= max! b!)
))))
(check-sat)
