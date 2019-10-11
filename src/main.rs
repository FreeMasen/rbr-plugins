//! rbr-plugin-runner (bin)
//! step6
use wasmer_runtime::{
    Ctx,
    func,
    instantiate,
    imports,
    memory::MemoryView,
};

use std::{
    env::args,
    fs::read,
    error::Error,
};
mod lib;
use lib::{
    deserialize,
    serialize,
    Thing,
};
type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    // Plugin Running Code will go here
    let wasm = get_wasm()?;
    let instance = instantiate(&wasm, &imports!{
        "env" => {
            "print_str" => func!(print_str),
        },
    })?;
    let context = instance.context();
    let memory = context.memory(0);
    let len = inject_thing(&Thing { stuff: "double".into(), times: 1}, &memory.view())?;
    let double = instance.func::<(u32, u32), u32>("double_")?;
    let start = double.call(5, len)?;
    let view = memory.view();
    let two = extract_thing(start, get_len(&view), &view)?;
    dbg!(two);
    Ok(())
}

fn inject_thing(t: &Thing, mem: &MemoryView<u8>) -> Res<u32> {
    for cell in mem[0..5].iter() {
        cell.set(0)
    }
    let bytes = serialize(t)?;
    for (cell, byte) in mem[5..].iter().zip(bytes.iter()) {
        cell.set(*byte)
    }
    Ok(bytes.len() as u32)
}

fn extract_string(start: u32, len: u32, mem: &MemoryView<u8>) -> Res<String> {
    let bytes = extract_bytes(start, len, mem);
    let ret = String::from_utf8(bytes)?;
    Ok(ret)
}

fn extract_thing(start: u32, len: u32, mem: &MemoryView<u8>) -> Res<Thing> {
    let bytes = extract_bytes(start, len, mem);
    let ret = deserialize(&bytes)?;
    Ok(ret)
}

fn extract_bytes(start: u32, len: u32, mem: &MemoryView<u8>) -> Vec<u8> {
    let end = (start + len) as usize;
    mem[start as usize..end]
                .iter()
                .map(|c| c.get())
                .collect()
}

fn get_len(mem: &MemoryView<u8>) -> u32 {
    let mut len_bytes = [0;4];
    for (cell, byte) in mem[1..5].iter().zip(len_bytes.iter_mut()) {
        *byte = cell.get()
    }
    u32::from_ne_bytes(len_bytes)
}

fn get_wasm() -> Res<Vec<u8>> {
    let mut cmd = args();
    // dump current app name
    let _ = cmd.next();
    let path = if let Some(p) = cmd.next() {
        p
    } else {
        String::from("target/wasm32-unknown-unknown/debug/rbr_plugin.wasm")
    };
    let ret = read(path)?;
    Ok(ret)
}

fn print_str(ctx: &mut Ctx, start: u32, len: u32) -> Res<()> {
    let s = extract_string(start, len, &ctx.memory(0).view())?;
    println!("WASM: {}", s);
    Ok(())
}