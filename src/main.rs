pub mod style;

use druid::widget::{prelude::*, Align, Flex};
use druid::widget::{Button, Label};
use druid::{Application, ClipboardFormat, AppLauncher, Data, Lens, WindowDesc};
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};

#[derive(Clone, Data, Lens)]
struct State {
    message: Option<String>,
}

pub fn main() {
    let hotkey_manager = GlobalHotKeyManager::new().expect("Failed to launch hotkey manager");
    let shift_d = HotKey::new(Some(Modifiers::SHIFT), Code::KeyD);
    let _ = hotkey_manager.register_all(&[shift_d]);

    let shortcuts = KeyboardShortcuts { shift_d };

    GlobalHotKeyEvent::set_event_handler(Some(move |event| shortcuts.handler(event) ));

    let main_window = WindowDesc::new(build_root_widget())
        .title("Inkscape Figures")
        .window_size((400.0, 400.0));

    let initial_state: State = State { message: None };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

struct KeyboardShortcuts {
    shift_d: HotKey,
}

impl KeyboardShortcuts {
    fn handler(&self, event: GlobalHotKeyEvent) {
        println!("Event: {:?}", event);

        if event.id == self.shift_d.id() {
            println!("Shift+D");
            let mut style = style::Style::new();
            style.fill_color = Some("white");
            style.fill_opacity = Some(1.0);
            style.stroke_color = Some("black");
            style.stroke_dash = Some(style::StrokeDash::Dotted);
            style.stroke_width = Some(style::StrokeThickness::Normal);
            style.marker_end = true;

            set_style(style);
        }
    }
}

fn build_root_widget() -> impl Widget<State> {
    let label = Label::new(|data: &State, _env: &Env| {
        match &data.message {
            Some(s) => format!("{}", s),
            None => format!("No message"),
        }
    });

    let button = Button::new("Test clipboard")
    .on_click(|_, state: &mut State, _| {
        state.message = Some("Style copied to clipboard".to_string());

        let mut style = style::Style::new();
        style.fill_color = Some("white");
        style.fill_opacity = Some(1.0);
        style.stroke_color = Some("black");
        style.stroke_dash = Some(style::StrokeDash::Solid);
        style.stroke_width = Some(style::StrokeThickness::Normal);
        style.marker_end = true;

        set_style(style);
    });

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(button)
        .with_spacer(20.0)
        .with_child(label);

    // center the two widgets in the available space
    Align::centered(layout)

}

fn set_style(style: style::Style) {
    let svg_string = style.to_string();
    let mut clipboard = Application::global().clipboard();

    let formats = [ ClipboardFormat::new("image/x-inkscape-svg", svg_string), ];
    clipboard.put_formats(&formats);
}
