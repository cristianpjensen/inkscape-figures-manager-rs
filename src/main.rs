mod style;
mod shortcuts;
mod clipboard;

use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

pub fn main() {
    let event_loop = EventLoopBuilder::new().build().unwrap();

    // set up hotkey shortcuts
    let hotkeys_manager = GlobalHotKeyManager::new().expect("hotkey manager should launch");
    let kbd_shortcuts = shortcuts::setup_hotkeys(&hotkeys_manager);

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            if let Ok(event) = global_hotkey_channel.try_recv() {
                if event.state == HotKeyState::Released {
                    return;
                }

                kbd_shortcuts.handler(event);
            }
        })
        .unwrap();
}
