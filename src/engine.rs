



#[derive(Deserialize)]
struct SheetRect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

#[derive(Deserialize)]
struct Cell {
    frame: SheetRect,
}

#[derive(Deserialize)]
pub struct Sheet {
    frames: HashMap<String,Cell>,
}

pub struct WalkTheDog {
    image: Option<HtmlImageElement>,
    sheet: Option<Sheet>,
    frame: u8,
}
impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog {
            image: None,
            sheet: None,
            frame: 0,
        }
    }
}

enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}



pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}



pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn clear(&self,rect: &Rect) {
        self.context.claer_rect(
            rect.x.info(),
            rect.y.info(),
            rect.width.info(),
            rect.height.info(),
        )
    }
}

pub fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect){
    self.context    
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &image,
            frame.x.info(),
            frame.y.info(),
            frame.width.info(),
            frame.height.info(),
            destination.x.info(),
            destination.y.info(),
            destination.width.info(),
            destination.height.info(),
        )
        .expect("Drawing is throwing execptions! Unrecovering error.");
}






pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;

    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx= Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc :: clone(&success_tx);
    let success_callback = browser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok()
            .and_then(|mut opt| opt.take()){
                success_tx.send(Ok(()));
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> =
        browser::closure_once(move |err| {
            if let Some(error_tx) = error_tx.lock().ok()
                .and_then(|mut opt| opt.take()){
                    error_tx.send(Err(anyhow!("Error Loading Image: {:#?}", err)));
                }
    });
    image.set.onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set.onerror(Some(error_callback.as.ref().unchecked_ref()));
    image.set_src(source);

    complete_rx.await??;

    Ok(image)

}

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self);
    fn draw(&self, context: &Renderer);

}

const FRAME_SIZE: f32 = 1.0/60.0*1000.0;

pub struct GameLoop {
    last_frame :f64,
    accumulated_delta: f32,
}
type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl GameLoop {
    pub async fn start(mut game:impl Game + 'static) -> Result <()> {
        let mut game = game.initialize().await?;
        let mut game_loop = GameLoop {
            last_frame: browser::now()?;
            accumulated_delta: 0.0,
        };

        let renderer = Renderer{
            context: browser::context()?;
        };

        let f = SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(browser::create_raf_closure(move|perf: f64| {
            game_loop.accumulated_delta += (perf - game_loop.last_frame) ans f32;
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update();
                game_loop.accumulated_delta -= FRAME_SIZE;
            }
            game_loop.last_frame = perf;
            game.draw(&renderer);

            browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));
    }
    browser::request_animation_frame(
        g.borrow()
            .as_ref()
            .ok_or_else(||anyhow!("GameLoop: Loop is None"))?,
    )?;
    OK(())
}

fn prepare_input() -> Result <UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_reveiver) = unbounded();
    let keydown_sender = Rc::new(RcfCell::new(keydown_sender));
    ket keyup_sender = Rc::clone(&keydown_sender);
    let onkeydown = browser::closure_wrap(
        Box::new(move |keycode: web_sys::KeyboardEvent| {
            keydown_sender
                .borrow_mut()
                .start_send(KeyPress::KeyDown(keycode));
        }) 
            as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    let onkeyup = browser::closure_wrap(
        Box::new(move |keycode: web_sys::KeyboardEvent| {
            keyup_sender
                .borrow_mut()
                .start_send(KeyPress::KeyUp(keycode));
        })
            as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    browser::window()?.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));

    browser::window()?.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    
    onkeydown.forget();
    onkeyup.forget();

    Ok(keyevent_reveiver)

}

fn process_input(
    state: &mut KeyState,
    keyevent_reveiver: &mut UnboundedReceiver<KeyPress>
){
    loop{
        match keyevent_reveiver.try_next(){
            Ok(None) =>break,
            Err(_err) =>break,
            Ok(Some(evt)) => match evt {
                KeyPress::KeyUp(evt) => state.set_released(&evt.code()),
                KeyPress::KeyDown(evt) =>state.set_pressed(&evt.code(),evt),
            },
        };
    }
}

pub struct KeyState {
    pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
}

impl KeyState {
    fn new() -> Self {
        KeyState {
            pressed_keys: HashMap::new(),
        }
    }
    pub fn is_pressed(&self,code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }
    fn set_pressed_keys.insert(code.into(),event){
        self.pressed_keys.insert(code.into(),event);
    }
    fn set_released(&mut self, code: &str){
        self.pressed_keys_remove(code.into());
    }

}