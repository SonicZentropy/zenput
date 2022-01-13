use crate::common::*;
use std::{thread::sleep, time::Duration};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub enum BlockInput {
    Block,
    DontBlock,
}

//AKey/BKey etc plain enum with nothing in it
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
pub enum KeybdKey {
    BackspaceKey,
    TabKey,
    EnterKey,
    EscapeKey,
    SpaceKey,
    HomeKey,
    LeftKey,
    UpKey,
    RightKey,
    DownKey,
    InsertKey,
    DeleteKey,
    Numrow0Key,
    Numrow1Key,
    Numrow2Key,
    Numrow3Key,
    Numrow4Key,
    Numrow5Key,
    Numrow6Key,
    Numrow7Key,
    Numrow8Key,
    Numrow9Key,
    AKey,
    BKey,
    CKey,
    DKey,
    EKey,
    FKey,
    GKey,
    HKey,
    IKey,
    JKey,
    KKey,
    LKey,
    MKey,
    NKey,
    OKey,
    PKey,
    QKey,
    RKey,
    SKey,
    TKey,
    UKey,
    VKey,
    WKey,
    XKey,
    YKey,
    ZKey,
    Numpad0Key,
    Numpad1Key,
    Numpad2Key,
    Numpad3Key,
    Numpad4Key,
    Numpad5Key,
    Numpad6Key,
    Numpad7Key,
    Numpad8Key,
    Numpad9Key,
    F1Key,
    F2Key,
    F3Key,
    F4Key,
    F5Key,
    F6Key,
    F7Key,
    F8Key,
    F9Key,
    F10Key,
    F11Key,
    F12Key,
    F13Key,
    F14Key,
    F15Key,
    F16Key,
    F17Key,
    F18Key,
    F19Key,
    F20Key,
    F21Key,
    F22Key,
    F23Key,
    F24Key,
    NumLockKey,
    ScrollLockKey,
    CapsLockKey,
    LShiftKey,
    RShiftKey,
    LControlKey,
    RControlKey,

    #[strum(disabled)]
    OtherKey(u64),
}



impl KeybdKey {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, should_propagate: bool,  callback: F) {
        KEYBD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::NormalBind(should_propagate, Arc::new(callback)));
    }

    pub fn block_bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        KEYBD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::BlockBind(Arc::new(callback)));
    }

    pub fn blockable_bind<F: Fn() -> BlockInput + Send + Sync + 'static>(self, callback: F) {
        KEYBD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::BlockableBind(Arc::new(callback)));
    }

    pub fn bind_all<F: Fn(KeybdKey) + Send + Sync + Copy + 'static>(callback: F) {
        for key in KeybdKey::iter() {
            let fire = move || {
                callback(key);
            };

            KEYBD_BINDS
                .lock()
                .unwrap()
                .insert(key, Bind::NormalBind(true, Arc::new(fire)));
        }
    }

    pub fn unbind(self) {
        KEYBD_BINDS.lock().unwrap().remove(&self);
    }
}



pub fn from_keybd_key(k: KeybdKey) -> Option<char> {
    match k {
        KeybdKey::AKey => Some('a'),
        KeybdKey::BKey => Some('b'),
        KeybdKey::CKey => Some('c'),
        KeybdKey::DKey => Some('d'),
        KeybdKey::EKey => Some('e'),
        KeybdKey::FKey => Some('f'),
        KeybdKey::GKey => Some('g'),
        KeybdKey::HKey => Some('h'),
        KeybdKey::IKey => Some('i'),
        KeybdKey::JKey => Some('j'),
        KeybdKey::KKey => Some('k'),
        KeybdKey::LKey => Some('l'),
        KeybdKey::MKey => Some('m'),
        KeybdKey::NKey => Some('n'),
        KeybdKey::OKey => Some('o'),
        KeybdKey::PKey => Some('p'),
        KeybdKey::QKey => Some('q'),
        KeybdKey::RKey => Some('r'),
        KeybdKey::SKey => Some('s'),
        KeybdKey::TKey => Some('t'),
        KeybdKey::UKey => Some('u'),
        KeybdKey::VKey => Some('v'),
        KeybdKey::WKey => Some('w'),
        KeybdKey::XKey => Some('x'),
        KeybdKey::YKey => Some('y'),
        KeybdKey::ZKey => Some('z'),
        KeybdKey::Numpad0Key => Some('0'),
        KeybdKey::Numpad1Key => Some('1'),
        KeybdKey::Numpad2Key => Some('2'),
        KeybdKey::Numpad3Key => Some('3'),
        KeybdKey::Numpad4Key => Some('4'),
        KeybdKey::Numpad5Key => Some('5'),
        KeybdKey::Numpad6Key => Some('6'),
        KeybdKey::Numpad7Key => Some('7'),
        KeybdKey::Numpad8Key => Some('8'),
        KeybdKey::Numpad9Key => Some('9'),
        KeybdKey::Numrow0Key => Some('0'),
        KeybdKey::Numrow1Key => Some('1'),
        KeybdKey::Numrow2Key => Some('2'),
        KeybdKey::Numrow3Key => Some('3'),
        KeybdKey::Numrow4Key => Some('4'),
        KeybdKey::Numrow5Key => Some('5'),
        KeybdKey::Numrow6Key => Some('6'),
        KeybdKey::Numrow7Key => Some('7'),
        KeybdKey::Numrow8Key => Some('8'),
        KeybdKey::Numrow9Key => Some('9'),
        _ => None
    }
}

pub fn get_keybd_key(c: char) -> Option<KeybdKey> {
    match c {
        ' ' => Some(KeybdKey::SpaceKey),
        'A' | 'a' => Some(KeybdKey::AKey),
        'B' | 'b' => Some(KeybdKey::BKey),
        'C' | 'c' => Some(KeybdKey::CKey),
        'D' | 'd' => Some(KeybdKey::DKey),
        'E' | 'e' => Some(KeybdKey::EKey),
        'F' | 'f' => Some(KeybdKey::FKey),
        'G' | 'g' => Some(KeybdKey::GKey),
        'H' | 'h' => Some(KeybdKey::HKey),
        'I' | 'i' => Some(KeybdKey::IKey),
        'J' | 'j' => Some(KeybdKey::JKey),
        'K' | 'k' => Some(KeybdKey::KKey),
        'L' | 'l' => Some(KeybdKey::LKey),
        'M' | 'm' => Some(KeybdKey::MKey),
        'N' | 'n' => Some(KeybdKey::NKey),
        'O' | 'o' => Some(KeybdKey::OKey),
        'P' | 'p' => Some(KeybdKey::PKey),
        'Q' | 'q' => Some(KeybdKey::QKey),
        'R' | 'r' => Some(KeybdKey::RKey),
        'S' | 's' => Some(KeybdKey::SKey),
        'T' | 't' => Some(KeybdKey::TKey),
        'U' | 'u' => Some(KeybdKey::UKey),
        'V' | 'v' => Some(KeybdKey::VKey),
        'W' | 'w' => Some(KeybdKey::WKey),
        'X' | 'x' => Some(KeybdKey::XKey),
        'Y' | 'y' => Some(KeybdKey::YKey),
        'Z' | 'z' => Some(KeybdKey::ZKey),
        _ => None,
    }
}


pub struct KeySequence(pub &'static str);

impl KeySequence {
    pub fn send(&self) {
        for c in self.0.chars() {
            let mut uppercase = false;
            if let Some(keybd_key) = {
                if c.is_uppercase() {
                    uppercase = true;
                }
                get_keybd_key(c)
            } {
                if uppercase {
                    KeybdKey::LShiftKey.press(true);
                }
                keybd_key.press(true);
                sleep(Duration::from_millis(20));
                keybd_key.release(true);
                if uppercase {
                    KeybdKey::LShiftKey.release(true);
                }
            };
        }
    }
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum MouseButton {
    LeftButton,
    MiddleButton,
    RightButton,
    X1Button,
    X2Button,
    OtherButton(u32),
}

pub struct MouseCursor;

pub struct MouseWheel;

impl MouseButton {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, should_propagate: bool, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::NormalBind(should_propagate, Arc::new(callback)));
    }

    pub fn block_bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::BlockBind(Arc::new(callback)));
    }

    pub fn blockable_bind<F: Fn() -> BlockInput + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::BlockableBind(Arc::new(callback)));
    }

    pub fn unbind(self) {
        MOUSE_BINDS.lock().unwrap().remove(&self);
    }
}
