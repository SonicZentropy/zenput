use crate::{common::*, linux::inputs::*, public::*};
use input::{
    event::{
        keyboard::{
            KeyState, {KeyboardEvent, KeyboardEventTrait},
        },
        pointer::{ButtonState, PointerEvent::*},
        Event::{self, *},
    },
    Libinput, LibinputInterface,
};
use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::close,
};
use std::{
    os::unix::io::RawFd,
    path::Path, thread::sleep, time::Duration, ptr::null, mem::MaybeUninit,
};
use uinput::event::{controller::{Controller, Mouse}, Event as UinputEvent, relative::Position};
use x11::xlib::*;
use once_cell::sync::Lazy;

mod inputs;

type ButtonStatesMap = HashMap<MouseButton, bool>;
type KeyStatesMap = HashMap<KeybdKey, bool>;

static BUTTON_STATES: Lazy<Mutex<ButtonStatesMap>> = Lazy::new(|| Mutex::new(ButtonStatesMap::new()));
static KEY_STATES: Lazy<Mutex<KeyStatesMap>> = Lazy::new(|| Mutex::new(KeyStatesMap::new()));
static SEND_DISPLAY: Lazy<AtomicPtr<Display>> = Lazy::new(|| {
    unsafe { XInitThreads() };
    AtomicPtr::new(unsafe { XOpenDisplay(null()) })
});
static FAKE_DEVICE: Lazy<Mutex<uinput::Device>> = Lazy::new(|| {
    Mutex::new(
        uinput::default()
            .unwrap()
            .name("inputbot")
            .unwrap()
            .event(uinput::event::Keyboard::All)
            .unwrap()
            .event(UinputEvent::Controller(Controller::Mouse(Mouse::Left)))
            .unwrap()
            .event(UinputEvent::Controller(Controller::Mouse(Mouse::Right)))
            .unwrap()
            .event(UinputEvent::Controller(Controller::Mouse(Mouse::Middle)))
            .unwrap()
            .event(UinputEvent::Controller(Controller::Mouse(Mouse::Side)))
            .unwrap()
            .event(UinputEvent::Controller(Controller::Mouse(Mouse::Extra)))
            .unwrap()
            .event(UinputEvent::Controller(Controller::Mouse(Mouse::Forward)))
            .unwrap()
            .event(UinputEvent::Controller(Controller::Mouse(Mouse::Back)))
            .unwrap()
            .event(UinputEvent::Controller(Controller::Mouse(Mouse::Task)))
            .unwrap()
            .event(Position::X)
            .unwrap()
            .event(Position::Y)
            .unwrap()
            .create()
            .unwrap(),
    )
});


/// Requests the fake device to be generated.
/// 
/// Can be called before using the fake device to prevent it from 
/// building when you first try to use it.
pub fn init_device() {
    FAKE_DEVICE.lock().unwrap();
}


impl KeybdKey {
    pub fn is_pressed(self) -> bool {
        *KEY_STATES.lock().unwrap().entry(self).or_insert(false)
    }

    pub fn press(self) {
        let mut device = FAKE_DEVICE.lock().unwrap();

        device
            .write(0x01, key_to_scan_code(self), 1)
            .unwrap();
        device.synchronize().unwrap();
    }

    pub fn release(self) {
        let mut device = FAKE_DEVICE.lock().unwrap();

        device
            .write(0x01, key_to_scan_code(self), 0)
            .unwrap();
        device.synchronize().unwrap();
    }

    pub fn is_toggled(self) -> bool {
        if let Some(key) = match self {
            KeybdKey::ScrollLockKey => Some(4),
            KeybdKey::NumLockKey => Some(2),
            KeybdKey::CapsLockKey => Some(1),
            _ => None,
        } {
            let mut state: XKeyboardState = unsafe { MaybeUninit::zeroed().assume_init() };
            SEND_DISPLAY.with(|display| unsafe {
                XGetKeyboardControl(display, &mut state);
            });
            state.led_mask & key != 0
        } else {
            false
        }
    }
}

impl MouseButton {
    pub fn is_pressed(self) -> bool {
        *BUTTON_STATES.lock().unwrap().entry(self).or_insert(false)
    }

    pub fn press(self) {
        let mut device = FAKE_DEVICE.lock().unwrap();
        device.press(&Controller::Mouse(Mouse::from(self))).unwrap();
        device.synchronize().unwrap();
    }

    pub fn release(self) {
        let mut device = FAKE_DEVICE.lock().unwrap();
        device.release(&Controller::Mouse(Mouse::from(self))).unwrap();
        device.synchronize().unwrap();
    }
}

impl MouseCursor {
    pub fn move_rel(x: i32, y: i32) {
        SEND_DISPLAY.with(|display| unsafe {
            XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
        });
    }

    pub fn move_abs(x: i32, y: i32) {
        SEND_DISPLAY.with(|display| unsafe {
            XWarpPointer(display, 0, XRootWindow(display, XDefaultScreen(display)), 0, 0, 0, 0, x, y);
        });
    }
}

impl MouseWheel {
    pub fn scroll_ver(y: i32) {
        if y < 0 {
          MouseButton::OtherButton(4).press();
          MouseButton::OtherButton(4).release();
        } else {
          MouseButton::OtherButton(5).press();
          MouseButton::OtherButton(5).release();
        }
    }
    pub fn scroll_hor(x: i32) {
        if x < 0 {
          MouseButton::OtherButton(6).press();
          MouseButton::OtherButton(6).release();
        } else {
          MouseButton::OtherButton(7).press();
          MouseButton::OtherButton(7).release();
        }
    }
}

struct LibinputInterfaceRaw;

impl LibinputInterfaceRaw {
    fn seat(&self) -> String {
        String::from("seat0")
    }
}

impl LibinputInterface for LibinputInterfaceRaw {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> std::result::Result<RawFd, i32> {
        if let Ok(fd) = open(path, OFlag::from_bits_truncate(flags), Mode::empty()) {
            Ok(fd)
        } else {
            Err(1)
        }
    }

    fn close_restricted(&mut self, fd: RawFd) {
        let _ = close(fd);
    }
}

pub fn handle_input_events() {
    let mut libinput_context = Libinput::new_with_udev(LibinputInterfaceRaw);
    libinput_context
        .udev_assign_seat(&LibinputInterfaceRaw.seat())
        .unwrap();
    while !MOUSE_BINDS.lock().unwrap().is_empty() || !KEYBD_BINDS.lock().unwrap().is_empty() {
        libinput_context.dispatch().unwrap();
        while let Some(event) = libinput_context.next() {
            handle_input_event(event);
        }
        sleep(Duration::from_millis(10));
    }
}

fn handle_input_event(event: Event) {
    match event {
        Keyboard(keyboard_event) => {
            let KeyboardEvent::Key(keyboard_key_event) = keyboard_event;
            let key = keyboard_key_event.key();
            if let Some(keybd_key) = scan_code_to_key(key) {
                if keyboard_key_event.key_state() == KeyState::Pressed {
                    KEY_STATES.lock().unwrap().insert(keybd_key, true);
                    if let Some(Bind::NormalBind(cb)) = KEYBD_BINDS.lock().unwrap().get(&keybd_key) {
                        let cb = Arc::clone(cb);
                        spawn(move || cb());
                    };
                } else {
                    KEY_STATES.lock().unwrap().insert(keybd_key, false);
                }
            }
        }
        Pointer(pointer_event) => {
            if let Button(button_event) = pointer_event {
                let button = button_event.button();
                if let Some(mouse_button) = match button {
                    272 => Some(MouseButton::LeftButton),
                    273 => Some(MouseButton::RightButton),
                    274 => Some(MouseButton::MiddleButton),
                    275 => Some(MouseButton::X1Button),
                    276 => Some(MouseButton::X2Button),
                    _ => None,
                } {
                    if button_event.button_state() == ButtonState::Pressed {
                        BUTTON_STATES.lock().unwrap().insert(mouse_button, true);
                        if let Some(Bind::NormalBind(cb)) = MOUSE_BINDS.lock().unwrap().get(&mouse_button) {
                            let cb = Arc::clone(cb);
                            spawn(move || cb());
                        };
                    } else {
                        BUTTON_STATES.lock().unwrap().insert(mouse_button, false);
                    }
                }
            }
        }
        _ => {}
    }
}

trait DisplayAcquirable {
    fn with<F, Z>(&self, cb: F) -> Z
    where
        F: FnOnce(*mut Display) -> Z;
}

impl DisplayAcquirable for AtomicPtr<Display> {
    fn with<F, Z>(&self, cb: F) -> Z
    where
        F: FnOnce(*mut Display) -> Z,
    {
        let display = self.load(Ordering::Relaxed);
        unsafe {
            XLockDisplay(display);
        };
        let cb_result = cb(display);
        unsafe {
            XFlush(display);
            XUnlockDisplay(display);
        };
        cb_result
    }
}
