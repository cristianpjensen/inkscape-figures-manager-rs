mod clipboard;
mod shortcuts;
mod style;

use clap::{Parser, Subcommand};
use glob::glob;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use tao::event_loop::{ControlFlow, EventLoopBuilder};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "A tool to manage and quickly create technical figures for LaTeX documents, using Inkscape, on macOS."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Listen for hotkeys to edit, and watches SVG files in the figures/ subdirectory
    /// and auto-saves as .pdf_tex on SVG save
    Start,
    /// Lists all figures for the current document, i.e., all SVGs in the figures/
    /// subdirectory
    List,
    /// Creates a new figure for the current document
    New { file_name: String },
    /// Edits an existing figure for the current document
    Edit { file_name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => {
            hotkeys_listener();
        }
        Commands::List => {
            for entry in glob("figures/*.svg").expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => println!("{:?}", path.display()),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Commands::New { file_name } => {
            println!("New file: {}", file_name);
        }
        Commands::Edit { file_name } => {
            println!("Edit file: {}", file_name);
        }
    }
}

fn hotkeys_listener() {
    // Set up hotkey shortcuts
    let hotkeys_manager = GlobalHotKeyManager::new().expect("hotkey manager should launch");
    let kbd_shortcuts = shortcuts::setup_hotkeys(&hotkeys_manager);
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    let event_loop = EventLoopBuilder::new().build();

    event_loop.run(move |_, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = global_hotkey_channel.try_recv() {
            kbd_shortcuts.handler(event);
        }
    });
}
