// crates/dryad_runtime/src/lib.rs
pub mod interpreter;
pub mod native_functions;

pub use interpreter::{Interpreter, Value};
pub use native_functions::{NativeFunctionRegistry, NativeModule};
