// Copyright (c) Microsoft. All rights reserved.

#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::use_self)]

pub mod error;
pub mod provisioning;

pub use crate::error::Error;
pub use crate::provisioning::{
    BackupProvisioning, DpsSymmetricKeyProvisioning, DpsTpmProvisioning, DpsX509Provisioning,
    Provision, ProvisioningResult, ReprovisioningStatus,
};
