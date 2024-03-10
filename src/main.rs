mod clipboard;
mod shortcuts;
mod style;

use clap::{Parser, Subcommand};
use colored::Colorize;
use glob::glob;
use notify::{poll::ScanEvent, Config, PollWatcher, RecursiveMode, Watcher};
use rdev;
use std::{cell::RefCell, path::Path, rc::Rc, time::Duration};

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
    New {
        path: String,
        /// Prints a LaTeX template for the figure
        #[arg(short, long, default_value_t = false)]
        latex: bool,
        /// Lowercases, replaces spaces by dashes, and adds the file extension to the path
        #[arg(short, long, default_value_t = false)]
        format_path: bool,
        /// Sets the directory of the figure. Can be used in conjunction with `format-path` to set the directory and
        /// format the path at the same time. E.g., `ifm new my_figure --format-path --directory figures "..."`
        #[arg(short, long, default_value_t = String::default())]
        directory: String,
    },
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
        Commands::New {
            path,
            latex,
            format_path,
            directory,
        } => {
            let mut path = path.clone();

            if *format_path {
                path = path.to_lowercase();
                path = path.replace(" ", "-");
                path = format!("{path}.svg");
            }

            if *directory != "" {
                path = format!("{directory}/{path}");
            }

            match create_figure(&path) {
                Ok(_) => match open_figure(&path) {
                    Ok(_) => {
                        if *latex {
                            let path = Path::new(&path);
                            let no_ext_path = path.with_extension("");
                            let no_extension = no_ext_path.to_str().unwrap();
                            let file_stem = path.file_stem().unwrap().to_str().unwrap();

                            println!(
                                "{}",
                                vec![
                                    "\\begin{marginfigure}",
                                    "    \\centering",
                                    &format!("    \\incfig{{{}}}", no_extension),
                                    "    \\caption{}",
                                    &format!("    \\label{{fig:{}}}", file_stem),
                                    "\\end{marginfigure}"
                                ]
                                .join("\n")
                            );
                        }
                    }
                    Err(e) => eprintln!("{} {}", "error opening figure:".red(), e),
                },
                Err(e) => eprintln!("{} {}", "error creating figure:".red(), e),
            }
        }
        Commands::Edit { path } => match open_figure(path) {
            Ok(_) => {}
            Err(e) => eprintln!("{} {}", "error opening figure:".red(), e),
        },
    }
}

fn hotkeys_listener() {
    let hotkeys = Rc::new(RefCell::new(shortcuts::HotkeyListener::new()));

    println!("starting hotkeys listener");

    // This will block. We use the grab function to ensure that the hotkeys used with Alt are not passed to Inkscape.
    if let Err(e) = rdev::grab(move |event| hotkeys.borrow_mut().callback(event)) {
        eprintln!("{} {:?}", "error listening to hotkeys:".red(), e);
    }
}

fn autosave_pdf_tex() -> Result<(), notify::Error> {
    // Differentiate between file save events and file scan events
    enum Message {
        Event(notify::Result<notify::Event>),
        Scan(ScanEvent),
    }

    let (tx, rx) = std::sync::mpsc::channel();
    let tx_c = tx.clone();

    // Initialize watcher that checks for file saves by polling every second
    let mut watcher = PollWatcher::with_initial_scan(
        move |event| {
            tx_c.send(Message::Event(event))
                .expect("should be able to send event");
        },
        Config::default().with_poll_interval(Duration::from_secs(1)),
        move |scan_event| {
            tx.send(Message::Scan(scan_event))
                .expect("should be able to send scan event");
        },
    )?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    println!("starting auto-save");

    for res in rx {
        match res {
            // When an SVG is saved, compile it as PDF with TeX
            Message::Event(e) => {
                if let Ok(e) = e {
                    for path in e.paths {
                        if path.extension().is_some_and(|ext| ext == "svg") {
                            let path_stem = path
                                .file_stem()
                                .expect("should be able to get file_stem")
                                .to_string_lossy();
                            let output = std::process::Command::new("inkscape")
                                .arg(path.to_string_lossy().to_string())
                                .arg("--export-area-page")
                                .arg("--export-dpi=300")
                                .arg("--export-type=pdf")
                                .arg("--export-latex")
                                .arg(format!(
                                    "--export-filename={}",
                                    path.canonicalize()
                                        .expect("should be able to canonicalize path")
                                        .to_string_lossy()
                                        .replace(".svg", ".pdf")
                                ))
                                .output();

                            match output {
                                Ok(_) => println!("saved as pdf: {path_stem}"),
                                Err(e) => {
                                    eprintln!("{} {}", "error saving {path_stem} as pdf:".red(), e)
                                }
                            }
                        }
                    }
                }
            }
            // Let the user know when a file has been scanned, indicating that the file is being watched
            Message::Scan(e) => {
                if let Ok(path) = e {
                    if path.extension().is_some_and(|ext| ext == "svg") {
                        let path_stem = path
                            .file_stem()
                            .expect("should be able to get file_stem")
                            .to_string_lossy();
                        println!("found figure: {path_stem}");
                    }
                }
            }
        }
    }

    Ok(())
}

fn list_figures() {
    for path in glob("figures/**/*.svg")
        .expect("should be able to glob `figures/` directory")
        .flatten()
    {
        println!("{}", path.as_os_str().to_string_lossy());
    }
}

fn create_figure(path: &str) -> std::io::Result<u64> {
    // First get the home dir
    let Ok(home_dir) = std::env::var("HOME") else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "$HOME environment variable not set",
        ));
    };

    // Then copy the template to the new figure
    let template_path = Path::new(&home_dir).join(".config/ifm/template.svg");
    std::fs::copy(template_path, Path::new(path))
}

fn open_figure(path: &str) -> std::io::Result<std::process::Child> {
    // Make sure the file exists before attempting to open it with inkscape
    if !file_exists(path) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("`{path}` does not exist"),
        ));
    }

    // Make sure the file is an SVG
    if !std::path::Path::new(path)
        .extension()
        .is_some_and(|ext| ext == "svg")
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("`{path}` is not an SVG file"),
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
