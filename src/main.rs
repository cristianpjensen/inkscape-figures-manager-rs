pub mod style;
pub mod shortcuts;

use druid::{Application, ClipboardFormat, AppLauncher, WindowDesc, widget::Flex};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use std::{thread, time::Duration};
use tfc::{traits::*, Context, Key};

pub fn main() {
    // set up hotkey shortcuts
    let hotkey_manager = GlobalHotKeyManager::new().expect("hotkey manager should launch");
    let kbd_shortcuts = shortcuts::setup_hotkeys(&hotkey_manager);
    GlobalHotKeyEvent::set_event_handler(Some(move |event| kbd_shortcuts.handler(event)));

    // set up gui such that we can use the druid clipboard, which is necessary
    // for specifying the MIME type
    let main_window = WindowDesc::new(Flex::column())
        .title("Inkscape Figures")
        .window_size((100.0, 100.0));

    // launch gui
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
        .expect("application should launch");
}

fn set_style(style: &style::Style) {
    // put the SVG string with style and proper MIME type (so inkscape
    // recognizes it) on the clipboard and paste style by pressing META+SHIFT+V
    let svg_string = style.to_string();

    let mut clipboard = Application::try_global().expect("gui should be running").clipboard();
    let formats = [ ClipboardFormat::new("image/x-inkscape-svg", svg_string), ];
    clipboard.put_formats(&formats);

    paste_style();
}

fn paste_style() {
    let mut ctx = Context::new().expect("Failed to launch paste context");

    // For OS-specific reasons, it's necessary to wait a moment after
    // creating the context before generating events.
    thread::sleep(Duration::from_millis(10));
    let _ = ctx.key_down(Key::ControlOrMeta);
    let _ = ctx.key_down(Key::Shift);
    let _ = ctx.key_click(Key::V);
    let _ = ctx.key_up(Key::ControlOrMeta);
    let _ = ctx.key_up(Key::Shift);
}
