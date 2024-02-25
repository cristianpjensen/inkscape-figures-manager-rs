mod clipboard;
mod shortcuts;
mod style;

use std::{path::Path, time::Duration};

use clap::{Parser, Subcommand};
use glob::glob;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use notify::{poll::ScanEvent, Config, PollWatcher, RecursiveMode, Watcher};
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
    New { name: String },
    /// Edits an existing figure for the current document
    Edit { name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => {
            std::thread::spawn(autosave_pdf_tex);
            hotkeys_listener();
        }
        Commands::List => list_figures(),
        Commands::New { name } => {
            create_figure(name);
            open_figure(name);
        }
        Commands::Edit { name } => open_figure(name),
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

fn autosave_pdf_tex() -> Result<(), notify::Error> {
    let (tx, rx) = std::sync::mpsc::channel();
    let tx_c = tx.clone();

    enum Message {
        Event(notify::Result<notify::Event>),
        Scan(ScanEvent),
    }

    let mut watcher = PollWatcher::with_initial_scan(
        move |event| {
            tx_c.send(Message::Event(event)).unwrap();
        },
        Config::default().with_poll_interval(Duration::from_secs(1)),
        move |scan_event| {
            tx.send(Message::Scan(scan_event)).unwrap();
        },
    )?;

    watcher.watch(Path::new("figures"), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Message::Event(e) => {
                if let Ok(e) = e {
                    for path in e.paths {
                        if path.extension().is_some_and(|ext| ext == "svg") {
                            let path_stem = path.file_stem().unwrap().to_string_lossy();
                            let output = std::process::Command::new("inkscape")
                                .arg("--export-area-page")
                                .arg("--export-dpi=300")
                                .arg("--export-type=pdf")
                                .arg("--export-latex")
                                .arg(format!(
                                    "--export-filename={}",
                                    path.canonicalize()
                                        .unwrap()
                                        .to_string_lossy()
                                        .replace(".svg", ".pdf")
                                ))
                                .output();

                            match output {
                                Ok(_) => println!("saved as pdf: {path_stem}"),
                                Err(e) => println!("ERROR: saving {path_stem} as pdf: {e:#?}"),
                            }
                        }
                    }
                }
            }
            Message::Scan(e) => {
                if let Ok(path) = e {
                    if path.extension().is_some_and(|ext| ext == "svg") {
                        let path_stem = path.file_stem().unwrap().to_string_lossy();
                        println!("found figure: {path_stem}");
                    }
                }
            }
        }
    }

    Ok(())
}

fn list_figures() {
    for entry in glob("figures/*.svg").expect("should be able to glob figures/ directory") {
        if let Ok(path) = entry {
            if let Some(filename) = path.file_name() {
                println!("{}", filename.to_string_lossy());
            }
        }
    }
}

fn create_figure(filename: &str) {
    println!("Create figure: {}", filename);
}

fn open_figure(filename: &str) {
    println!("Open figure: {}", filename);
}
