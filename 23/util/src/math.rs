use super::*;
use num::NumCast;

pub trait Number: Debug + Clone + NumCast {
    fn sqrt(self) -> f64 {
        f64::sqrt(self.uinton())
    }

    fn floor(number: f64) -> Self {
        number.floor().uinton()
    }

    fn ceil(number: f64) -> Self {
        number.ceil().uinton()
    }
}
// impl Number for isize {}
// impl Number for i8 {}
// impl Number for i16 {}
// impl Number for i32 {}
// impl Number for i64 {}
// impl Number for i128 {}
// impl Number for usize {}
// impl Number for u8 {}
// impl Number for u16 {}
// impl Number for u32 {}
// impl Number for u64 {}
// impl Number for u128 {}
// impl Number for f64 {}

macro_rules! impl_number {
    ($t:ty) => {
        impl Number for $t {}
        impl CostFunction for $t {
            fn inf() -> Self {
                <$t>::MAX
            }
            fn zero() -> Self {
                0
            }
        }
    };
}

impl_number!(isize);
impl_number!(i8);
impl_number!(i16);
impl_number!(i32);
impl_number!(i64);
impl_number!(i128);
impl_number!(usize);
impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(u128);
impl Number for f32 {}
impl Number for f64 {}
pub trait UnsafeFromNum<A>: Sized
where
    A: Number,
{
    fn ufromn(a: A) -> Self;
}

impl<A: Number, B: Number> UnsafeFromNum<A> for B {
    fn ufromn(a: A) -> B {
        match B::from(a.clone()) {
            Some(v) => v,
            None => panic!(
                "Unable to convert {:?} from {} to {}",
                a,
                type_name::<A>(),
                type_name::<B>()
            ),
        }
    }
}

pub trait UnsafeIntoNum<B>: Sized
where
    B: Number,
{
    fn uinton(self) -> B;
}

impl<B, A> UnsafeIntoNum<B> for A
where
    A: Number,
    B: Number + UnsafeFromNum<A>,
{
    fn uinton(self) -> B {
        B::ufromn(self)
    }
}
