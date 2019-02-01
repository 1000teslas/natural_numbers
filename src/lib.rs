//! # Natural Numbers
//!
//! `natural_numbers` provides natural numbers and operations on them, based on the successor operation.

/// Trait for types implementing natural numbers.
pub trait Nat
where
    Self: Sized + Clone,
{
    /// Converts a value of type Nat into the corresponding value of type u32.
    /// # Examples
    /// ```
    /// use natural_numbers::{Nat,U32Nat};
    ///
    /// let zero = U32Nat::zero();
    /// assert_eq!(zero.value(), 0);
    /// ```
    fn value(&self) -> u32;

    /// Converts a value of type u32 into the corresponding value of type Nat.
    /// # Examples
    /// ```
    /// use natural_numbers::{Nat,U32Nat};
    ///
    /// assert_eq!(U32Nat::wrap(0), U32Nat::zero());
    /// ```
    fn wrap(n: u32) -> Self;

    fn zero() -> Self;

    fn is_zero(&self) -> bool;

    /// The successor of a value.
    /// # Examples
    /// ```
    /// use natural_numbers::{Nat,U32Nat};
    ///
    /// assert_eq!(U32Nat::wrap(1), U32Nat::zero().succ());
    /// ```
    fn succ(&self) -> Self;

    /// The value of which this is a successor, or None if the value is zero.
    /// # Examples
    /// ```
    /// use natural_numbers::{Nat,U32Nat};
    ///
    /// let one = U32Nat::zero().succ();
    /// assert_eq!(one.pred().unwrap(), U32Nat::zero());
    /// ```
    fn pred(&self) -> Option<Self> {
        match self.value() {
            0 => None,
            n => Some(Self::wrap(n - 1)),
        }
    }

    /// The sum of two values.
    /// # Examples
    /// ```
    /// use natural_numbers::{Nat,U32Nat};
    ///
    /// let two = U32Nat::wrap(2);
    /// let three = U32Nat::wrap(3);
    /// let five = U32Nat::wrap(5);
    /// assert_eq!(two.add(&three), five);
    fn add(&self, n: &Self) -> Self {
        match n.pred() {
            None => self.clone(),
            Some(m) => self.add(&m).succ(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct U32Nat(u32);

// impl Ord for U32Nat {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match other.pred() {
//             None => match self.pred() {
//                 None => Ordering::Equal,
//                 Some(_) => Ordering::Greater,
//             },
//             Some(n) => match self.pred() {
//                 None => Ordering::Less,
//                 Some(m) => m.cmp(&n),
//             },
//         }
//     }
// }

impl Nat for U32Nat {
    fn value(&self) -> u32 {
        self.0
    }

    fn wrap(n: u32) -> Self {
        U32Nat(n)
    }

    fn zero() -> Self {
        U32Nat(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn succ(&self) -> Self {
        U32Nat(self.0 + 1)
    }
}
