use vstd::prelude::*;

verus!{

/**************************************
 * Stubbed interface to channels
 * (verified channel implementation requires an additional technical feature not on mainline Verus)
 */

#[verifier::external_body]
#[verifier::reject_recursive_types(T)]
struct Sender<T> { _t: std::marker::PhantomData<T> }

#[verifier::external_body]
#[verifier::reject_recursive_types(T)]
struct Receiver<T> { _t: std::marker::PhantomData<T> }

impl<T> Sender<T> {
    #[verifier::prophetic]
    uninterp spec fn remaining(self) -> Seq<T>;

    #[verifier::external_body]
    fn send(&mut self, payload: T)
        ensures
            old(self).remaining().len() > 0,
            payload == old(self).remaining()[0],
            final(self).remaining() == old(self).remaining().drop_first(),
    {
        unimplemented!()
    }
}

impl<T> Receiver<T> {
    #[verifier::prophetic]
    uninterp spec fn remaining(self) -> Seq<T>;

    #[verifier::external_body]
    fn recv(&mut self) -> (payload: T)
        ensures
            old(self).remaining().len() > 0,
            payload == old(self).remaining()[0],
            final(self).remaining() == old(self).remaining().drop_first(),
    {
        unimplemented!()
    }
}

#[verifier::external_body]
fn new_channel<T>() -> ((producer, consumer): (Sender<T>, Receiver<T>))
    ensures producer.remaining() == consumer.remaining()
{
    unimplemented!()
}

/**************************************
 * Example usage
 */

fn test_channels() -> Result<(), ()> {
    let (s, r) = new_channel::<u64>();

    let t1 = vstd::thread::spawn(||
        ensures s.remaining().len() >= 3,
            s.remaining()[0] == 1,
            s.remaining()[1] == 2,
            s.remaining()[2] == 3,
    {
        let mut s = s;
        s.send(1);
        s.send(2);
        s.send(3);
    });

    let t2 = vstd::thread::spawn(|| -> (ret: (u64, u64, u64))
        ensures s.remaining().len() >= 3,
            s.remaining()[0] == ret.0,
            s.remaining()[1] == ret.1,
            s.remaining()[2] == ret.2,
    {
        let mut r = r;
        let x = r.recv();
        let y = r.recv();
        let z = r.recv();
        return (x, y, z);
    });

    t1.join()?;
    let (x, y, z) = t2.join()?;

    assert(x == 1);
    assert(y == 2);
    assert(z == 3);

    Ok(())
}

}

fn main() { }
