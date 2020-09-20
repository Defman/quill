use std::path::PathBuf;
use std::fs;
use std::io::{self, Read};

use wasmer_runtime::{
    instantiate,
    DynFunc,
    Value,
    imports,
    error,
};

pub fn load(path: PathBuf) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;

    let mut buf = Vec::new();
    
    file.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn run(wasm: &[u8]) -> error::Result<()> {    
    let import_object = imports! {};

    let instance = instantiate(wasm, &import_object)?;

    let values = instance
        .exports
        .get::<DynFunc>("__quill_init_")?
        .call(&[])?;

    if let Some(ptr) = values.first() {
        println!("{:?}", ptr);
    };

    Ok(())
}