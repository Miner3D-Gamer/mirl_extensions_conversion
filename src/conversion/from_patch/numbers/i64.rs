// i64
const impl FromPatch<i64> for i128 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
const impl FromPatch<i64> for f32 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
const impl FromPatch<i64> for f64 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
const impl FromPatch<i64> for f16 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
const impl FromPatch<i64> for f128 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
use crate::FromPatch;
