#![feature(iter_chain)]
#![feature(error_generic_member_access)]
#![feature(anonymous_lifetime_in_impl_trait)]

use num_format::Locale;

pub mod colors;
pub mod console_widget;
pub mod enclosure;
pub mod err;
pub mod lsblk;
pub mod utils;
pub mod zfs;

pub const LOCALE: &Locale = &Locale::en;
