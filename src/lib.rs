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

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("No Canvas Element found with ID 'canvas'"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|element| anyhow!("Error converting {:#?} to HtmlCanvasElement" , element))
}

pub fn context() -> Result<CanvasRenderingContext2d>{
    canvas()?
    .get_context("2d")
    .map_err(|js_value| anyhow!("Error getting 2d context {:#?}" , js_value))?
    .ok_or_else(|| anyhow!("No 2d context found"))?
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .map_err(|element| {
        anyhow!( "Error converting {:#?} to CanvasRenderingContext2d" , element)
    })
        
}

pub fn spawn_local<F> (future: F)
where
    F: Future<Output = ()> + 'static,
    {
        wasm_bindgen_futures::spawn_local(future);
    }
// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let document = browser::document().expect("No Document Found");
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

            wasm_bindgen_futures::spawn_local(async move {
               
                let json = fetch_json("rhb.json")
                    .await
                    .expect("Could not fetch rhb.json");

                let sheet: Sheet= json
                    .into_serde()
                    .expect("Could not convert rhb.json into a Sheet structure");

                let (success_tx,success_rx) = futures::channel::oneshot::channel::<Result<(),JsValue>>();
                
                let success_tx = Rc::new(Mutex::new(Some(success_tx)));
                let error_tx = Rc::clone(&success_tx);
                let image  = web_sys::HtmlImageElement::new().unwrap();
                let callback = Closure::once(move || {
                    if let Some(success_tx) =success_tx.lock().ok().and_then(|mut opt| opt.take()){
                        success_tx.send(Ok(()));
                    }
                        
                        //.send(Ok(()));
                    //web_sys::console::log_1(&JsValue::from_str("loaded"));
                });
                let error_callback = Closure::once(move |err| {
                    if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()){
                        error_tx.send(Err(err));
                    }
                        
                });
                image.set_onload(Some(callback.as_ref().unchecked_ref()));
                image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
                callback.forget();
                image.set_src("rhb.png");
                success_rx.await;

                
                let mut frame = -1;
                let interval_callback = Closure::wrap(Box::new(move || {
                    frame = (frame+1)%8;
                    
                    let frame_name = format!("Run ({}).png", frame+1);
                    let sprite = sheet.frames.get(&frame_name).expect("Cell not found");
                    context.clear_rect(0.0,0.0,600.0,600.0);
                    
                    context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &image,
                    sprite.frame.x.into(),
                    sprite.frame.y.into(),
                    sprite.frame.w.into(),
                    sprite.frame.h.into(),
                    300.0,
                    300.0,
                    sprite.frame.w.into(),
                    sprite.frame.h.into(),
                    );
                }) as Box<dyn FnMut()>);

                browser::window()
                    .unwrap()
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        interval_callback.as_ref().unchecked_ref(),
                        50,
                );
                interval_callback.forget();

                
                
                

            
            
        });
    
    Ok(())
}

async fn fetch_json(json_path: &str) -> Result<JsValue,JsValue> {
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(
        window.fetch_with_str(json_path)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;
    wasm_bindgen_futures::JsFuture::from(resp.json()?).await
}


























