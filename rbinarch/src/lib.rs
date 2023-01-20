mod binarch;
mod magic;

pub use binarch::Binarch;
pub use magic::{Arch, Endian, Kind, MAX_MAGIC_LEN};
