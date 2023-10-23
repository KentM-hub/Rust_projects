use web_sys::Window;
use web_sys::Document;

marco_rules! log {
    ($( $t:tt) *) => {
        web_sys::console::log_1(&format!( $($t)*).into());
    }
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}

pub fn document() -> Result<Document> {
    window()?.document().ok_or_else(|| anyhow!("No Document Found"))
}