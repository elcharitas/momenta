#![no_std]
//! Momenta Core - Core types and utilities for the Momenta framework
//!
//! This crate provides the foundational types used throughout the Momenta ecosystem:
//! - Node types (Element, Text, Fragment, etc.)
//! - Component trait and utilities
//! - Reactive signals system
//!
//! This is typically not used directly; use the `momenta` crate instead.

extern crate alloc;

pub mod nodes;
pub mod signals;

pub use paste::paste;

pub mod prelude {
    pub use crate::nodes::{Component, HtmlWriter, Node, classes};
    #[cfg(any(feature = "computed", feature = "full-reactivity"))]
    pub use crate::signals::create_computed;
    #[cfg(any(feature = "memoization", feature = "full-reactivity"))]
    pub use crate::signals::create_memo;
    pub use crate::signals::{
        Signal, SignalValue, batch, create_effect, create_effect_with_cleanup, create_signal,
    };
    pub use momenta_macros::{component, rsx, when};
}
