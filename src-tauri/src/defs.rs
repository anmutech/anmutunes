use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Track {
    pub id: i64,
    pub name: String,
    pub artist_id: i64,
    pub album_artist_id: i64,
    pub album_id: i64,
    pub genre_id: i64,
    pub total_time: i32, // (ms to s?)
    pub disc_number: i32,
    pub track_number: i32,
    //cover: String, // base64 encoded jpg/png?
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DBTrack {
    pub orig_track_id: i64,
    pub name: String,
    pub artist_id: i64,
    pub album_artist_id: i64,
    pub composer_id: i64,
    pub album_id: i64,
    pub genre_id: i64,
    pub kind: String,
    pub size: i64,
    pub total_time: i64,
    pub disc_number: i64,
    pub disc_count: i64,
    pub track_number: i64,
    pub track_count: i64,
    pub year: i64,
    pub date_modified: String,
    pub date_added: String,
    pub bit_rate: i64,
    pub sample_rate: i64,
    pub release_date: String,
    pub normalization: i64,
    pub artwork_count: i64,
    pub sort_name: String,
    pub persistent_id: String,
    pub track_type: String,
    pub purchased: i64,
    pub has_video: i64,
    pub hd: i64,
    pub video_width: i64,
    pub video_height: i64,
    pub music_video: i64,
    pub location: String,
    pub file_folder_count: i64,
    pub library_folder_count: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AudioTrack {
    pub id: i64,
    pub location: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Album {
    pub id: i64,
    pub artist_id: i64,
    pub name: String,
    pub sort_album: String,
    pub genre_id: i64,
    pub year: i64,
    pub release_date: String,
    pub date_modified: String,
    pub date_added: String,
    pub tracks: Vec<i64>,
    pub cover_id: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cover {
    pub id: i64,
    pub album_id: i64,
    pub data: String, // base64 encoded jpg/png/bmp/etc.
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub sort_artist: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ArtistAlbums {
    pub id: i64,
    pub albums: Vec<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ArtistTracks {
    pub id: i64,
    pub tracks: Vec<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Composer {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ComposerTracks {
    pub id: i64,
    pub tracks: Vec<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GenreTracks {
    pub id: i64,
    pub tracks: Vec<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub tracks: Vec<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DBPlaylist {
    pub orig_playlist_id: i64,
    pub name: String,
    pub description: String,
    pub master: i64,
    pub persistent_id: String,
    pub parent_persistent_id: String,
    pub distinguished_kind: i64,
    pub visible: i64,
    pub all_items: i64,
    pub folder: i64,
    pub smart_info: String,
    pub smart_criteria: String,
    pub date_modified: String,
    pub date_added: String,
    pub tracks: Vec<i64>,
}

impl Default for DBPlaylist {
    fn default() -> DBPlaylist {
        return DBPlaylist {
            orig_playlist_id: 0,
            name: "".to_string(),
            description: "".to_string(),
            master: 0,
            persistent_id: "".to_string(),
            parent_persistent_id: "".to_string(),
            distinguished_kind: 0,
            visible: 1,
            all_items: 0,
            folder: 0,
            smart_info: "".to_string(),
            smart_criteria: "".to_string(),
            date_modified: "".to_string(),
            date_added: "".to_string(),
            tracks: vec![],
        };
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum RepeatMode {
    RepeatTrack,
    RepeatQueue,
    RepeatNone,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Output {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Order {
    ByName,
    ByReleaseDate,
    ByAddedDate,
    ByModifiedDate,
    ByArtist,
    ByAlbumArtist,
    ByComposer,
    ByAlbum,
    ByGenre,
    BySize,
    ByTime,
    ByNameInverse,
    ByReleaseDateInverse,
    ByAddedDateInverse,
    ByModifiedDateInverse,
    ByArtistInverse,
    ByAlbumArtistInverse,
    ByComposerInverse,
    ByAlbumInverse,
    ByGenreInverse,
    BySizeInverse,
    ByTimeInverse,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum DataType {
    Artist,
    Composer,
    Album,
    Cover,
    Genre,
    Track,
    Video, // TODO: Do we need type video? Currently videos are just tracks
    Playlist,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AudioRequest {
    // Playback Events
    PlayPause(bool), // true is play, false is pause
    Next,
    Prev,
    QueueJump(i64),
    QueueMove(Vec<i64>),
    QueueRemove(Vec<i64>),
    HistoryJump(i64),
    HistoryRemove,
    Mute(bool), // true is mute, false is not
    Volume(i32),
    Output(i32), // uses the output id
    Seek(i64),
    Shuffle(bool),
    Repeat(RepeatMode), // no repeat, repeat one, repeat queue
    Init,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DBRequest {
    Play(DataType, Vec<i64>, Option<Vec<Order>>), // Play track(s),album(s),artist(s),composer(s),genre(s),playlist(s), by ID(s). composer and genre supports order.
    // Queue
    QueueInsert(DataType, Vec<i64>, Option<usize>, Option<Vec<Order>>), // insert track(s),album(s),artist(s),composer(s),genre(s),playlist(s), by ID(s). composer and genre supports order. defaults to append if no index is given
    AudioBackendRecover(Vec<i64>, Vec<i64>),
    // Playlist
    NewPlaylist(Playlist), // Save current playlist with name and array of track IDs
    UpdatePlaylist(Playlist),
    // Files
    AddToLibrary(Vec<String>), // Add file(s)/directory to library
    ImportLibrary(String),
    UpdateTrackLocations(String, String),
    DeleteById(DataType, Vec<i64>, bool), // Delete from db, delete files if allowed
    // EditMetadata(String, HashMap<String, String>), // e.g., { "artist": "Queen" }
    // Data
    GetDataOrder(DataType, Option<Vec<Order>>), // type, order (default alphabetical)
    GetCoversById(Vec<i64>),
    GetTrackPaths(Vec<i64>),
    Search(String, Option<Vec<DataType>>, Option<i64>),
    UpdateTracks(Vec<Track>, Vec<String>, Vec<String>, Vec<String>),
    UpdateAlbum(Album, String, String),
    UpdateArtist(Artist),
    UpdateComposer(Composer),
    UpdateGenre(Genre),
    ExtractCovers,
    ExtractCover(i64),
    Init,
    UpdateConfig(ConfigState), // Only to be called from a configrequest after updating config.json file
    OpenContainingDir(DataType, i64),
    CopyNotCopied,
    ListenedToTrack(i64),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DBData {
    Play(Vec<AudioTrack>), // array of filepaths, overwrites current queue
    QueueInsert(Vec<AudioTrack>, Option<usize>), // insert filepaths. defaults to append if no index is given
    AudioBackendRecover(Option<Vec<AudioTrack>>, Option<Vec<AudioTrack>>),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ConfigRequest {
    Get,
    Set(ConfigState),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DBState {
    pub tracks_max: Option<i64>,
    pub albums_max: Option<i64>,
    pub artists_max: Option<i64>,
    pub genres_max: Option<i64>,
    pub playlists_max: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AudioState {
    pub is_playing: Option<bool>,
    pub is_muted: Option<bool>,
    pub volume: Option<i32>,
    pub output: Option<Output>,
    pub position: Option<i64>, // Current playback position (ms or s?)
    pub shuffle_mode: Option<bool>,
    pub repeat_mode: Option<RepeatMode>,
    pub current_track: Option<i64>,
    pub queue: Option<Vec<i64>>,
    pub history: Option<Vec<i64>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AudioBackendState {
    pub volume: i32,
    pub position: i64,
    pub shuffle_mode: bool,
    pub repeat_mode: RepeatMode,
    pub current_id: i64,
    pub current_location: String,
    pub queue: Vec<i64>,
    pub history: Vec<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConfigState {
    pub version: Version,
    pub theme: Theme,
    pub custom_colors: ThemeColors,
    pub startup_view: View,
    pub language: Language,
    pub look_for_updates: bool,
    pub media_path: String,
    pub manage_folders: bool,
    pub allow_delete_from_db: bool,
    pub allow_delete_files: bool,
    pub is_new: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Version {
    pub major: i64,
    pub minor: i64,
    pub patch: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Theme {
    System,
    Light,
    Dark,
    Custom,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ThemeColors {
    pub background: String,
    pub background_active: String,
    pub background_hover: String,
    pub background_button: String,
    pub border_color: String,
    pub accent_input: String,
    pub warn: String,
    pub text: String,
    pub text_dim: String,
    pub text_highlight: String,
    pub shadow: String,
}

impl Default for ThemeColors {
    fn default() -> ThemeColors {
        return ThemeColors {
            background: "#ffffff".to_string(),
            background_active: "#eae1e1".to_string(),
            background_hover: "#efe8e8".to_string(),
            background_button: "#f5f0f0".to_string(),
            border_color: "#d7d1d1".to_string(),
            accent_input: "#d7d1d1".to_string(),
            warn: "#aa0000".to_string(),
            text: "#000000".to_string(),
            text_dim: "#787878".to_string(),
            text_highlight: "#3b3b3b".to_string(),
            shadow: "#00000020".to_string(),
        };
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum View {
    Recents,
    Tracks,
    Albums,
    Artists,
    Composers,
    Genres,
    Playlists,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Language {
    System,
    Czech,
    Danish,
    German,
    Greek,
    English,
    Spanish,
    French,
    Italian,
    Japanese,
    Korean,
    Luxembourgish,
    Dutch,
    Polish,
    Portuguese,
    Turkish,
    Chinese,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BackendMessage {
    pub notification: Option<Notification>,
    pub error: Option<String>,
    pub warning: Option<Warning>,
    pub progress: Option<Progress>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Notification {
    LibraryImport,
    None,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Warning {
    None,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Progress {
    pub info: ProgressInfo,
    pub value: Option<i64>,
    pub done: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ProgressInfo {
    LibraryImport,
    FileImport,
    CoverExtract,
    Delete,
    UpdateTracks,
    UpdateAlbum,
    None,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Search {
    pub tracks: Option<Vec<i64>>,
    pub albums: Option<Vec<i64>>,
    pub genres: Option<Vec<i64>>,
    pub artists: Option<Vec<i64>>,
    pub composers: Option<Vec<i64>>,
    pub playlists: Option<Vec<i64>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Data {
    // queue is a list of track IDs
    pub queue: Option<Vec<i64>>,
    pub tracks: Option<Vec<Track>>,
    pub albums: Option<Vec<Album>>,
    pub artists: Option<Vec<Artist>>,
    pub artist_albums: Option<Vec<ArtistAlbums>>,
    pub artist_tracks: Option<Vec<ArtistTracks>>,
    pub composers: Option<Vec<Composer>>,
    pub composer_tracks: Option<Vec<ComposerTracks>>,
    pub covers: Option<Vec<Cover>>,
    pub genres: Option<Vec<Genre>>,
    pub genre_tracks: Option<Vec<GenreTracks>>,
    pub playlists: Option<Vec<Playlist>>,
    pub spacetime: Option<SpaceTime>,
    pub search: Option<Search>,
    pub albums_order: Option<(Vec<Order>, Vec<i64>)>,
    pub artists_order: Option<(Vec<Order>, Vec<i64>)>,
    pub composers_order: Option<(Vec<Order>, Vec<i64>)>,
    pub genres_order: Option<(Vec<Order>, Vec<i64>)>,
    pub playlists_order: Option<(Vec<Order>, Vec<i64>)>,
    pub tracks_order: Option<(Vec<Order>, Vec<i64>)>,
    pub error: Option<String>,
    pub loading: Option<Loading>,
    pub is_init: Option<bool>,
    //pub selectedTracks: Vec<usize>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Loading {
    pub index: Option<i64>,
    pub max: Option<i64>,
    pub message: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpaceTime {
    pub space: Option<i64>,
    pub time: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Image {
    pub media_type: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Meta {
    pub name: Option<String>,
    pub artist: Option<String>,
    pub album_artist: Option<String>,
    pub composer: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>, // check Genrest table, create new if not existing from inexistent? tags[]
    pub kind: Option<String>,
    pub size: Option<i64>, // not available? calculate from copy file or os call.
    pub total_time: Option<i64>,
    pub disc_number: Option<i64>,
    pub disc_count: Option<i64>,
    pub track_number: Option<i64>,
    pub track_count: Option<i64>,
    pub year: Option<i64>,
    pub bit_rate: Option<i64>,
    pub sample_rate: Option<i64>,
    pub release_date: Option<String>,
    // normalization: not really available... tags[15-18] have values, but not sure if useful, also so far no use for normalization value.,
    // artwork_count: -1, although theoretically I could use Visuals [01] FrontCover,
    // sort_name: not available, also not really used,
    // persistent_id: not available, can calculate hash with salt of current time to prevent collision or pass some uuid,
    // track_type: "File", aka useless,
    // purchased: NULL,
    pub location: Option<String>,
    // file_folder_count: always 5 as it seems...,
    // library_folder_count: always 1 as it seems...,
    pub cover: Option<Image>,
}
