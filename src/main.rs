// Project: WASMERISM
//
// Author: sreejith.naarakathil@gmail.com

use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    // An engine stores and configures global compilation settings like
    // optimization level, enabled wasm features, etc.
    let engine = Engine::default();

    // We start off by creating a `Module` which represents a compiled form
    // of our input wasm module. In this case it'll be JIT-compiled after
    // we parse the text format.
    let module = Module::from_file(&engine, "./src/hello.wat")?;
    let module2 = Module::from_file(&engine, "./src/add.wat")?;

    // A `Store` is what will own instances, functions, globals, etc. All wasm
    // items are stored within a `Store`, and it's what we'll always be using to
    // interact with the wasm world. Custom data can be stored in stores but for
    // now we just use `()`.
    let mut store = Store::new(&engine, ());

    // With a compiled `Module` we can then instantiate it, creating
    // an `Instance` which we can actually poke at functions on.
    let instance = Instance::new(&mut store, &module, &[])?;
    let instance2 = Instance::new(&mut store, &module2, &[])?;

    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our `answer` exported function and
    // run it.
    let answer = instance.get_func(&mut store, "answer")
        .expect("`answer` was not an exported function");
    let add = instance2.get_func(&mut store, "add")
        .expect("`add` was not an exported function");
    // There's a few ways we can call the `answer` `Func` value. The easiest
    // is to statically assert its signature with `typed` (in this case
    // asserting it takes no arguments and returns one i32) and then call it.
    let answer = answer.typed::<(), i32>(&store)?;
    let add = add.typed::<(i32, i32), i32>(&store)?;

    // And finally we can call our function! Note that the error propagation
    // with `?` is done to handle the case where the wasm function traps.
    let result = answer.call(&mut store, ())?;
    println!("Answer: {:?}", result);
    let result = add.call(&mut store, (4, 5))?;
    println!("Result: {:?}", result);    
    Ok(())
}