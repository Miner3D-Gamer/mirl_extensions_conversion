// f64

const impl FromPatch<f16> for f64 {
    fn from_value(v: f16) -> Self {
        (v as Self)
    }
}
const impl FromPatch<f16> for f128 {
    fn from_value(v: f16) -> Self {
        (v)
    }
}
const impl FromPatch<Self> for f16 {
    fn from_value(v: Self) -> Self {
        (v as Self)
    }
}
const impl FromPatch<f16> for f32 {
    fn from_value(v: f16) -> Self {
        (v as Self)
    }
}
use crate::FromPatch;
