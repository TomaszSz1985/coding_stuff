mod config;
mod error;
mod raid;

use crate::raid::{cmd_fail, cmd_init, cmd_read, cmd_rebuild, cmd_status, cmd_write};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "traid", about = "RAID 5 array simulator")]

struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init {
        #[arg(short, long)]
        disks: u8,

        #[arg(short, long)]
        block_size: u64,

        #[arg(short = 's', long)]
        disk_size: u64,
    },

    Write {
        #[arg(short, long)]
        data: String,
    },

    Read {
        #[arg(short, long)]
        offset: u64,

        #[arg(short, long)]
        len: u64,
    },

    Status,
    Fail {
        #[arg(short, long)]
        disk: u8,
    },
    Rebuild {
        #[arg(short, long)]
        disk: u8,
    },
}

fn main() {
    let cli = Cli::parse();

    let result: Result<(), crate::error::TraidError> = match cli.command {
        Command::Fail { disk } => cmd_fail(disk),
        Command::Init {
            disks,
            block_size,
            disk_size,
        } => cmd_init(disks, block_size, disk_size),
        Command::Read { offset, len } => cmd_read(offset, len),
        Command::Rebuild { disk } => cmd_rebuild(disk),
        Command::Status => cmd_status(),
        Command::Write { data } => cmd_write(data),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
