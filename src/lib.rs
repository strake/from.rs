#![no_std]

extern crate void;

use void::Void;

pub trait TryFrom<A>: Sized {
    type Error;
    fn try_from(A) -> Result<Self, Self::Error>;
}

impl<A, T: From<A>> TryFrom<A> for T {
    type Error = Void;

    #[inline(always)]
    fn try_from(a: A) -> Result<Self, Self::Error> { Ok(Self::from(a)) }
}
