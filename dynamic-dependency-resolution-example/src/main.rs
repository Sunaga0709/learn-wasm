use clap::Parser;
use mynamespace::greet::greetable::Host;
use wasmtime::{
    component::{bindgen, Component, Linker, ResourceTable},
    Engine, Store,
};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path: "/Users/sunaga/wasm/greet/wit",
    world: "hello-world",
});

#[derive(Debug, Parser)]
struct Cli {
    wasm_file: String,
}

fn _main() {
    let cli = Cli::parse();

    if let Err(e) = _start(cli) {
        eprintln!("{e}");
    }
}

fn _start(cli: Cli) -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, Greet::new("Native code".to_string()));

    let component = Component::from_file(&engine, &cli.wasm_file)?;

    HelloWorld::add_to_linker(&mut linker, |greet: &mut Greet| greet)?;

    let _ = wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let hello_world = HelloWorld::instantiate(&mut store, &component, &linker)?;

    let message = hello_world
        .mynamespace_greet_sayable()
        .call_say(&mut store)?;

    println!("{message}");

    Ok(())
}

struct Greet {
    name: String,
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
}

impl Greet {
    fn new(name: String) -> Self {
        let wasi_ctx = WasiCtxBuilder::new().build();
        let resource_table = ResourceTable::new();

        Self {
            name,
            wasi_ctx,
            resource_table,
        }
    }
}

impl Host for Greet {
    fn name(&mut self) -> String {
        self.name.clone()
    }

    fn greet(&mut self, name: String) -> String {
        format!("Hello from {name}")
    }
}

impl WasiView for Greet {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = start(cli).await {
        eprintln!("{e}");
    }
}

async fn start(cli: Cli) -> anyhow::Result<()> {
    let mut config = wasmtime::Config::new();
    config.async_support(true);

    let engine = Engine::default();

    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ViewCtx::new());

    wasmtime_wasi::add_to_linker_async(&mut linker)?;

    let component = Component::from_file(&engine, &cli.wasm_file)?;

    let (command, _) =
        wasmtime_wasi::bindings::Command::instantiate_async(&mut store, &component, &linker)
            .await?;

    let guest = command.wasi_cli_run();

    let _ = guest.call_run(&mut store).await?;

    Ok(())
}
