use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use num_bigint::BigInt;
pub use num_rational::BigRational;
use num_traits::{ConstOne, ConstZero, Num, One, Signed, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberState {
    /// A number state indicating that this is only an approximation rather than a real value
    /// This can be caused square roots etc
    Approximation,
    /// A number state indicating that this is exact
    Exact,
}

impl NumberState {
    fn combine(&self, other: Self) -> Self {
        match (self, other) {
            (NumberState::Exact, NumberState::Exact) => NumberState::Exact,
            _ => NumberState::Approximation,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    value: BigRational,
    state: NumberState,
}

impl Number {
    pub fn from_num(rational: impl Into<BigRational>) -> Self {
        Self {
            value: rational.into(),
            state: NumberState::Exact,
        }
    }

    pub fn try_from_num<T: TryInto<BigRational>>(rational: T) -> Result<Self, T::Error> {
        Ok(Self {
            value: rational.try_into()?,
            state: NumberState::Exact,
        })
    }

    pub fn state(&self) -> NumberState {
        self.state
    }
}

impl Number {
    pub const fn pi() -> Self {
        Self {
            value: BigRational::new_raw(
                BigInt::new_const(1886663519),
                BigInt::new_const(600543650),
            ),
            state: NumberState::Approximation,
        }
    }
}

impl Num for Number {
    type FromStrRadixErr = <BigRational as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self {
            value: BigRational::from_str_radix(str, radix)?,
            state: NumberState::Exact,
        })
    }
}

impl Signed for Number {
    fn abs(&self) -> Self {
        Self {
            value: self.value.abs(),
            ..*self
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self {
            value: self.value.abs_sub(&other.value),
            state: self.state.combine(other.state),
        }
    }

    fn signum(&self) -> Self {
        Self {
            value: self.value.signum(),
            ..*self
        }
    }

    fn is_positive(&self) -> bool {
        self.value.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.value.is_negative()
    }
}

impl One for Number {
    fn one() -> Self {
        Self::ONE
    }
}

impl ConstOne for Number {
    const ONE: Self = Self {
        value: BigRational::ONE,
        state: NumberState::Exact,
    };
}

impl Zero for Number {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl ConstZero for Number {
    const ZERO: Self = Self {
        value: BigRational::ZERO,
        state: NumberState::Exact,
    };
}

macro_rules! number_binary_op_impl {
    ($trait:ident, $func:ident) => {
        impl $trait<Number> for Number {
            type Output = Number;

            fn $func(self, rhs: Number) -> Self::Output {
                Self::Output {
                    value: self.value.$func(rhs.value),
                    state: self.state.combine(rhs.state),
                }
            }
        }
    };
}

macro_rules! number_binary_op_assign_impl {
    ($trait:ident, $func:ident) => {
        impl $trait<Number> for Number {
            fn $func(&mut self, rhs: Number) {
                self.value.$func(rhs.value);
                self.state = self.state.combine(rhs.state);
            }
        }
    };
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: self.value.neg(),
            ..self
        }
    }
}

number_binary_op_impl!(Add, add);
number_binary_op_impl!(Sub, sub);
number_binary_op_impl!(Div, div);
number_binary_op_impl!(Mul, mul);
number_binary_op_impl!(Rem, rem);

number_binary_op_assign_impl!(AddAssign, add_assign);
number_binary_op_assign_impl!(SubAssign, sub_assign);
number_binary_op_assign_impl!(DivAssign, div_assign);
number_binary_op_assign_impl!(MulAssign, mul_assign);
number_binary_op_assign_impl!(RemAssign, rem_assign);
