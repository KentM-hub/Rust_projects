use anyhow::Result;
use async_trait::async_trait;
use web_sys::HtmlImageElement;


use crate::{
    browser,
    engine::{self, Game , KeyState, Point , Rect , Renderer , Sheet},
};

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
    position: Point,
}
impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog {
            image: None,
            sheet: None,
            frame: 0,
            position: Point { x:0, y:0},
        }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
            let sheet = browser::fetch_json("rhb.json").await?.into_serde()?;
            let image = Some(engine::load_image("rhb.png").await?);
            Ok(Box::new(WalkTheDog {
                image,
                sheet,
                position: self.position,
                frame: self.frame,
            }))
    }
    fn update(&mut self, keystate: &KeyState){
        let mut velocity = Point {x: 0, y:0}
        if keystate.is_pressed("ArrowDown") {
            velocity.y+=3;
        }
        if keystat.is_pressed("ArrowUp"){
            velocity.y -=3;
        }
        if keystate.is_pressed("ArrowRight"){
            velocity.x +=3;
        }
        if keystate.is_pressed("ArrowLeft"){
            velocity.x -=3;
        }
        self.position.x+=velocity.x;
        self.position.y +=velocity.y;

        if self.frame < 23 {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
    }
    fn draw(&self, renderer: &Renderer) {
      

        renderer.clear(Rect {
            x:0.0,
            y:0.0,
            width:600.0,
            height:600.0,
        });
        let current_sprite=(self.frame/3)+1;
        let frame_name = format!("Run ({}).png",current_sprite);
        let sprite = self   
            .sheet
            .as_ref()
            .and_then(|sheet| sheet.frames.get(&frame_name))
            .expect("Cell not found");

        self.image.as_ref().map(|image| {
            renderer.draw_image(
                &image,
                &Rect {
                    x: sprite.frame.x.into(),
                    y: sprite.frame.y.into(),
                    width: sprite.frame.w.into(),
                    htight: sprite.frame.h.into(),
                },
                &Rect {
                    x:self.position.x.into(),
                    y:self.position.x.into(),
                    width: sprite.frame.w.into(),
                    htight: sprite.frame.h.into(),
                }
            )
        })
        
    }
}

