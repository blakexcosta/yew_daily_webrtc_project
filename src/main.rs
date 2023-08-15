mod components;
mod moretests;
mod test;

use components::songdetails::*;
use components::songslist::*;
use components::videodetails::*;
use components::videoslist::*;
use gloo_net::http::Request;
use moretests::experimental::*;
use test::moretests1::*;
use test::moretests2::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // walkthrough based upon: https://yew.rs/docs/tutorial#handling-state
    // run with `trunk serve --proxy-backend=https://yew.rs/tutorial` if using rest api to retrieve data or `trunk serve --open` otherwise
    // videos vec
    // let videos_vec = vec![
    //     Video {
    //         id: 1,
    //         title: "Building and breaking things".to_string(),
    //         speaker: "John Doe".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 2,
    //         title: "The development process".to_string(),
    //         speaker: "Jane Smith".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 3,
    //         title: "The Web 7.0".to_string(),
    //         speaker: "Matt Miller".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 4,
    //         title: "Mouseless development".to_string(),
    //         speaker: "Tom Jerry".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 5,
    //         title: "More Development work".to_string(),
    //         speaker: "Bobs Burgers".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    // ];

    // get videos from rest api
    let videos_vec = use_state(|| vec![]);
    {
        // query https api
        let videos_vec = videos_vec.clone();
        use_effect_with_deps(
            move |_| {
                let videos_vec = videos_vec.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos_vec.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    // songs vec
    let song_vec = vec![
        Song {
            id: 1,
            title: "Song 1".to_string(),
            artist: "Artist 1".to_string(),
        },
        Song {
            id: 2,
            title: "Song 2".to_string(),
            artist: "Artist 2".to_string(),
        },
    ];

    // // buttons
    // let button3 = button("Button 3");
    // let button2 = button("Button 2");
    // let button = button("Button 1");
    // // let buttons = vec![button, button2, button3];
    // let buttons = vec![button, button2, button3]
    //     .iter()
    //     .map(|b| {
    //         html! {
    //             // have to clone the value otherwise get issues with borrow checker with shared reference
    //             <p>{b.clone()}</p>
    //         }
    //     })
    //     .collect::<Html>();

    // TODO: Research Handling state and Fetching external Rest API data
    // TODO: Code Review

    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };
    let video_details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    // set initial song state
    let selected_song = use_state(|| None);
    // set new song state
    let on_song_select = {
        let selected_song = selected_song.clone();
        Callback::from(move |song: Song| selected_song.set(Some(song)))
    };
    // render song details to screen when selected
    let song_details = selected_song.as_ref().map(|song| {
        html! {
            <div>
            <SongDetails song={song.clone()} />
            <p>{format!("oogly boogly, triggered with onclick action")}</p>
            </div>
        }
    });

    // Returned HTML
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                // <p>{ "John Doe: Building and breaking things" }</p>
                // <p>{ "Jane Smith: The development process" }</p>
                // <p>{ "Matt Miller: The Web 7.0" }</p>
                // <p>{ "Tom Jerry: Mouseless development" }</p>
                //{videos}
                <VideosList videos={(*videos_vec).clone()} on_click={on_video_select.clone()} />

            </div>
            // <div>
            //     <h3>{ "John Doe: Building and breaking things" }</h3>
            //     <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
            // </div>
            { for video_details }
            // Appended Code ----------------
            <div>
                <SongsList songs={song_vec.clone()} on_click={on_song_select.clone()}/>
            </div>

            { for song_details }
            // <div>
            //     {buttons}
            // </div>
            // Appended Code ----------------
            // display values of newly created code


        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
