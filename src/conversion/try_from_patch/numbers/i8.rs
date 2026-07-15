// i8
const impl TryFromPatch<i8> for u8 {
    fn try_from_value(v: i8) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
const impl TryFromPatch<i8> for u16 {
    fn try_from_value(v: i8) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
const impl TryFromPatch<i8> for u32 {
    fn try_from_value(v: i8) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
const impl TryFromPatch<i8> for u64 {
    fn try_from_value(v: i8) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
const impl TryFromPatch<i8> for u128 {
    fn try_from_value(v: i8) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
const impl TryFromPatch<i8> for usize {
    fn try_from_value(v: i8) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
use crate::TryFromPatch;
