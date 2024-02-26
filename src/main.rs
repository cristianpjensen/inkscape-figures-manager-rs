mod clipboard;
mod shortcuts;
mod style;

use clap::{Parser, Subcommand};
use colored::Colorize;
use glob::glob;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use notify::{poll::ScanEvent, Config, PollWatcher, RecursiveMode, Watcher};
use std::{path::Path, time::Duration};
use tao::event_loop::{ControlFlow, EventLoopBuilder};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "Tool to manage and quickly create technical figures for LaTeX documents, using Inkscape, on macOS."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Listen for hotkeys to edit, and watches SVG files in the `figures/` subdirectory and auto-saves as .pdf_tex on
    /// SVG save.
    Start,
    /// Lists all figures for the current document, i.e., all SVGs in the `figures/` subdirectory.
    List,
    /// Creates a new figure for the current document. Give the path to the figure, including the `figures/` subdirectory.
    /// E.g., `ifm new figures/my_figure.svg`. Make sure that `$HOME/.config/ifm/template.svg` exists, since that will be
    /// the template that will be copied to the new file.
    New { path: String },
    /// Edits an existing figure for the current document. Give the path to the figure, including the `figures/`
    /// subdirectory. E.g., `ifm edit figures/my_figure.svg`.
    Edit { path: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => {
            std::thread::spawn(|| {
                if let Err(e) = autosave_pdf_tex() {
                    eprintln!("{} {}", "error watching figures directory:".red(), e);
                };
            });
            hotkeys_listener();
        }
        Commands::List => list_figures(),
        Commands::New { path } => match create_figure(path) {
            Ok(_) => match open_figure(path) {
                Ok(_) => {}
                Err(e) => eprintln!("{} {}", "error opening figure:".red(), e),
            },
            Err(e) => eprintln!("{} {}", "error creating figure:".red(), e),
        },
        Commands::Edit { path } => match open_figure(path) {
            Ok(_) => {}
            Err(e) => eprintln!("{} {}", "error opening figure:".red(), e),
        },
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

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

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
                                Err(e) => eprintln!("{} {}", "error saving {path_stem} as pdf:", e),
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
    for entry in glob("figures/**/*.svg").expect("should be able to glob `figures/` directory") {
        if let Ok(path) = entry {
            println!("{}", path.as_os_str().to_string_lossy());
        }
    }
}

fn create_figure(path: &str) -> std::io::Result<u64> {
    println!("creating figure `{path}`");

    // First get the home dir
    let home_dir = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "$HOME environment variable not set",
            ))
        }
    };

    // Then copy the template to the new figure
    let template_path = Path::new(&home_dir).join(".config/ifm/template.svg");
    std::fs::copy(template_path, Path::new(path))
}

fn open_figure(path: &str) -> std::io::Result<std::process::Child> {
    println!("opening figure `{path}`");

    if !file_exists(path) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("`{path}` does not exist"),
        ));
    }

    std::process::Command::new("inkscape")
        .arg(path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
}

fn file_exists(file_path: &str) -> bool {
    std::fs::metadata(file_path).is_ok()
}
