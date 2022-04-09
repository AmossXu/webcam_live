use crate::{VideoStream, Devices};
use serde_json::json;
use sycamore::futures::ScopeSpawnLocal;
use sycamore::{prelude::*, view};
use tracing::info;

#[component]
pub fn Video<G: Html>(ctx: ScopeRef) -> View<G> {
    let video_ref = ctx.create_node_ref::<G>();
    ctx.spawn_local(async move {
        let el = video_ref.get::<DomNode>().unchecked_into();
        let video_stream = VideoStream::new(el);

        video_stream
            .set_video_src(&json!({
                "audio": false,
                "video": {
                    "facingMode": "user",
                    "width": 640,
                    "height": 480
                }
            }))
            .await;

            let devices = Devices::load().await;
            info!("divices: {:?}", devices);
    });
    view!(ctx, div {
        video(
            ref=video_ref,
            class="border border-gray-400 rounded-lg",
            autoplay=true,
            width=640,
            height=480,
            // control=true
        )
    })
}
