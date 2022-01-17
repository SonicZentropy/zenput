use crate::public::*;
pub use std::{
    collections::hash_map::HashMap,
    sync::atomic::{AtomicPtr, Ordering},
    sync::{Arc, Mutex},
    thread::spawn,
};
use std::collections::HashSet;
use once_cell::sync::Lazy;


pub enum Bind {
    NormalBind(bool, BindHandler), // bool for propagating original key event or not
    BlockBind(BlockBindHandler),
    BlockableBind(BlockableBindHandler),
}

pub type BindHandler = Arc<dyn Fn() + Send + Sync + 'static>;

pub type BlockBindHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockableBindHandler = Arc<dyn Fn() -> BlockInput + Send + Sync + 'static>;
pub type KeybdBindMap = HashMap<KeybdKey, Bind>;
pub type MouseBindMap = HashMap<MouseButton, Bind>;
pub type InternalKeypressCache = HashSet<KeybdKey>;

pub static KEYBD_BINDS: Lazy<Mutex<KeybdBindMap>> = Lazy::new(|| Mutex::new(KeybdBindMap::new()));
pub static MOUSE_BINDS: Lazy<Mutex<MouseBindMap>> = Lazy::new(|| Mutex::new(MouseBindMap::new()));
pub static INTERNAL_KEY_CACHE: Lazy<Mutex<InternalKeypressCache>> = Lazy::new(|| Mutex::new(InternalKeypressCache::new()));
