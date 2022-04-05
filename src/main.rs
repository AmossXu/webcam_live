use serde_json::json;
use sycamore::futures::{spawn_local, ScopeSpawnLocal};
use sycamore::{prelude::*, view};
use web_sys::HtmlVideoElement;
use webcam_live::VideoStream;
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

#[component]
fn Video<G: Html>(ctx: ScopeRef) -> View<G> {
    let video_ref = ctx.create_node_ref::<G>();
    ctx.spawn_local(async move {
        let el = video_ref.get::<DomNode>().unchecked_into();
        let video_stream = VideoStream::new(el);

        video_stream
            .set_video_src(&json!({
                "audio": false,
                "video": {
                    "facingMode": "enviroment",
                    "width": 640,
                    "height": 480
                }
            }))
            .await;
    });
    view!(ctx, div {
        video(
            ref=video_ref,
            class="border border-gray-400 rounded-lg",
            autoplay=false,
            width=1280,
            height=720
        )
    })
}

// #[component]
// fn Video<G: Html>(ctx: ScopeRef) -> View<G> {
//     //     // 异步 下一个event loop执行

//     //     // create_scope(|ctx| {
//     //     //     let video_ref = ctx.create_node_ref();

//     //     //     spawn_local(async move {
//     //     //         let el = video_ref
//     //     //             .get::<DomNode>()
//     //     //             .unchecked_into::<HtmlVideoElement>();
//     //     //         let video_stream = VideoStream::new(el);
//     //     //     });
//     //     // });
//     view! {ctx,
//         div {
//             video(
//                 // ref=video_ref,
//                 class="border border-gray-400 rounded-lg",
//                 autoplay=false,
//                 width=1280,
//                 height=720
//             )
//         }
//     };
// }
