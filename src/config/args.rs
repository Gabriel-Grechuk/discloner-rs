use clap::Parser;

/// Simple program for clonning files or disks.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Input file.
    #[arg(short, long)]
    input: String,

    /// Output file.
    #[arg(short, long)]
    output: Vec<String>,

    /// Size of the blocks that will be copied.
    #[arg(short, long, default_value_t = 512)]
    block_size: u32,

    /// Count of blocks that should be copied.
    #[arg(short, long)]
    count: Option<u64>,

    /// Amount of data that should be copied in [B, KB, MB, GB]. Ex: "100KB"
    #[arg(short, long)]
    size: Option<String>,

    /// Resume last failed of aborted operation.
    #[arg(short, long, default_value_t = false)]
    resume: bool,
}
