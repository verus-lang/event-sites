(assert
  (not (=>
    (= a@0 5)
    (=>
     (has_type a_ref@0 (MUTREF $ (SINT 32)))
     (=>
      (= (%I (mut_ref_current% a_ref@0)) a@0)
      (=>
       (= a@1 (%I (mut_ref_future% a_ref@0)))
       (=>
        (= a_ref@1 (mut_ref_update_current% a_ref@0 (I 20)))
        (=>
         (has_type a_ref@1 (MUTREF $ (SINT 32)))
         (=>
          (has_resolved $ (MUTREF $ (SINT 32)) a_ref@1)
          (= a@1 20)
 )))))))))
 (check-sat)
