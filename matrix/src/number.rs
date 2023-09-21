use std::ops::{Add, AddAssign, Mul, MulAssign};

pub trait Number<T>:
    Sized + Default + Copy + Mul<Output = T> + MulAssign + Add<Output = T> + AddAssign
{
}
impl Number<u64> for u64 {}
impl Number<u32> for u32 {}
