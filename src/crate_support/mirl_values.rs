impl FromPatch<i8> for Number {
    fn from_value(v: i8) -> Self {
        Self::Int(i128::from(v))
    }
}
impl FromPatch<i16> for Number {
    fn from_value(v: i16) -> Self {
        Self::Int(i128::from(v))
    }
}
impl FromPatch<i32> for Number {
    fn from_value(v: i32) -> Self {
        Self::Int(i128::from(v))
    }
}
impl FromPatch<i64> for Number {
    fn from_value(v: i64) -> Self {
        Self::Int(i128::from(v))
    }
}
impl FromPatch<i128> for Number {
    fn from_value(v: i128) -> Self {
        Self::Int(v)
    }
}
impl FromPatch<u8> for Number {
    fn from_value(v: u8) -> Self {
        Self::Int(i128::from(v))
    }
}
impl FromPatch<u16> for Number {
    fn from_value(v: u16) -> Self {
        Self::Int(i128::from(v))
    }
}
impl FromPatch<u32> for Number {
    fn from_value(v: u32) -> Self {
        Self::Int(i128::from(v))
    }
}
impl FromPatch<u64> for Number {
    fn from_value(v: u64) -> Self {
        Self::Int(i128::from(v))
    }
}
impl FromPatch<u128> for Number {
    fn from_value(v: u128) -> Self {
        if v <= i128::MAX as u128 {
            Self::Int(v as i128)
        } else {
            Self::BigInt(BigInt::from(v))
        }
    }
}
impl FromPatch<f32> for Number {
    fn from_value(v: f32) -> Self {
        Self::Float(f64::from(v))
    }
}
impl FromPatch<f64> for Number {
    fn from_value(v: f64) -> Self {
        Self::Float(v)
    }
}
impl FromPatch<BigInt> for Number {
    fn from_value(v: BigInt) -> Self {
        Self::maybe_narrow_bigint(v)
    }
}
impl FromPatch<BigFloat> for Number {
    fn from_value(v: BigFloat) -> Self {
        Self::maybe_narrow_bigfloat(v)
    }
}

use mirl_values_core::value::Number;
use num_bigfloat::BigFloat;
use num_bigint::BigInt;

use crate::*;
