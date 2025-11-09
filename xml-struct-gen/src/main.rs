use crate::codegen::codegen::generate_code;
use crate::postprocess::mark_root;
use crate::struct_scan::StructScanner;
use clap::Parser;
use std::path::PathBuf;

mod codegen;
mod postprocess;
mod struct_scan;
mod common;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input XML file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output Rust file path
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut scanner = StructScanner::new();
    let mut structs = scanner.scan_structs(&args.input)?;

    mark_root(&mut structs)?;

    let generated_code = generate_code(structs);

    std::fs::write(&args.output, generated_code.as_bytes()).map_err(|e| {
        anyhow::anyhow!(
            "Failed to write output file {}: {}",
            args.output.display(),
            e
        )
    })?;

    Ok(())
}
