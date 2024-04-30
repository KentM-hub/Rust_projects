/*
use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsCast;
use rand::prelude::*;
use std::rc::Rc;
use std::sync::Mutex;

use serde::Deserialize;
use std::collections::HashMap;
use anyhow::{anyhow,Result};

//use async_std::task;
// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}
#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String,Cell>,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}

#[macro_use]
mod browser;

*/

#[macro_use]
mod browser;
mod engine;
mod game;


use engine::GameLoop;
use game::WalkTheDog;
use wasm_bindgen::prelude::*;


#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    browser::spawn_local(async move {
        let game = WalkTheDog::new();
        GameLoop::start(game)
            .await
            .expect("Could not start Game Loop");
        });
        Ok(())
             

}

