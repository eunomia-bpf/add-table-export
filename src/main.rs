use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use clap::Parser;
use core_wasm_ast::ExportDescr;
use wasm_encoder::ExportKind;
use wasm_encoder::ExportSection;
use wasm_encoder::RawSection;
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Add exports for `funcref table` for your wasm program"
)]
struct Args {
    #[arg(help = "The WebAssembly Module file to run")]
    wasm_module_file: String,
    #[arg(short = 'f', long, help = "Whether to override existed table export")]
    r#override: bool,
    #[arg(help = "Name to the export", default_value_t = String::from("__indirect_function_table"))]
    export_name: String,
    #[arg(short, long, help = "Output file name")]
    out_file: String,
}

fn main() -> anyhow::Result<()> {
    // println!("Hello, world!");
    let args = Args::parse();
    let input_data = std::fs::read(&args.wasm_module_file)
        .with_context(|| anyhow!("Failed to read input file"))?;

    let parsed =
        wasm_parser::parse(&input_data).map_err(|e| anyhow!("Failed to parse wasm file: {}", e))?;
    let guard = parsed.sections.lock().unwrap();
    let mut out_module = wasm_encoder::Module::new();
    for sec in guard.iter() {
        match &sec.value {
            core_wasm_ast::Section::Export((_, export)) => {
                let export = export.lock().unwrap();
                let mut out_export_sec = ExportSection::new();
                let mut export_added = false;
                for item in export.iter() {
                    if item.name == args.export_name {
                        if !args.r#override {
                            bail!("Export named `{}` already defined. use `--override` to override the export.",args.export_name);
                        } else {
                            out_export_sec.export(
                                &args.export_name,
                                wasm_encoder::ExportKind::Table,
                                0,
                            );
                            export_added = true;
                        }
                    }
                    let index = match &item.descr {
                        ExportDescr::Func(s)
                        | ExportDescr::Table(s)
                        | ExportDescr::Mem(s)
                        | ExportDescr::Global(s) => s.lock().unwrap().clone(),
                    };
                    out_export_sec.export(
                        &item.name,
                        match &item.descr {
                            ExportDescr::Func(_) => ExportKind::Func,
                            ExportDescr::Table(_) => ExportKind::Table,
                            ExportDescr::Mem(_) => ExportKind::Memory,
                            ExportDescr::Global(_) => ExportKind::Global,
                        },
                        index,
                    );
                }
                if !export_added {
                    out_export_sec.export(&args.export_name, ExportKind::Table, 0);
                }
                out_module.section(&out_export_sec);
            }
            _ => {
                let sec_id = input_data[sec.start_offset];
                let mut input = &input_data[sec.start_offset + 1..];
                let val = leb128::read::unsigned(&mut input)
                    .with_context(|| anyhow!("Invalid integer encountered"))?;
                let size = {
                    let mut buffer = Vec::new();
                    leb128::write::unsigned(&mut buffer, val).unwrap();
                    buffer.len()
                };
                out_module.section(&RawSection {
                    id: sec_id,
                    data: &input_data[sec.start_offset + 1 + size..sec.end_offset],
                });
            }
        }
    }
    std::fs::write(args.out_file, out_module.finish())
        .with_context(|| anyhow!("Failed to write output file"))?;
    return Ok(());
}
