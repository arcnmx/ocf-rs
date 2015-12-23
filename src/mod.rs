#![cfg_attr(feature = "unstable", plugin(serde_macros))]
#![cfg_attr(feature = "unstable", feature(plugin, custom_derive, custom_attribute))]

#[cfg(feature = "unstable")]
include!("lib.rs");

#[cfg(feature = "syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
