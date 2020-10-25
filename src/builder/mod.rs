//! Various builders to generate/alter wasm components

mod code;
mod data;
mod export;
mod global;
mod import;
mod invoke;
mod memory;
mod misc;
mod module;
mod table;

pub use self::code::{
    function, signature, signatures, FuncBodyBuilder, FunctionBuilder, FunctionDefinition,
    SignatureBuilder, SignaturesBuilder, TypeRefBuilder,
};
pub use self::data::DataSegmentBuilder;
pub use self::export::{export, ExportBuilder, ExportInternalBuilder};
pub use self::global::{global, GlobalBuilder};
pub use self::import::{import, ImportBuilder};
pub use self::invoke::Identity;
pub use self::memory::MemoryBuilder;
pub use self::module::{from_module, module, ModuleBuilder};
pub use self::table::{TableBuilder, TableDefinition, TableEntryDefinition};
