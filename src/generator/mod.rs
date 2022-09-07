mod r#async;
mod sync;

pub use r#async::{
    SerialU32Generator as SerialU32GeneratorAsync, SerialU64Generator as SerialU64GeneratorAsync,
    SerialUsizeGenerator as SerialUsizeGeneratorAsync,
};

pub use sync::{
    SerialU32Generator as SerialU32GeneratorSync, SerialU64Generator as SerialU64GeneratorSync,
    SerialUsizeGenerator as SerialUsizeGeneratorSync,
};
