use crate::{TryFromPatch, TryIntoPatch};

impl<T: TryIntoPatch<O>, O> TryFromPatch<(T,)> for (O,) {
    fn try_from_value(value: (T,)) -> Option<Self> {
        Some((value.0.try_into_value()?,))
    }
}

impl<T1: TryIntoPatch<O1>, T2: TryIntoPatch<O2>, O1, O2> TryFromPatch<(T1, T2)> for (O1, O2) {
    fn try_from_value(value: (T1, T2)) -> Option<Self> {
        Some((value.0.try_into_value()?, value.1.try_into_value()?))
    }
}

impl<T1: TryIntoPatch<O1>, T2: TryIntoPatch<O2>, T3: TryIntoPatch<O3>, O1, O2, O3>
    TryFromPatch<(T1, T2, T3)> for (O1, O2, O3)
{
    fn try_from_value(value: (T1, T2, T3)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    O1,
    O2,
    O3,
    O4,
> TryFromPatch<(T1, T2, T3, T4)> for (O1, O2, O3, O4)
{
    fn try_from_value(value: (T1, T2, T3, T4)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    O1,
    O2,
    O3,
    O4,
    O5,
> TryFromPatch<(T1, T2, T3, T4, T5)> for (O1, O2, O3, O4, O5)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
> TryFromPatch<(T1, T2, T3, T4, T5, T6)> for (O1, O2, O3, O4, O5, O6)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5, T6)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7)> for (O1, O2, O3, O4, O5, O6, O7)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5, T6, T7)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    T8: TryIntoPatch<O8>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7, T8)> for (O1, O2, O3, O4, O5, O6, O7, O8)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
            value.7.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    T8: TryIntoPatch<O8>,
    T9: TryIntoPatch<O9>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9)> for (O1, O2, O3, O4, O5, O6, O7, O8, O9)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
            value.7.try_into_value()?,
            value.8.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    T8: TryIntoPatch<O8>,
    T9: TryIntoPatch<O9>,
    T10: TryIntoPatch<O10>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
            value.7.try_into_value()?,
            value.8.try_into_value()?,
            value.9.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    T8: TryIntoPatch<O8>,
    T9: TryIntoPatch<O9>,
    T10: TryIntoPatch<O10>,
    T11: TryIntoPatch<O11>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
            value.7.try_into_value()?,
            value.8.try_into_value()?,
            value.9.try_into_value()?,
            value.10.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    T8: TryIntoPatch<O8>,
    T9: TryIntoPatch<O9>,
    T10: TryIntoPatch<O10>,
    T11: TryIntoPatch<O11>,
    T12: TryIntoPatch<O12>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
    O12,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
            value.7.try_into_value()?,
            value.8.try_into_value()?,
            value.9.try_into_value()?,
            value.10.try_into_value()?,
            value.11.try_into_value()?,
        ))
    }
}
use crate::{FromPatch, IntoPatch};

impl<T: IntoPatch<O>, O> FromPatch<(T,)> for (O,) {
    fn from_value(value: (T,)) -> Self {
        (value.0.into_value(),)
    }
}

impl<T1: IntoPatch<O1>, T2: IntoPatch<O2>, O1, O2> FromPatch<(T1, T2)> for (O1, O2) {
    fn from_value(value: (T1, T2)) -> Self {
        (value.0.into_value(), value.1.into_value())
    }
}

impl<T1: IntoPatch<O1>, T2: IntoPatch<O2>, T3: IntoPatch<O3>, O1, O2, O3> FromPatch<(T1, T2, T3)>
    for (O1, O2, O3)
{
    fn from_value(value: (T1, T2, T3)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
        )
    }
}

impl<T1: IntoPatch<O1>, T2: IntoPatch<O2>, T3: IntoPatch<O3>, T4: IntoPatch<O4>, O1, O2, O3, O4>
    FromPatch<(T1, T2, T3, T4)> for (O1, O2, O3, O4)
{
    fn from_value(value: (T1, T2, T3, T4)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    O1,
    O2,
    O3,
    O4,
    O5,
> FromPatch<(T1, T2, T3, T4, T5)> for (O1, O2, O3, O4, O5)
{
    fn from_value(value: (T1, T2, T3, T4, T5)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
> FromPatch<(T1, T2, T3, T4, T5, T6)> for (O1, O2, O3, O4, O5, O6)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7)> for (O1, O2, O3, O4, O5, O6, O7)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6, T7)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    T8: IntoPatch<O8>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7, T8)> for (O1, O2, O3, O4, O5, O6, O7, O8)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
            value.7.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    T8: IntoPatch<O8>,
    T9: IntoPatch<O9>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9)> for (O1, O2, O3, O4, O5, O6, O7, O8, O9)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
            value.7.into_value(),
            value.8.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    T8: IntoPatch<O8>,
    T9: IntoPatch<O9>,
    T10: IntoPatch<O10>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
            value.7.into_value(),
            value.8.into_value(),
            value.9.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    T8: IntoPatch<O8>,
    T9: IntoPatch<O9>,
    T10: IntoPatch<O10>,
    T11: IntoPatch<O11>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
            value.7.into_value(),
            value.8.into_value(),
            value.9.into_value(),
            value.10.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    T8: IntoPatch<O8>,
    T9: IntoPatch<O9>,
    T10: IntoPatch<O10>,
    T11: IntoPatch<O11>,
    T12: IntoPatch<O12>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
    O12,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
            value.7.into_value(),
            value.8.into_value(),
            value.9.into_value(),
            value.10.into_value(),
            value.11.into_value(),
        )
    }
}

impl<T> FromPatch<()> for [T; 0] {
    fn from_value((): ()) -> Self {
        []
    }
}

impl<T> FromPatch<(T,)> for [T; 1] {
    fn from_value(value: (T,)) -> Self {
        [value.0]
    }
}

impl<T> FromPatch<(T, T)> for [T; 2] {
    fn from_value(value: (T, T)) -> Self {
        [value.0, value.1]
    }
}

impl<T> FromPatch<(T, T, T)> for [T; 3] {
    fn from_value(value: (T, T, T)) -> Self {
        [value.0, value.1, value.2]
    }
}

impl<T> FromPatch<(T, T, T, T)> for [T; 4] {
    fn from_value(value: (T, T, T, T)) -> Self {
        [value.0, value.1, value.2, value.3]
    }
}

impl<T> FromPatch<(T, T, T, T, T)> for [T; 5] {
    fn from_value(value: (T, T, T, T, T)) -> Self {
        [value.0, value.1, value.2, value.3, value.4]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T)> for [T; 6] {
    fn from_value(value: (T, T, T, T, T, T)) -> Self {
        [value.0, value.1, value.2, value.3, value.4, value.5]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T, T)> for [T; 7] {
    fn from_value(value: (T, T, T, T, T, T, T)) -> Self {
        [
            value.0, value.1, value.2, value.3, value.4, value.5, value.6,
        ]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T, T, T)> for [T; 8] {
    fn from_value(value: (T, T, T, T, T, T, T, T)) -> Self {
        [
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7,
        ]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T, T, T, T)> for [T; 9] {
    fn from_value(value: (T, T, T, T, T, T, T, T, T)) -> Self {
        [
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8,
        ]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T, T, T, T, T)> for [T; 10] {
    fn from_value(value: (T, T, T, T, T, T, T, T, T, T)) -> Self {
        [
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8,
            value.9,
        ]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T, T, T, T, T, T)> for [T; 11] {
    fn from_value(value: (T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        [
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8,
            value.9, value.10,
        ]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T, T, T, T, T, T, T)> for [T; 12] {
    fn from_value(value: (T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        [
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8,
            value.9, value.10, value.11,
        ]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T, T, T, T, T, T, T, T)> for [T; 13] {
    fn from_value(value: (T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        [
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8,
            value.9, value.10, value.11, value.12,
        ]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T, T, T, T, T, T, T, T, T)> for [T; 14] {
    fn from_value(value: (T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        [
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8,
            value.9, value.10, value.11, value.12, value.13,
        ]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)> for [T; 15] {
    fn from_value(value: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        [
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8,
            value.9, value.10, value.11, value.12, value.13, value.14,
        ]
    }
}

impl<T> FromPatch<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)> for [T; 16] {
    fn from_value(value: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        [
            value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8,
            value.9, value.10, value.11, value.12, value.13, value.14, value.15,
        ]
    }
}
impl<T> FromPatch<[T; 0]> for () {
    fn from_value(_: [T; 0]) -> Self {}
}
impl<T> FromPatch<[T; 1]> for (T,) {
    fn from_value(value: [T; 1]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 2]> for (T, T) {
    fn from_value(value: [T; 2]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 3]> for (T, T, T) {
    fn from_value(value: [T; 3]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 4]> for (T, T, T, T) {
    fn from_value(value: [T; 4]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 5]> for (T, T, T, T, T) {
    fn from_value(value: [T; 5]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 6]> for (T, T, T, T, T, T) {
    fn from_value(value: [T; 6]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 7]> for (T, T, T, T, T, T, T) {
    fn from_value(value: [T; 7]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 8]> for (T, T, T, T, T, T, T, T) {
    fn from_value(value: [T; 8]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 9]> for (T, T, T, T, T, T, T, T, T) {
    fn from_value(value: [T; 9]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 10]> for (T, T, T, T, T, T, T, T, T, T) {
    fn from_value(value: [T; 10]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 11]> for (T, T, T, T, T, T, T, T, T, T, T) {
    fn from_value(value: [T; 11]) -> Self {
        value.into()
    }
}

impl<T> FromPatch<[T; 12]> for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_value(value: [T; 12]) -> Self {
        value.into()
    }
}
