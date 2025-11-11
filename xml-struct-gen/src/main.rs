use crate::codegen::codegen::generate_code;
use crate::struct_scan::StructScanner;
use anyhow::bail;
use clap::Parser;
use std::path::PathBuf;

mod codegen;
mod common;
mod postprocess;
mod struct_scan;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input XML file path(s)
    #[arg(short, long)]
    input: Vec<PathBuf>,

    /// Output Rust file path
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut scanner = StructScanner::new();
    if args.input.is_empty() {
        bail!("At least one input file is required");
    } else {
        for input in args.input.iter() {
            scanner.scan_structs(input)?;
        }
    }

    let mut structs = scanner.get();

    postprocess::validate_text_invariant(&structs)?;

    postprocess::mark_root(&mut structs)?;

    let ns = scanner.namespaces;
    let generated_code = generate_code(ns, structs)?;

    std::fs::write(&args.output, generated_code.as_bytes()).map_err(|e| {
        anyhow::anyhow!(
            "Failed to write output file {}: {}",
            args.output.display(),
            e
        )
    })?;

    Ok(())
}
