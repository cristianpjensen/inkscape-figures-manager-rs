use crate::{clipboard, style};
use rdev;
use std::{thread, time::Duration};
use tfc::KeyboardContext;

pub struct HotkeyListener<'a> {
    forming_style: bool,
    style: style::Style<'a>,
}

impl HotkeyListener<'_> {
    pub fn new() -> Self {
        Self {
            forming_style: false,
            style: style::Style::new(),
        }
    }

    pub fn callback(&mut self, event: rdev::Event) -> Option<rdev::Event> {
        match event.event_type {
            rdev::EventType::KeyPress(key) => match key {
                rdev::Key::Alt => {
                    println!("== starting style ==");
                    self.forming_style = true;
                }
                rdev::Key::Num1
                | rdev::Key::Num2
                | rdev::Key::Num3
                | rdev::Key::KeyQ
                | rdev::Key::KeyW
                | rdev::Key::KeyE
                | rdev::Key::KeyA
                | rdev::Key::KeyS
                | rdev::Key::KeyD
                | rdev::Key::KeyZ
                | rdev::Key::KeyX => {
                    if self.forming_style {
                        update_style(&mut self.style, key);
                    }
                }
                _ => {}
            },
            rdev::EventType::KeyRelease(key) => {
                if key == rdev::Key::Alt && self.forming_style {
                    println!("== applied style ==");
                    apply_style(&mut self.style);

                    self.forming_style = false;
                    self.style = style::Style::new();
                }
            }
            _ => {}
        };

        if self.forming_style {
            None
        } else {
            Some(event)
        }
    }
}

pub fn update_style(style: &mut style::Style, key: rdev::Key) {
    match key {
        rdev::Key::Num1 => {
            println!("stroke width:\tnormal");
            style.stroke_width = style::StrokeThickness::Normal;
        }
        rdev::Key::Num2 => {
            println!("stroke width:\tthick");
            style.stroke_width = style::StrokeThickness::Thick;
        }
        rdev::Key::Num3 => {
            println!("stroke width:\tvery thick");
            style.stroke_width = style::StrokeThickness::VeryThick;
        }
        rdev::Key::KeyQ => {
            println!("stroke:\t\tsolid");
            style.stroke_dash = style::StrokeDash::Solid;
        }
        rdev::Key::KeyW => {
            println!("stroke:\t\tdashed");
            style.stroke_dash = style::StrokeDash::Dashed;
        }
        rdev::Key::KeyE => {
            println!("stroke:\t\tdotted");
            style.stroke_dash = style::StrokeDash::Dotted;
        }
        rdev::Key::KeyA => {
            println!("fill:\t\twhite");
            style.fill_color = "white";
            style.fill_opacity = 1.0;
        }
        rdev::Key::KeyS => {
            println!("fill:\t\tgrey");
            style.fill_color = "black";
            style.fill_opacity = 0.12;
        }
        rdev::Key::KeyD => {
            println!("fill:\t\tblack");
            style.fill_color = "black";
            style.fill_opacity = 1.0;
        }
        rdev::Key::KeyZ => {
            println!("marker:\t\tstart");
            style.marker_start = true;
        }
        rdev::Key::KeyX => {
            println!("marker:\t\tend");
            style.marker_end = true;
        }
        _ => {}
    }
}

fn apply_style(style: &mut style::Style) {
    // put the SVG string with style and proper MIME type (so inkscape
    // recognizes it) on the clipboard and paste style by pressing META+SHIFT+V
    let svg_string = style.to_string();
    clipboard::copy_mime("image/x-inkscape-svg", &svg_string);
    paste_style();
}

fn paste_style() {
    let mut ctx = tfc::Context::new().expect("paste context should launch");

    // For OS-specific reasons, it's necessary to wait a moment after
    // creating the context before generating events.
    thread::sleep(Duration::from_millis(10));
    let _ = ctx.key_down(tfc::Key::ControlOrMeta);
    let _ = ctx.key_down(tfc::Key::Shift);
    let _ = ctx.key_click(tfc::Key::V);
    let _ = ctx.key_up(tfc::Key::ControlOrMeta);
    let _ = ctx.key_up(tfc::Key::Shift);
}
