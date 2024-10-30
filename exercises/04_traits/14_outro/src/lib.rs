use std::ops::Add;
use std::cmp::PartialEq;
// TODO: Define a new `SaturatingU16` type.
//   It should be possible to print its debug representation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SaturatingU16 {
//   It should hold a `u16` value.
    value: u16
}
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
    fn ne(&self, other: &u16) -> bool {
        self.value != *other
    }
}

//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
impl From<u16> for SaturatingU16 {

    fn from(item: u16) -> Self {
        Self {value: item}
    }
}
impl From<u8> for SaturatingU16 {

    fn from(item: u8) -> Self {
        Self {value: item as u16}
    }
}

impl From<&u16> for SaturatingU16 {

    fn from(item: &u16) -> Self {
        Self {value: *item }
    }
}

impl From<&u8> for SaturatingU16 {

    fn from(item: &u8) -> Self {
        Self {value: *item as u16 }
    }
}

//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
impl Add<SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { value: self.value.saturating_add(rhs.value)}
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: u16) -> SaturatingU16 {
        Self { value: self.value.saturating_add(rhs)}
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &u16) -> Self {
        Self { value: self.value.saturating_add(*rhs)}
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self { value: self.value.saturating_add((*rhs).value)}
    }
}

// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
