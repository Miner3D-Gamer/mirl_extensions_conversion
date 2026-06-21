//! This crate houses [`FromPatch`] and [`TryFromPatch`] (and derivatively [`IntoPatch`] and [`TryIntoPatch`])
//!
//! [`core::convert::From`]/[`core::convert::TryFrom`] has a lot of holes when it comes to number conversions.
//! This lib fixes that and adds a lot of other conversions as well. The more the merrier
//!
//!
//! ---
//!
//! It also houses the [`BoundsFitInBounds`] math trait
#![feature(f128)]
#![feature(f16)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)] // specialization has not yet been added by rust.
#![feature(specialization)]
#![feature(const_convert)]
#![feature(const_result_trait_fn)]
#![feature(const_cmp)]

mod conversion;

pub use conversion::*;

mod math;

pub use math::BoundsFitInBounds;

mod crate_support;
