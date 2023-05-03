// Project: WASMERISM
//
// Author: sreejith.naarakathil@gmail.com

use wasmer::{Store, Module, Instance, Value, imports};
use std::fs;

fn main() -> anyhow::Result<()> {
    // Load a wat file in different ways
    let add_one_wat = r#"
    (module
      (type $t0 (func (param i32) (result i32)))
      (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        get_local $p0
        i32.const 1
        i32.add))
    "#;
    let add_wat = fs::read_to_string("./src/add.wat")?;
    let hello_wat = fs::read_to_string("./src/hello.wat")?;

    // Setup store with all the modules loaded
    let mut store = Store::default();
    let add_one_module = Module::new(&store, &add_one_wat)?;
    let add_module = Module::new(&store, &add_wat)?;
    let hello_module = Module::new(&store, &hello_wat)?;

    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let add_one_instance = Instance::new(&mut store, &add_one_module, &import_object)?;
    let add_instance = Instance::new(&mut store, &add_module, &import_object)?;
    let hello_instance = Instance::new(&mut store, &hello_module, &import_object)?;

    // Call functions loaded from the wat files
    let add_one = add_one_instance.exports.get_function("add_one")?;
    let result = add_one.call(&mut store, &[Value::I32(42)])?;
    println!("Result: {:?}", result);

    let add = add_instance.exports.get_function("add")?;
    let result = add.call(&mut store, &[Value::I32(4), Value::I32(5)])?;
    println!("Result: {:?}", result);

    let hello = hello_instance.exports.get_function("hello")?;
    let result = hello.call(&mut store, &[])?;
    println!("Result: {:?}", result);

    Ok(())
}