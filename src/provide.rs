//! Internal helpers for error derives.
//!
//! This module is not intended for public consumption. The trait is
//! documented as hidden and mirrors the shim provided by `thiserror` to allow
//! derived errors to forward `core::error::Request` values to their sources.

use core::error::{Error, Request};

#[doc(hidden)]
pub trait ThiserrorProvide: Sealed {
    fn thiserror_provide<'a>(&'a self, request: &mut Request<'a>);
}

impl<T> ThiserrorProvide for T
where
    T: Error + ?Sized
{
    #[inline]
    fn thiserror_provide<'a>(&'a self, request: &mut Request<'a>) {
        self.provide(request);
    }
}

#[doc(hidden)]
pub trait Sealed {}

impl<T> Sealed for T where T: Error + ?Sized {}
