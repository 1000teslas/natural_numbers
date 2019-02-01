use natural_numbers::{Nat, U32Nat};

fn main() {
    let zero = U32Nat::zero();
    let one = U32Nat::zero().succ();
    assert_eq!(zero.add(&one), one);
}
