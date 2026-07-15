// f64

const impl FromPatch<f128> for f64 {
    fn from_value(v: f128) -> Self {
        (v as Self)
    }
}
const impl FromPatch<Self> for f128 {
    fn from_value(v: Self) -> Self {
        (v)
    }
}
const impl FromPatch<f128> for f16 {
    fn from_value(v: f128) -> Self {
        (v as Self)
    }
}
const impl FromPatch<f128> for f32 {
    fn from_value(v: f128) -> Self {
        (v as Self)
    }
}
use crate::FromPatch;
