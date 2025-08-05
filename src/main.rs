use clap::{Arg, Command};
use std::process::exit;

mod compress;
mod context_menu;
mod decompress;
mod progress_bar;

#[tokio::main]
async fn main() {
    let matches = Command::new("rzip")
        .about("Fast file and folder compression tool for Windows")
        .arg(
            Arg::new("encode")
                .short('e')
                .long("encode")
                .value_name("SOURCE")
                .help("Compress file or folder to zip archive")
                .conflicts_with_all(&["decode", "install", "uninstall"]),
        )
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .value_name("ARCHIVE")
                .help("Extract zip archive")
                .conflicts_with_all(&["encode", "install", "uninstall"]),
        )
        .arg(
            Arg::new("install")
                .long("install")
                .help("Install Windows Explorer context menu")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with_all(&["encode", "decode", "uninstall"]),
        )
        .arg(
            Arg::new("uninstall")
                .long("uninstall")
                .help("Uninstall Windows Explorer context menu")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with_all(&["encode", "decode", "install"]),
        )
        .get_matches();

    let result = match (
        matches.get_one::<String>("encode"),
        matches.get_one::<String>("decode"),
        matches.get_flag("install"),
        matches.get_flag("uninstall"),
    ) {
        (Some(source), _, _, _) => compress::compress(source).await.map(|_| "Compression completed"),
        (_, Some(archive), _, _) => decompress::decompress(archive).await.map(|_| "Decompression completed"),
        (_, _, true, _) => context_menu::install_context_menu().map(|_| "Context menu installed successfully"),
        (_, _, _, true) => context_menu::uninstall_context_menu().map(|_| "Context menu uninstalled successfully"),
        _ => {
            eprintln!("Invalid arguments. Use -e <source> or -d <archive> or --install/--uninstall");
            exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Operation failed: {}", e);
        exit(1);
    }
}
