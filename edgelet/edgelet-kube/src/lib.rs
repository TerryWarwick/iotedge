// Copyright (c) Microsoft. All rights reserved.

#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::use_self
)]

mod constants;
mod convert;
mod error;
mod module;
mod runtime;

pub use self::module::KubeModule;
pub use self::runtime::KubeModuleRuntime;
