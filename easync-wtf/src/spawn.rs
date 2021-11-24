use std::{future::Future, iter::Once};

use once_cell::sync::OnceCell;

pub type SpawnFn = fn(Box<dyn Future<Output = ()> + Send>);

pub static SPAWN: OnceCell<SpawnFn> = OnceCell::new();

pub fn init_executor(spawn_fn: SpawnFn) {
    let _ = SPAWN.set(spawn_fn);
}