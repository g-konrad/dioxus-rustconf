use dioxus::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(PartialEq, Props)]
struct VideoItemProps<'a> {
    video: &'a Video,
    detail: UseState<Option<usize>>,
}

#[derive(PartialEq, Props)]
struct VideoDetailProps<'a> {
    video: &'a Video,
}

fn VideoItem<'a>(cx: Scope<'a, VideoItemProps<'a>>) -> Element {
    let video = cx.props.video;

    cx.render(rsx!(
        p {
            onclick: move |_| cx.props.detail.set(Some(video.id)),
            "{video.speaker}: {video.title}"
        }
    ))
}

fn VideoDetail<'a>(cx: Scope<'a, VideoDetailProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            h3 {
                "{cx.props.video.title}"
            }
            img {
                src: "https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder",
                alt: "video thumbnail"
            }
        }
    ))
}

fn app(cx: Scope) -> Element {
    let detail = use_state(&cx, || None);
    let videos = use_future(&cx, (), |_| async move {
        Request::get("/tutorial/data.json")
            .send()
            .await
            .unwrap()
            .json::<Vec<Video>>()
            .await
            .unwrap()
    });

    let video_list = match videos.value() {
        Some(vs) => rsx!(vs.iter().map(|v: &Video| {
            rsx!(VideoItem {
                video: v,
                detail: detail.clone()
            })
        })),
        None => rsx!(""),
    };

    let video_detail = match *detail.current() {
        Some(id) => match videos.value() {
            Some(vs) => rsx!(VideoDetail { video: &vs[id - 1] }),
            None => rsx!(""),
        },
        None => rsx!(""),
    };

    cx.render(rsx!(
        h1 { "RustConf Explorer" }
        div {
            h3 { "Videos to watch" }
            video_list
            video_detail
        }
    ))
}
fn main() {
    dioxus::web::launch(app);
}
