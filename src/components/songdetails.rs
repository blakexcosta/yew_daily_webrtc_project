use crate::components::songslist::*;
use yew::prelude::*;

// props
#[derive(Properties, PartialEq)]
pub struct SongDetailsProps {
    pub song: Song,
}
// SongDetails component
#[function_component(SongDetails)]
pub fn video_details(SongDetailsProps { song }: &SongDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ song.id.clone() }</h3>
            <h3>{ song.title.clone() }</h3>
            <h3>{ song.artist.clone() }</h3>
        </div>
    }
}
