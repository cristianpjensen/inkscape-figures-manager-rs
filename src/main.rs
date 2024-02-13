pub mod style;

use druid::widget::{prelude::*, Align, Flex};
use druid::widget::Label;
use druid::{Application, ClipboardFormat, AppLauncher, Data, Lens, WindowDesc};
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, HotKeyState, hotkey::{HotKey, Modifiers, Code}};
// use livesplit_hotkey::{Hook, Hotkey, KeyCode, Modifiers};
use rdev::{simulate, EventType, Key, SimulateError};
use std::{thread, time};

#[derive(Clone, Data, Lens)]
struct State {
    message: Option<String>,
}

pub fn main() {
    // let hotkey_hook = Hook::new().expect("Failed to launch hotkey hook");

    // let _ = hotkey_hook.register(Hotkey { key_code: KeyCode::KeyQ, modifiers: Modifiers::ALT }, move || {
    //     let mut style = style::Style::new();

    //     println!("Stroke: dashed");
    //     style.stroke_dash = Some(style::StrokeDash::Dashed);

    //     set_style(style);

    //     // send(&EventType::KeyRelease(Key::Alt));
    //     // send(&EventType::KeyPress(Key::MetaLeft));
    //     // send(&EventType::KeyPress(Key::ShiftLeft));
    //     // send(&EventType::KeyPress(Key::KeyV));
    //     // send(&EventType::KeyRelease(Key::KeyV));
    //     // send(&EventType::KeyRelease(Key::MetaLeft));
    //     // send(&EventType::KeyRelease(Key::ShiftLeft));
    // });

    // Set up hotkeys
    let hotkey_manager = GlobalHotKeyManager::new().expect("Failed to launch hotkey manager");
    
    let alt_q = HotKey::new(Some(Modifiers::ALT), Code::KeyQ);
    let alt_w = HotKey::new(Some(Modifiers::ALT), Code::KeyW);
    let alt_e = HotKey::new(Some(Modifiers::ALT), Code::KeyE);
    let alt_r = HotKey::new(Some(Modifiers::ALT), Code::KeyR);
    let alt_t = HotKey::new(Some(Modifiers::ALT), Code::KeyT);
    let alt_y = HotKey::new(Some(Modifiers::ALT), Code::KeyY);

    let alt_a = HotKey::new(Some(Modifiers::ALT), Code::KeyA);
    let alt_s = HotKey::new(Some(Modifiers::ALT), Code::KeyS);
    let alt_d = HotKey::new(Some(Modifiers::ALT), Code::KeyD);
    let alt_f = HotKey::new(Some(Modifiers::ALT), Code::KeyF);

    let alt_z = HotKey::new(Some(Modifiers::ALT), Code::KeyZ);
    let alt_x = HotKey::new(Some(Modifiers::ALT), Code::KeyX);
    let alt_space = HotKey::new(Some(Modifiers::ALT), Code::Space);

    let _ = hotkey_manager.register_all(&[alt_q, alt_w, alt_e, alt_r, alt_t, alt_y, alt_a, alt_s, alt_d, alt_f, alt_z, alt_x, alt_space]);
    let shortcuts = KeyboardShortcuts { alt_q, alt_w, alt_e, alt_r, alt_t, alt_y, alt_a, alt_s, alt_d, alt_f, alt_z, alt_x, alt_space };

    GlobalHotKeyEvent::set_event_handler(Some(move |event| shortcuts.handler(event)));

    let main_window = WindowDesc::new(build_root_widget())
        .title("Inkscape Figures")
        .window_size((200.0, 200.0));

    let initial_state: State = State { message: None };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

struct KeyboardShortcuts {
    alt_q: HotKey,
    alt_w: HotKey,
    alt_e: HotKey,
    alt_r: HotKey,
    alt_t: HotKey,
    alt_y: HotKey,
    alt_a: HotKey,
    alt_s: HotKey,
    alt_d: HotKey,
    alt_f: HotKey,
    alt_z: HotKey,
    alt_x: HotKey,
    alt_space: HotKey,
}

impl KeyboardShortcuts {
    fn handler(&self, event: GlobalHotKeyEvent) {
        if event.state != HotKeyState::Released {
            return;
        }

        let mut style = style::Style::new();

        if event.id == self.alt_q.id() {
            println!("Stroke: solid");
            style.stroke_dash = Some(style::StrokeDash::Solid);
        }
        if event.id == self.alt_w.id() {
            println!("Stroke: dashed");
            style.stroke_dash = Some(style::StrokeDash::Dashed);
        }
        if event.id == self.alt_e.id() {
            println!("Stroke: dotted");
            style.stroke_dash = Some(style::StrokeDash::Dotted);
        }

        if event.id == self.alt_r.id() {
            println!("Stroke width: normal");
            style.stroke_width = Some(style::StrokeThickness::Normal);
        }
        if event.id == self.alt_t.id() {
            println!("Stroke width: thick");
            style.stroke_width = Some(style::StrokeThickness::Thick);
        }
        if event.id == self.alt_y.id() {
            println!("Stroke width: very thick");
            style.stroke_width = Some(style::StrokeThickness::VeryThick);
        }

        if event.id == self.alt_a.id() {
            println!("Fill: none");
            style.fill_color = Some("none");
        }
        if event.id == self.alt_s.id() {
            println!("Fill: white");
            style.fill_color = Some("white");
        }
        if event.id == self.alt_d.id() {
            println!("Fill: gray");
            style.fill_color = Some("#E0E0E0");
        }
        if event.id == self.alt_f.id() {
            println!("Fill: black");
            style.fill_color = Some("black");
        }

        if event.id == self.alt_z.id() || event.id == self.alt_space.id() {
            println!("Marker start");
            style.marker_start = Some(true)
        }
        if event.id == self.alt_x.id() || event.id == self.alt_space.id() {
            println!("Marker end");
            style.marker_end = Some(true)
        }

        set_style(&style);
    }
}

fn build_root_widget() -> impl Widget<State> {
    let label = Label::new(|data: &State, _env: &Env| {
        match &data.message {
            Some(s) => format!("{}", s),
            None => format!("No message"),
        }
    });

    let layout = Flex::column().with_child(label);

    Align::centered(layout)
}

fn set_style(style: &style::Style) {
    let svg_string = style.to_string();
    let mut clipboard = Application::global().clipboard();

    let formats = [ ClipboardFormat::new("image/x-inkscape-svg", svg_string), ];
    clipboard.put_formats(&formats);

    paste_style();
}

fn paste_style() {
    send(&EventType::KeyRelease(Key::Alt));
    // send(&EventType::KeyPress(Key::MetaLeft));
    // send(&EventType::KeyPress(Key::ShiftLeft));
    send(&EventType::KeyPress(Key::KeyV));
    // send(&EventType::KeyRelease(Key::MetaLeft));
    // send(&EventType::KeyRelease(Key::ShiftLeft));
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    thread::sleep(delay);
}
