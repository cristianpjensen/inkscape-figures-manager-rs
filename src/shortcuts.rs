use global_hotkey::hotkey::{Code, Modifiers};
use global_hotkey::GlobalHotKeyManager;
use global_hotkey::{GlobalHotKeyEvent, HotKeyState, hotkey::HotKey};
use crate::style;
use crate::set_style;

pub fn setup_hotkeys(hotkey_manager: &GlobalHotKeyManager) -> KeyboardShortcuts {
    let alt_1 = HotKey::new(Some(Modifiers::ALT), Code::Digit1);
    let alt_2 = HotKey::new(Some(Modifiers::ALT), Code::Digit2);
    let alt_3 = HotKey::new(Some(Modifiers::ALT), Code::Digit3);

    let alt_q = HotKey::new(Some(Modifiers::ALT), Code::KeyQ);
    let alt_w = HotKey::new(Some(Modifiers::ALT), Code::KeyW);
    let alt_e = HotKey::new(Some(Modifiers::ALT), Code::KeyE);

    let alt_a = HotKey::new(Some(Modifiers::ALT), Code::KeyA);
    let alt_s = HotKey::new(Some(Modifiers::ALT), Code::KeyS);
    let alt_d = HotKey::new(Some(Modifiers::ALT), Code::KeyD);
    let alt_f = HotKey::new(Some(Modifiers::ALT), Code::KeyF);

    let alt_z = HotKey::new(Some(Modifiers::ALT), Code::KeyZ);
    let alt_x = HotKey::new(Some(Modifiers::ALT), Code::KeyX);
    let alt_space = HotKey::new(Some(Modifiers::ALT), Code::Space);

    // Register hotkeys
    let _ = hotkey_manager.register_all(&[alt_1, alt_2, alt_3, alt_q, alt_w, alt_e, alt_a, alt_s, alt_d, alt_f, alt_z, alt_x, alt_space]);

    KeyboardShortcuts { alt_1, alt_2, alt_3, alt_q, alt_w, alt_e, alt_a, alt_s, alt_d, alt_f, alt_z, alt_x, alt_space }
}

pub struct KeyboardShortcuts {
    pub alt_1: HotKey,
    pub alt_2: HotKey,
    pub alt_3: HotKey,
    pub alt_q: HotKey,
    pub alt_w: HotKey,
    pub alt_e: HotKey,
    pub alt_a: HotKey,
    pub alt_s: HotKey,
    pub alt_d: HotKey,
    pub alt_f: HotKey,
    pub alt_z: HotKey,
    pub alt_x: HotKey,
    pub alt_space: HotKey,
}

impl KeyboardShortcuts {
    pub fn handler(&self, event: GlobalHotKeyEvent) {
        if event.state != HotKeyState::Released {
            return;
        }

        let mut style = style::Style::new();

        if event.id == self.alt_1.id() {
            println!("stroke width:\tnormal");
            style.stroke_width = Some(style::StrokeThickness::Normal);
        }
        if event.id == self.alt_2.id() {
            println!("stroke width:\tthick");
            style.stroke_width = Some(style::StrokeThickness::Thick);
        }
        if event.id == self.alt_3.id() {
            println!("stroke width:\tvery thick");
            style.stroke_width = Some(style::StrokeThickness::VeryThick);
        }

        if event.id == self.alt_q.id() {
            println!("stroke:\t\tsolid");
            style.stroke_dash = Some(style::StrokeDash::Solid);
        }
        if event.id == self.alt_w.id() {
            println!("stroke:\t\tdashed");
            style.stroke_dash = Some(style::StrokeDash::Dashed);
        }
        if event.id == self.alt_e.id() {
            println!("stroke:\t\tdotted");
            style.stroke_dash = Some(style::StrokeDash::Dotted);
        }

        if event.id == self.alt_a.id() {
            println!("fill:\t\tnone");
            style.fill_color = Some("none");
        }
        if event.id == self.alt_s.id() {
            println!("fill:\t\twhite");
            style.fill_color = Some("white");
        }
        if event.id == self.alt_d.id() {
            println!("fill:\t\tgray");
            style.fill_color = Some("#E0E0E0");
        }
        if event.id == self.alt_f.id() {
            println!("fill:\t\tblack");
            style.fill_color = Some("black");
        }

        if event.id == self.alt_z.id() {
            println!("marker:\t\tstart");
            style.marker_start = Some(true)
        }
        if event.id == self.alt_x.id() {
            println!("marker:\t\tend");
            style.marker_end = Some(true)
        }

        if event.id == self.alt_space.id() {
            println!("marker:\t\tnone");
            style.marker_start = Some(false);
            style.marker_end = Some(false);
        }

        set_style(&style);
    }
}
