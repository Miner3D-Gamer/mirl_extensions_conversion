use mirl_extensions_core::{Bounded, LowerBounded, UpperBounded};

use crate::IntoPatch;

/// Check if the upper and lower bounds fit inside the bounds of another value
pub const trait BoundsFitInBounds {
    #[must_use]
    /// Check if the upper and lower bounds fit inside the bounds of another value
    ///
    /// This assumes that the other value is the same size or smaller
    /// If this isn't true this function may crash
    fn bounds_fit_in_bounds<
        Other: [const] LowerBounded + [const] UpperBounded + [const] IntoPatch<Self>,
    >() -> bool
    where
        Self: [const] LowerBounded + [const] UpperBounded + [const] core::cmp::PartialOrd + Copy;

    /// Check if the value can fit inside the upper and lower bounds of another value
    ///
    /// This assumes that the other value is the same size or smaller
    /// If this isn't true this function may crash
    fn value_fits_in_bounds<
        Other: [const] LowerBounded + [const] UpperBounded + [const] IntoPatch<Self>,
    >(
        &self,
    ) -> bool
    where
        Self: Bounded + [const] core::cmp::PartialOrd + Copy;
}

const impl<S: [const] LowerBounded + [const] UpperBounded + [const] core::cmp::PartialOrd + Copy>
    BoundsFitInBounds for S
{
    fn bounds_fit_in_bounds<
        Other: [const] LowerBounded + [const] UpperBounded + [const] IntoPatch<Self>,
    >() -> bool {
        Self::min_bound() >= Other::min_bound().into_value()
            && Self::max_bound() <= Other::max_bound().into_value()
    }

    fn value_fits_in_bounds<
        Other: [const] LowerBounded + [const] UpperBounded + [const] IntoPatch<Self>,
    >(
        &self,
    ) -> bool {
        *self >= Other::min_bound().into_value() && *self <= Other::max_bound().into_value()
    }
}
