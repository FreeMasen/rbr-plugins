//! rbr-plugin-runner
//! step4
use wasmer_runtime::{
    instantiate,
    imports,
    memory::MemoryView,
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
    let context = instance.context();
    let memory = context.memory(0);
    let len = inject_str("double", &memory.view());
    let double = instance.func::<(u32, u32), u32>("double_")?;
    let start = double.call(5, len)?;
    let two = extract_string(start, &memory.view())?;
    dbg!(two);
    Ok(())
}

fn inject_str(s: &str, mem: &MemoryView<u8>) -> u32 {
    for cell in mem[0..5].iter() {
        cell.set(0)
    }
    let bytes = s.as_bytes();
    for (cell, byte) in mem[5..].iter().zip(bytes.iter()) {
        cell.set(*byte)
    }
    bytes.len() as u32
}

fn extract_string(start: u32, mem: &MemoryView<u8>) -> Res<String> {
    let mut len_bytes = [0;4];
    for (cell, byte) in mem[1..5].iter().zip(len_bytes.iter_mut()) {
        *byte = cell.get()
    }
    let len = u32::from_ne_bytes(len_bytes);
    let end = (start + len) as usize;
    let bytes = mem[start as usize..end]
                .iter()
                .map(|c| c.get())
                .collect();
    let ret = String::from_utf8(bytes)?;
    Ok(ret)
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