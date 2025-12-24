export interface Track {
  id: number;
  name: string;
  artist_id: number;
  album_artist_id: number;
  album_id: number;
  genre_id: number;
  total_time: number; // (ms to s?)
  disc_number: number;
  track_number: number;
  //cover: string, // base64 encoded jpg/png?
}

export interface Album {
  id: number;
  artist_id: number;
  name: string;
  sort_album: string;
  genre_id: number;
  year: number;
  release_date: string;
  date_modified: string;
  date_added: string;
  tracks: number[];
  cover_id: number;
}

export interface Cover {
  id: number;
  album_id: number;
  data: string; // base64 encoded jpg/png
}

export interface Artist {
  id: number;
  name: string;
  sort_artist: string;
}

export interface ArtistAlbums {
  id: number;
  albums: number[];
}

export interface ArtistTracks {
  id: number;
  tracks: number[];
}

export interface Composer {
  id: number;
  name: string;
}

export interface ComposerTracks {
  id: number;
  tracks: number[];
}

export interface Genre {
  id: number;
  name: string;
}

export interface GenreTracks {
  id: number;
  tracks: number[];
}

export interface Playlist {
  id: number;
  name: string;
  description: string;
  tracks: number[]; // list of track IDs
}

export enum RepeatMode {
  RepeatTrack = "RepeatTrack",
  RepeatQueue = "RepeatQueue",
  RepeatNone = "RepeatNone",
}

export interface Output {
  id: number;
  name: string;
}

export enum Order {
  ByName = "ByName",
  ByReleaseDate = "ByReleaseDate",
  ByAddedDate = "ByAddedDate",
  ByModifiedDate = "ByModifiedDate",
  ByArtist = "ByArtist",
  ByAlbumArtist = "ByAlbumArtist",
  ByComposer = "ByComposer",
  ByAlbum = "ByAlbum",
  ByGenre = "ByGenre",
  BySize = "BySize",
  ByTime = "ByTime",
  ByNameInverse = "ByNameInverse",
  ByReleaseDateInverse = "ByReleaseDateInverse",
  ByAddedDateInverse = "ByAddedDateInverse",
  ByModifiedDateInverse = "ByModifiedDateInverse",
  ByArtistInverse = "ByArtistInverse",
  ByAlbumArtistInverse = "ByAlbumArtistInverse",
  ByComposerInverse = "ByComposerInverse",
  ByAlbumInverse = "ByAlbumInverse",
  ByGenreInverse = "ByGenreInverse",
  BySizeInverse = "BySizeInverse",
  ByTimeInverse = "ByTimeInverse",
}

export enum DataType {
  Artist = "Artist",
  Composer = "Composer",
  Album = "Album",
  Cover = "Cover",
  Genre = "Genre",
  Track = "Track",
  Video = "Video",
  Playlist = "Playlist",
  None = "None",
}

export enum ActiveView {
  Loading,
  First,
  Recents,
  Tracks,
  Albums,
  Artists,
  Composers,
  Genres,
  Playlists,
}

export interface DBState {
  tracks_max: number | null;
  albums_max: number | null;
  artists_max: number | null;
  genres_max: number | null;
  playlists_max: number | null;
  media_path: string | null;
}

export interface AudioState {
  is_playing: boolean | null;
  is_muted: boolean | null;
  volume: number | null;
  output: Output | null;
  position: number | null; // Current playback position (ms)
  shuffle_mode: boolean | null;
  repeat_mode: RepeatMode | null;
  current_track: number | null;
  queue: number[] | null;
  history: number[] | null;
}

export interface ConfigState {
  version: Version;
  theme: Theme;
  custom_colors: ThemeColors;
  startup_view: View;
  language: Language;
  look_for_updates: boolean;
  media_path: string;
  manage_folders: boolean;
  allow_delete_from_db: boolean;
  allow_delete_files: boolean;
  is_new: boolean;
}

export interface Version {
  major: number;
  minor: number;
  patch: number;
}

export enum Theme {
  System = "System",
  Light = "Light",
  Dark = "Dark",
  Custom = "Custom",
}

export interface ThemeColors {
  background: string;
  background_active: string;
  background_hover: string;
  background_button: string;
  border_color: string;
  accent_input: string;
  warn: string;
  text: string;
  text_dim: string;
  text_highlight: string;
  shadow: string;
}

export enum View {
  Recents = "Recents",
  Tracks = "Tracks",
  Albums = "Albums",
  Artists = "Artists",
  Composers = "Composers",
  Genres = "Genres",
  Playlists = "Playlists",
}

export enum Language {
  System = "System",
  Czech = "Czech",
  Danish = "Danish",
  German = "German",
  Greek = "Greek",
  English = "English",
  Spanish = "Spanish",
  French = "French",
  Italian = "Italian",
  Japanese = "Japanese",
  Korean = "Korean",
  Luxembourgish = "Luxembourgish",
  Dutch = "Dutch",
  Polish = "Polish",
  Portuguese = "Portuguese",
  Turkish = "Turkish",
  Chinese = "Chinese",
}

export interface Translation {
  common: {
    name: string;
    description: string;
    track_one: string;
    track_other: string;
    title: string;
    artist_one: string;
    artist_other: string;
    composer_one: string;
    composer_other: string;
    album_one: string;
    album_other: string;
    albumartist: string;
    genre_one: string;
    genre_other: string;
    cdnumber: string;
    cdnumber_long: string;
    tracknumber: string;
    tracknumber_long: string;
    tracklength: string;
    playlist_one: string;
    playlist_other: string;
    year: string;
    release: string;
    release_long: string;
    added: string;
    cover: string;
    emptylist: string;
    listisempty: string;
    newplaylist: string;
    mediapath: string;
    present: string;
    save: string;
    abort: string;
    yes: string;
    no: string;
    close: string;
    delete: string;
    system: string;
  };
  viewselect: {
    recents: string;
    tracks: string;
    albums: string;
    artists: string;
    composers: string;
    genres: string;
    playlists: string;
  };
  coloredit: {
    background: string;
    background_active: string;
    background_hover: string;
    background_button: string;
    border: string;
    accent: string;
    warn: string;
    text: string;
    text_dim: string;
    text_highlight: string;
    shadow: string;
  };
  contextmenu: {
    play: string;
    playrandom: string;
    playnext: string;
    addqueue: string;
    addplaylist: string;
    showartist: string;
    showgenre: string;
    showalbum: string;
    showcomposer: string;
    showplaylist: string;
    showlyrics: string;
    openpath: string;
    showsimilar: string;
    rating: string;
    info: string;
    edit: string;
    delete: string;
    extractcover: string;
  };
  edit: {
    cannotedit: string;
    editalbum: string;
    editartist: string;
    editcomposer: string;
    editgenre: string;
    editplaylist: string;
    newplaylist: string;
    edittrack: string;
    sort_album: string;
  };
  import: {
    importlibrary: string;
  };
  mediapath: {
    currentmediapath: string;
    draganddroppath: string;
    newmediapath: string;
    explain: string;
  };
  playlistselect: {
    namenewplaylist: string;
    createnewplaylist: string;
  };
  queue: {
    queue: string;
    history: string;
  };
  settings: {
    settings: string;
    general: string;
    playback: string;
    files: string;
    tools: string;
    theme: {
      theme: string;
      system: string;
      dark: string;
      light: string;
      custom: string;
      editcolors: string;
      resetlight: string;
      resetdark: string;
    };
    startupview: string;
    language: string;
    searchforupdates: string;
    mediapath: string;
    changemediapath: string;
    managefolders: string;
    managefolders_explain_active: string;
    managefolders_explain_inactive: string;
    copynotcopiedmedia: string;
    allowdelete: string;
    allowdelete_active: string;
    allowdelete_inactive: string;
    allowdeletefiles: string;
    allowdeletefiles_active: string;
    allowdeletefiles_inactive: string;
    extractcovers: string;
    extractcovers_explain: string;
    showmissingtracks: string;
    showmissingtracks_explain: string;
    standardisenames: string;
    standardisenames_explain: string;
    extractmetadata: string;
    extractmetadata_explain: string;
    support_please: string;
    updatesarehere: string;
  };
  setup: {
    setup: string;
    mediapath: string;
    mediapath_explain: string;
    mediapath_inactive_explain: string;
    searchforupdates_active_explain: string;
    searchforupdates_inactive_explain: string;
  };
  tutorial: {
    tutorial: string;
    ccmessage: string;
    attributions: string;
  };
  sections: {
    today: string;
    lastweek: string;
    lastmonth: string;
    lastthreemonths: string;
    lastsixmonths: string;
    thisyear: string;
  };
  times: {
    year_one: string;
    year_other: string;
    year_short: string;
    month_one: string;
    month_other: string;
    month_short: string;
    week_one: string;
    week_other: string;
    week_short: string;
    day_one: string;
    day_other: string;
    day_short: string;
    hour_one: string;
    hour_other: string;
    hour_short: string;
    minute_one: string;
    minute_other: string;
    minute_short: string;
    second_one: string;
    second_other: string;
    second_short: string;
  };
  centercontent: {
    coverextract_work: string;
    coverextract_done: string;
    libraryimport_work: string;
    libraryimport_done: string;
    fileimport_work: string;
    fileimport_done: string;
    delete_work: string;
    delete_done: string;
    updatetracks_work: string;
    updatetracks_done: string;
    updatealbum_work: string;
    updatealbum_done: string;
  };
  deletemodal: {
    albumsandtracks: string;
    artistandtracks: string;
    composerandtracks: string;
    genreandtracks: string;
    track: string;
    playlist: string;
    deleteincludingfiles: string;
  };
  notification: {
    libraryimport: {
      first: string;
      second: string;
      third: string;
    };
  };
  emptyview: {
    nothinghere: string;
    importlibrary: {
      first: string;
      second: string;
      third: string;
    };
    importfiles: {
      first: string;
      second: string;
      third: string;
    };
  };
  playlist: {
    cannotmodify1024: string;
  };
  llminfo: {
    long: string;
    short: string;
    notice: string;
  };
}

export interface BackendMessage {
  notification: Notification | null;
  error: string | null;
  warning: Warning | null;
  progress: Progress | null;
}

export enum Notification {
  LibraryImport = "LibraryImport",
  None = "None",
}

export enum Warning {
  None = "None",
}

export interface Progress {
  info: ProgressInfo;
  value: number | null;
  done: boolean;
}

export enum ProgressInfo {
  LibraryImport = "LibraryImport",
  FileImport = "FileImport",
  CoverExtract = "CoverExtract",
  Delete = "Delete",
  UpdateTracks = "UpdateTracks",
  UpdateAlbum = "UpdateAlbum",
  None = "None",
}

export interface Search {
  tracks: number[] | null;
  albums: number[] | null;
  genres: number[] | null;
  artists: number[] | null;
  composers: number[] | null;
  playlists: number[] | null;
}

export interface SpaceTime {
  space: number | null;
  time: number | null;
}

export interface Data {
  queue: number[] | null;
  search: Search | null;
  tracks: Track[] | null;
  albums: Album[] | null;
  artists: Artist[] | null;
  artist_albums: ArtistAlbums[] | null;
  artist_tracks: ArtistTracks[] | null;
  composers: Composer[] | null;
  composer_tracks: ComposerTracks[] | null;
  covers: Cover[] | null;
  genres: Genre[] | null;
  genre_tracks: GenreTracks[] | null;
  playlists: Playlist[] | null;
  spacetime: SpaceTime | null;
  order: Order[] | null;
  albums_order: [Order[], number[]] | null;
  artists_order: [Order[], number[]] | null;
  composers_order: [Order[], number[]] | null;
  genres_order: [Order[], number[]] | null;
  playlists_order: [Order[], number[]] | null;
  tracks_order: [Order[], number[]] | null;
}

export interface AppState {
  error: string | null;
  //settings: Settings | null;
}

export interface CD {
  index: number;
  tracks: Track[];
}
