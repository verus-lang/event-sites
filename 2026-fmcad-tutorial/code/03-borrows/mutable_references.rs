fn example() {
    let mut a = 5;
    let a_ref = &mut a;
    *a_ref = 20;

    assert(a == 20);
}
