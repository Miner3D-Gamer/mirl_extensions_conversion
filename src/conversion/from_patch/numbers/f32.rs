use crate::FromPatch;

// f32
const impl FromPatch<f32> for f16 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
const impl FromPatch<f32> for f64 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}

const impl FromPatch<f32> for f128 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
