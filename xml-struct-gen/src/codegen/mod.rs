use bimap::BiMap;
use xml::name::OwnedName;

pub mod codegen;
pub mod stubs;
mod stubs_ns;
mod stubs_write;
pub mod util;

pub type AssignedTypeMap = BiMap<Vec<OwnedName>, String>;
