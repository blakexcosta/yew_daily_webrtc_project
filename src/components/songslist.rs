use yew::prelude::*;

// song struct
#[derive(Clone, PartialEq)]
pub struct Song {
    pub id: usize,
    pub title: String,
    pub artist: String,
}
// creating new Props
#[derive(Properties, PartialEq)]
pub struct SongsListProps {
    pub songs: Vec<Song>,
    pub on_click: Callback<Song>, // callback
}
// songs list component
#[function_component(SongsList)]
pub fn songs_list(SongsListProps { songs, on_click }: &SongsListProps) -> Html {
    let on_click = on_click.clone();
    return songs
        .iter()
        .map(|song| {
            let on_song_select = {
                let on_click = on_click.clone();
                let song = song.clone();
                Callback::from(move |_| on_click.emit(song.clone())) //emit event onclick
            };
            html! {
                <p key={song.id} onclick={on_song_select}>{format!("{} BY {}", song.title, song.artist)}</p>
            }
        })
        .collect(); // dont have to call collect::<Html> here is the return type is Html
}
