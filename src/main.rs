//! rbr-plugin-runner
//! step2
use wasmer_runtime::{
    instantiate,
    imports,
};

use std::{
    env::args,
    fs::read,
    error::Error,
};
type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    // Plugin Running Code will go here
    let wasm = get_wasm()?;
    let instance = instantiate(&wasm, &imports!{})?;
    let double = instance.func::<u32, u32>("double")?;
    let two = double.call(1)?;
    dbg!(two);
    Ok(())
}

fn get_wasm() -> Res<Vec<u8>> {
    let mut cmd = args();
    // dump currnet app name
    let _ = cmd.next();
    let path = if let Some(p) = cmd.next() {
        p
    } else {
        String::from("target/wasm32-unknown-unknown/debug/rbr_plugin.wasm")
    };
    let ret = read(path)?;
    Ok(ret)
}