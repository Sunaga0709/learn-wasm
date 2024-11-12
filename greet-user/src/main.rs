use clap::Parser;

use wasmtime::component::{Component, Instance, Linker, TypedFunc};
use wasmtime::{Engine, Store};

mod greetable {
    use wasmtime::component::bindgen;

    bindgen!({
        world: "greetable-provider",
        path: "/Users/sunaga/wasm/greet/wit/",
    });
}

mod sayable {
    use wasmtime::component::bindgen;

    bindgen!({
        world: "hello-world",
        path: "/Users/sunaga/wasm/greet/wit/",
    });
}

#[derive(Debug, Parser)]
struct Args {
    wasm_file: String,
}

fn main() {
    let args = Args::parse();

    // Cargo Component
    // if let Err(e) = start(args) {
    //     eprintln!("error: {e:?}");
    // }

    // bindgen macro
    // if let Err(e) = start_macro(args) {
    //     eprintln!("error: {e:?}");
    // }

    // sayable
    if let Err(e) = start_hello_world(args) {
        eprintln!("error: {e:?}");
    }
}

#[allow(unused)]
fn start_hello_world(args: Args) -> anyhow::Result<()> {
    let engine = Engine::default();
    let linker = Linker::new(&engine);
    let component = Component::from_file(&engine, &args.wasm_file)?;
    let mut store = Store::new(&engine, ());

    let says = sayable::HelloWorld::instantiate(&mut store, &component, &linker)?;
    let say = says.mynamespace_greet_sayable().call_say(&mut store)?;

    print!("{}", say);

    Ok(())
}

#[allow(unused)]
fn start_macro(args: Args) -> anyhow::Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, &args.wasm_file)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let provider = greetable::GreetableProvider::instantiate(&mut store, &component, &linker)?;
    let greetable = provider.mynamespace_greet_greetable();

    let message = greetable.call_greet(&mut store, "world")?;
    dbg!(message);

    let name = greetable.call_name(&mut store)?;
    dbg!(name.clone());

    let message = greetable.call_greet(&mut store, &name)?;
    dbg!(message);

    Ok(())
}

#[allow(unused)]
fn start_cargo_component(args: Args) -> anyhow::Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, &args.wasm_file)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let instance = linker.instantiate(&mut store, &component)?;

    let greetable_index = instance
        .get_export(&mut store, None, "mynamespace:greet/greetable")
        .unwrap();

    let name_index = instance
        .get_export(&mut store, Some(&greetable_index), "name")
        .unwrap();
    let greet_index = instance
        .get_export(&mut store, Some(&greetable_index), "greet")
        .unwrap();

    let name: TypedFunc<(), (String,)> = instance.get_typed_func(&mut store, name_index).unwrap();
    let greet: TypedFunc<(String,), (String,)> =
        instance.get_typed_func(&mut store, greet_index).unwrap();

    let argument = "World".to_string();

    let (return_value,) = greet.call(&mut store, (argument,))?;
    greet.post_return(&mut store)?;
    println!("{return_value}");

    let (returned_name,) = name.call(&mut store, ())?;
    name.post_return(&mut store)?;

    let (return_value,) = greet.call(&mut store, (returned_name,))?;
    greet.post_return(&mut store)?;
    println!("{return_value}");

    Ok(())
}

// #[allow(unused)]
// fn get_greetable_function(
//     instance: &Instance,
//     store: &mut Store<()>,
// ) -> (TypedFunc<(), (String,)>, TypedFunc<(String,), (String,)>) {
//     let mut exports = instance.exports(store);
//
//     let mut greetable = exports.instance("mynamespace:greet/reetable").unwrap();
//
//     (greetable.name, greetable.greet)
// }
