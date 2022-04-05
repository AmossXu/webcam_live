use sycamore::view;
use webcam_live::Video;
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    // println!("Hello, world!");
    sycamore::render(|ctx| {
        view! {ctx,
            div(class="container p-2 hero-text") {
                Video()
            }
        }
    })
}
