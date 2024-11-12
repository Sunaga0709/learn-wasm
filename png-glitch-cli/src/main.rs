use clap::Parser;
use png_glitch::{FilterType, PngGlitch};

use std::io::{Read, Write};

mod bindings;
use bindings::sunaga0709::glitch_art::png_glitchable::{
    glitch, FilterType as WasmFilterType, ScanLine as WasmScanLine,
};

impl From<WasmFilterType> for FilterType {
    fn from(value: WasmFilterType) -> Self {
        match value {
            WasmFilterType::Up => Self::Up,
            WasmFilterType::Sub => Self::Sub,
            WasmFilterType::None => Self::None,
            WasmFilterType::Paeth => Self::Paeth,
            WasmFilterType::Average => Self::Average,
        }
    }
}

impl From<FilterType> for WasmFilterType {
    fn from(value: FilterType) -> Self {
        match value {
            FilterType::Up => Self::Up,
            FilterType::Sub => Self::Sub,
            FilterType::None => Self::None,
            FilterType::Paeth => Self::Paeth,
            FilterType::Average => Self::Average,
        }
    }
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, default_value = "glitched.png")]
    output_file: String,
    input_file: String,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = start(&cli) {
        eprintln!("{e}");
    }
}

fn start(cli: &Cli) -> anyhow::Result<()> {
    let mut glitch = PngGlitch::open(&cli.input_file)?;

    run(&mut glitch);

    glitch.save(&cli.output_file)?;

    Ok(())
}

fn run(png_glitch: &mut PngGlitch) {
    png_glitch.foreach_scanline(|scanline| {
        let mut pixel_data = vec![];

        if let Ok(_) = scanline.read_to_end(&mut pixel_data) {
            let filter_type = scanline.filter_type().into();
            let wasm_scan_line = WasmScanLine {
                filter_type,
                pixel_data,
            };

            let returned_scan_line = glitch(&wasm_scan_line);

            scanline.set_filter_type(returned_scan_line.filter_type.into());
            let _ = scanline.write(&returned_scan_line.pixel_data);
        }
    });
}
