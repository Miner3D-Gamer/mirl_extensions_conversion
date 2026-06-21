use mirl_extensions_core::Bounded;

use crate::IntoPatch;

/// Check if the upper and lower bounds fit inside the bounds of another value
pub const trait BoundsFitInBounds {
    #[must_use]
    /// Check if the upper and lower bounds fit inside the bounds of another value
    ///
    /// This assumes that the other value is the same size or smaller
    /// If this isn't true this function may crash
    fn bounds_fit_in_bounds<Other: Bounded + [const] IntoPatch<Self>>() -> bool
    where
        Self: Bounded + [const] core::cmp::PartialOrd + Copy;

    /// Check if the value can fit inside the upper and lower bounds of another value
    ///
    /// This assumes that the other value is the same size or smaller
    /// If this isn't true this function may crash
    fn value_fits_in_bounds<Other: Bounded + [const] IntoPatch<Self>>(
        &self,
    ) -> bool
    where
        Self: Bounded + [const] core::cmp::PartialOrd + Copy;
}

impl<S: Bounded + [const] core::cmp::PartialOrd + Copy> const BoundsFitInBounds
    for S
{
    fn bounds_fit_in_bounds<Other: Bounded + [const] IntoPatch<Self>>() -> bool
    {
        Self::MIN >= Other::MIN.into_value()
            && Self::MAX <= Other::MAX.into_value()
    }

    fn value_fits_in_bounds<Other: Bounded + [const] IntoPatch<Self>>(
        &self,
    ) -> bool {
        *self >= Other::MIN.into_value() && *self <= Other::MAX.into_value()
    }
}
