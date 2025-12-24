import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  DataType,
  Order,
  RepeatMode,
  type Output,
  type Album,
  type Artist,
  type Data,
  type DBState,
  type Genre,
  type Playlist,
  type SpaceTime,
  type Track,
  type AudioState,
  type Cover,
  type Search,
  type Composer,
  ActiveView,
  type ConfigState,
  Language,
  View,
  Theme,
  type Version,
  type ThemeColors,
  type BackendMessage,
  Notification,
  ProgressInfo,
  type Progress,
} from "./defs";
import { translations } from "./localisation/localisation.svelte";

export const data = $state({
  queue: [] as number[],
  albums: new Map() as Map<number, Album>,
  albums_order: { order: [] as Order[], ids: [] as number[] },
  artists: new Map() as Map<number, Artist>,
  artists_order: { order: [] as Order[], ids: [] as number[] },
  artist_albums: new Map() as Map<number, number[]>,
  artist_tracks: new Map() as Map<number, number[]>,
  composers: new Map() as Map<number, Composer>,
  composers_order: { order: [] as Order[], ids: [] as number[] },
  composer_tracks: new Map() as Map<number, number[]>,
  covers: new Map() as Map<number, Cover>,
  genres: new Map() as Map<number, Genre>,
  genres_order: { order: [] as Order[], ids: [] as number[] },
  genre_tracks: new Map() as Map<number, number[]>,
  tracks: new Map() as Map<number, Track>,
  tracks_order: { order: [] as Order[], ids: [] as number[] },
  playlists: new Map() as Map<number, Playlist>,
  playlists_order: { order: [] as Order[], ids: [] as number[] },
  // TODO: some way to make it easier to find which section&row an album is in to jump to it?
  sections: [] as { title: string; albums: number[] }[],
  rows: [] as (string[] | number[])[],
  spacetime: { space: null, time: null } as SpaceTime,
  search: {} as Search,
});

export const db = $state({
  tracks_max: 0,
  albums_max: 0,
  artists_max: 0,
  genres_max: 0,
  playlists_max: 0,
  media_path: "",
});

export const audio_state = $state({
  is_playing: false,
  is_muted: false,
  volume: 100,
  output: { id: 0, name: "" } as Output,
  position: 0, // Current playback position (ms)
  shuffle_mode: false,
  repeat_mode: RepeatMode.RepeatNone,
  current_track: 0,
  queue: [] as number[],
  history: [] as number[],
});

// TODO: store config state in localstorage to retrieve on reopen
export const config_state = $state({
  version: {
    major: 0,
    minor: 0,
    patch: 0,
  } as Version,
  theme: Theme.Dark,
  custom_colors: {
    background: "",
    background_active: "",
    background_hover: "",
    background_button: "",
    border_color: "",
    accent_input: "",
    warn: "",
    text: "",
    text_dim: "",
    text_highlight: "",
    shadow: "",
  } as ThemeColors,
  startup_view: View.Recents,
  language: Language.System,
  look_for_updates: false,
  media_path: "",
  manage_folders: false,
  allow_delete_from_db: false,
  allow_delete_files: false,
  is_new: false,
});

export const custom_colors_backup = $state({
  background: "",
  background_active: "",
  background_hover: "",
  background_button: "",
  border_color: "",
  accent_input: "",
  warn: "",
  text: "",
  text_dim: "",
  text_highlight: "",
  shadow: "",
} as ThemeColors);

export const app_state = $state({
  max_columns_albums: 4,
  max_columns_tracks: 0,
  new_data: 0,
  new_order: 0,
  new_tracks: 0,
  new_albums: 0,
  new_artists: 0,
  new_composers: 0,
  new_genres: 0,
  new_playlists: 0,
  new_covers: 0,
  new_queue: 0,
  open_split: -1,
  resized: 0,
  scroll_position: 0,
  scroll_row_id: 0,
  scroll_album_id: 0,
  scroll_data_by_range: false,
  received_config: false,
  changed_language: 0,
});

export enum ContextAction {
  Play,
  PlayRandom,
  PlayNext,
  AddQueue,
  AddPlaylist,
  ShowArtist,
  ShowGenre,
  ShowAlbum,
  ShowComposer,
  ShowPlaylist,
  ShowLyrics, // TODO: not sure if I want this
  OpenPath,
  ShowSimilar, // TODO: smart feature, requires way more backend code
  Rating,
  Info,
  Edit,
  Delete,
  ExtractCover,
}

export enum ContextView {
  Queue,
  History,
  Search,
  None,
  Any,
}

export interface Context {
  id: number;
  type: DataType;
}

export const contextMenuState = $state({
  context: { id: 0, type: DataType.None } as Context,
  actions: [] as ContextAction[],
  position: { x: 0, y: 0 },
  visible: false,
});

export const queueModalState = $state({
  visible: false,
  position: 0,
  queue_hist: true,
});

export const searchModalState = $state({
  visible: false,
  datatypes: [
    DataType.Album,
    DataType.Artist,
    DataType.Composer,
    DataType.Genre,
    DataType.Playlist,
    DataType.Track,
  ],
  searchterm: "",
});

export const editModalState = $state({
  visible: false,
  id: 0,
  type: DataType.None,
});

export const importModalState = $state({
  visible: false,
  path: "",
});

export const mediaPathModalState = $state({
  visible: false,
  path: "",
});

export const playlistSelectModalState = $state({
  visible: false,
  track_ids: [] as number[],
});

export const settingsModalState = $state({
  visible: false,
});

export const customColorModalState = $state({
  visible: false,
});

export const deleteModalState = $state({
  visible: false,
  context: {
    type: DataType.None,
    id: 0,
  } as Context,
});

export const viewState = $state({
  view: ActiveView.Loading,
  order: [] as Order[],
  selected: {
    album: 0,
    artist: 0,
    composer: 0,
    genre: 0,
    playlist: 0,
  },
});

export const notificationState = $state({
  visible: false,
  notification: Notification.None,
});

export const progress = $state({
  active: false,
  data: {
    info: ProgressInfo.None,
    value: null,
    done: false,
  } as Progress,
});

export const tutorialModalState = $state({
  visible: false,
});

export function millisecondsToReadableString(milliseconds: number) {
  const second = 1000;
  const minute = 60 * second;
  const hour = 60 * minute;
  const day = 24 * hour;
  const week = 7 * day;
  const month = 30.44 * day; // Approximation based on average days per month
  const year = 365 * day; // Ignoring leap years

  let output = "";

  let years = Math.floor(milliseconds / year);
  milliseconds %= year;
  if (years > 0) {
    output += `${years}${translations.times.year_short} `;
  }

  let months = Math.floor(milliseconds / month);
  milliseconds %= month;
  if (months > 0) {
    output += `${months}${translations.times.month_short} `;
  }

  let weeks = Math.floor(milliseconds / week);
  milliseconds %= week;
  if (weeks > 0) {
    output += `${weeks}${translations.times.week_short} `;
  }

  let days = Math.floor(milliseconds / day);
  milliseconds %= day;
  if (days > 0) {
    output += `${days}${translations.times.day_short} `;
  }

  let hours = Math.floor(milliseconds / hour);
  milliseconds %= hour;
  if (hours > 0) {
    output += `${hours.toString().padStart(2, "0")}:`;
  }

  let minutes = Math.floor(milliseconds / minute);
  milliseconds %= minute;
  output += `${minutes.toString().padStart(2, "0")}:`;

  let seconds = Math.floor(milliseconds / second);
  output += `${seconds.toString().padStart(2, "0")}`;

  return output;
}

function generateSections() {
  // TODO: move this to backend, we only need generateRows in frontend.
  let currentDate = new Date();
  let currentYear = currentDate.getFullYear();
  let lastYear = currentYear;
  let sectionRange = -1;

  let sections = [];
  let currentSection = { title: "", albums: [] as number[] };

  data.albums_order.ids.forEach((album_id, index) => {
    let album = data.albums.get(album_id);

    if (album !== undefined) {
      let dateAdded = new Date(album.date_added);
      let yearAdded = dateAdded.getFullYear();
      let diffInDays =
        (currentDate.getTime() - dateAdded.getTime()) / (1000 * 3600 * 24);

      if (yearAdded == currentYear && diffInDays > sectionRange) {
        // We need to create a new section title
        // The title depends on the date_added of the current album
        if (diffInDays < 1) {
          if (sectionRange != -1) {
            sections.push(currentSection);
          }
          sectionRange = 1;
          currentSection = { title: "today", albums: [] };
        } else if (diffInDays < 7) {
          if (sectionRange != -1) {
            sections.push(currentSection);
          }
          sectionRange = 7;
          currentSection = {
            title: "lastweek",
            albums: [],
          };
        } else if (diffInDays < 30) {
          if (sectionRange != -1) {
            sections.push(currentSection);
          }
          sectionRange = 30;
          currentSection = {
            title: "lastmonth",
            albums: [],
          };
        } else if (diffInDays < 91) {
          if (sectionRange != -1) {
            sections.push(currentSection);
          }
          sectionRange = 91;
          currentSection = {
            title: "last3months",
            albums: [],
          };
        } else if (diffInDays < 182) {
          if (sectionRange != -1) {
            sections.push(currentSection);
          }
          sectionRange = 182;
          currentSection = {
            title: "last6months",
            albums: [],
          };
        } else if (diffInDays < 365) {
          if (sectionRange != -1) {
            sections.push(currentSection);
          }
          sectionRange = 365;
          currentSection = {
            title: "thisyear",
            albums: [],
          };
        }
      } else {
        if (yearAdded != lastYear) {
          if (sectionRange != -1) {
            sections.push(currentSection);
          }
          // We need to create a new section
          // use the album year
          lastYear = yearAdded;
          currentSection = { title: String(yearAdded), albums: [] };
        }
      }

      currentSection.albums.push(album.id);
    }
  });

  sections.push(currentSection);

  data.sections = sections;
}

function chunkArray(array: number[], size: number) {
  const result = [];
  for (let i = 0; i < array.length; i += size) {
    result.push(array.slice(i, i + size));
  }
  return result;
}

export function generateRows() {
  let rows: (string[] | number[])[] = [];

  for (let i = 0; i < data.sections.length; i++) {
    const section = data.sections[i];

    rows.push([section.title]);

    rows = rows.concat(
      chunkArray(section.albums, app_state.max_columns_albums)
    );
  }

  data.rows = rows;
}

export const tableHeaderState = $state({
  name: "",
  artist: "",
  albumArtist: "",
  album: "",
  genre: "",
  time: "",
  releaseDate: "",
  addedDate: "",
});

export function updateTableHeaderState() {
  if (viewState.order.includes(Order.ByName)) {
    tableHeaderState.name = "down";
  } else if (viewState.order.includes(Order.ByNameInverse)) {
    tableHeaderState.name = "up";
  } else {
    tableHeaderState.name = "";
  }

  if (viewState.order.includes(Order.ByArtist)) {
    tableHeaderState.artist = "down";
  } else if (viewState.order.includes(Order.ByArtistInverse)) {
    tableHeaderState.artist = "up";
  } else {
    tableHeaderState.artist = "";
  }

  if (viewState.order.includes(Order.ByAlbumArtist)) {
    tableHeaderState.albumArtist = "down";
  } else if (viewState.order.includes(Order.ByAlbumArtistInverse)) {
    tableHeaderState.albumArtist = "up";
  } else {
    tableHeaderState.albumArtist = "";
  }

  if (viewState.order.includes(Order.ByAlbum)) {
    tableHeaderState.album = "down";
  } else if (viewState.order.includes(Order.ByAlbumInverse)) {
    tableHeaderState.album = "up";
  } else {
    tableHeaderState.album = "";
  }

  if (viewState.order.includes(Order.ByGenre)) {
    tableHeaderState.genre = "down";
  } else if (viewState.order.includes(Order.ByGenreInverse)) {
    tableHeaderState.genre = "up";
  } else {
    tableHeaderState.genre = "";
  }

  if (viewState.order.includes(Order.ByTime)) {
    tableHeaderState.time = "down";
  } else if (viewState.order.includes(Order.ByTimeInverse)) {
    tableHeaderState.time = "up";
  } else {
    tableHeaderState.time = "";
  }

  if (viewState.order.includes(Order.ByReleaseDate)) {
    tableHeaderState.releaseDate = "down";
  } else if (viewState.order.includes(Order.ByReleaseDateInverse)) {
    tableHeaderState.releaseDate = "up";
  } else {
    tableHeaderState.releaseDate = "";
  }

  if (viewState.order.includes(Order.ByAddedDate)) {
    tableHeaderState.addedDate = "down";
  } else if (viewState.order.includes(Order.ByAddedDateInverse)) {
    tableHeaderState.addedDate = "up";
  } else {
    tableHeaderState.addedDate = "";
  }
}

export function translateLanguage(language: Language) {
  // Values based on this: https://en.wikipedia.org/wiki/IETF_language_tag
  switch (language) {
    case Language.System:
      return "System";
    case Language.Czech:
      return "Čeština";
    case Language.Danish:
      return "Dansk";
    case Language.German:
      return "Deutsch";
    case Language.Greek:
      return "Ελληνικά";
    case Language.English:
      return "English";
    case Language.Spanish:
      return "Español";
    case Language.French:
      return "Français";
    case Language.Italian:
      return "Italiano";
    case Language.Japanese:
      return "日本語";
    case Language.Korean:
      return "한국어";
    case Language.Luxembourgish:
      return "Lëtzebuergesch";
    case Language.Dutch:
      return "Nederlands";
    case Language.Polish:
      return "Polski";
    case Language.Portuguese:
      return "Português";
    case Language.Turkish:
      return "Türkçe";
    case Language.Chinese:
      return "中文";
  }
}

export function translateTheme(theme: Theme) {
  switch (theme) {
    case Theme.System:
      return translations.settings.theme.system;
    case Theme.Light:
      return translations.settings.theme.light;
    case Theme.Dark:
      return translations.settings.theme.dark;
    case Theme.Custom:
      return translations.settings.theme.custom;
  }
}

export function translateStartupView(view: View) {
  switch (view) {
    case View.Recents:
      return translations.viewselect.recents;
    case View.Tracks:
      return translations.viewselect.tracks;
    case View.Albums:
      return translations.viewselect.albums;
    case View.Artists:
      return translations.viewselect.artists;
    case View.Composers:
      return translations.viewselect.composers;
    case View.Genres:
      return translations.viewselect.genres;
    case View.Playlists:
      return translations.viewselect.playlists;
  }
}

export let requested_covers: number[] = [];

export function initState() {
  // Initialize listeners
  listen("config_state", (event) => {
    console.log("ConfigState: ", event.payload);
    let config = event.payload as ConfigState;

    config_state.allow_delete_files = config.allow_delete_files;
    config_state.allow_delete_from_db = config.allow_delete_from_db;
    config_state.custom_colors = config.custom_colors;
    config_state.is_new = config.is_new;
    config_state.language = config.language;
    config_state.look_for_updates = config.look_for_updates;
    config_state.manage_folders = config.manage_folders;
    config_state.media_path = config.media_path;
    config_state.startup_view = config.startup_view;
    config_state.theme = config.theme;
    config_state.version = config.version;

    custom_colors_backup.background = config.custom_colors.background;
    custom_colors_backup.background_active =
      config.custom_colors.background_active;
    custom_colors_backup.background_hover =
      config.custom_colors.background_hover;
    custom_colors_backup.background_button =
      config.custom_colors.background_button;
    custom_colors_backup.border_color = config.custom_colors.border_color;
    custom_colors_backup.accent_input = config.custom_colors.accent_input;
    custom_colors_backup.warn = config.custom_colors.warn;
    custom_colors_backup.text = config.custom_colors.text;
    custom_colors_backup.text_dim = config.custom_colors.text_dim;
    custom_colors_backup.text_highlight = config.custom_colors.text_highlight;
    custom_colors_backup.shadow = config.custom_colors.shadow;

    if (config_state.is_new) {
      viewState.view = ActiveView.First;
    } else if (!app_state.received_config) {
      app_state.received_config = true;

      viewState.view = ActiveView.Recents;
      switch (config_state.startup_view) {
        case View.Albums:
          viewState.view = ActiveView.Albums;
          break;
        case View.Artists:
          viewState.view = ActiveView.Artists;
          break;
        case View.Composers:
          viewState.view = ActiveView.Composers;
          break;
        case View.Genres:
          viewState.view = ActiveView.Genres;
          break;
        case View.Playlists:
          viewState.view = ActiveView.Playlists;
          break;
        case View.Recents:
          viewState.view = ActiveView.Recents;
          break;
        case View.Tracks:
          viewState.view = ActiveView.Tracks;
          break;

        default:
          viewState.view = ActiveView.Recents;
          break;
      }
    }
  });

  listen("data", (event) => {
    console.log("Data: ", event.payload);
    let data_msg = event.payload as Data;

    // Not ideal to have this here, but seems to work
    if (!app_state.received_config) {
      invoke("configrequest", { request: "Get" });
    }

    if (data_msg.queue != null) {
      data.queue = data_msg.queue;
      app_state.new_queue += 1;
    }

    if (data_msg.tracks != null) {
      console.log("got tracks: ", data_msg.tracks);
      data_msg.tracks.forEach((track) => {
        data.tracks.set(track.id, track);
      });
      app_state.scroll_data_by_range = false;
      app_state.new_tracks += 1;
    }

    if (data_msg.albums != null) {
      data_msg.albums.forEach((album) => {
        data.albums.set(album.id, album);
      });
      app_state.scroll_data_by_range = false;
      app_state.new_albums += 1;
    }

    if (data_msg.artists != null) {
      data_msg.artists.forEach((artist) => {
        data.artists.set(artist.id, artist);
      });
      app_state.scroll_data_by_range = false;
      app_state.new_artists += 1;
    }

    if (data_msg.composers != null) {
      data_msg.composers.forEach((composer) => {
        data.composers.set(composer.id, composer);
      });
      app_state.scroll_data_by_range = false;
      app_state.new_composers += 1;
    }

    if (data_msg.covers != null) {
      // TODO: implement access based discard of covers to keep frontend memory footprint lean
      let received_covers: number[] = [];
      data_msg.covers.forEach((cover) => {
        received_covers.push(cover.id);
        //requested_covers.push(cover.id);
        data.covers.set(cover.id, cover);
      });

      let unreceived_covers = [];
      for (let i = 0; i < requested_covers.length; i++) {
        if (!received_covers.includes(requested_covers[i])) {
          unreceived_covers.push(requested_covers[i]);
        }
      }
      requested_covers = unreceived_covers;
      app_state.new_covers += 1;
    }

    if (data_msg.genres != null) {
      data_msg.genres.forEach((genre) => {
        data.genres.set(genre.id, genre);
      });
      app_state.scroll_data_by_range = false;
      app_state.new_genres += 1;
    }

    if (data_msg.playlists != null) {
      data_msg.playlists.forEach((playlist) => {
        data.playlists.set(playlist.id, playlist);
      });
      app_state.scroll_data_by_range = false;
      app_state.new_playlists += 1;
    }

    if (data_msg.spacetime != null) {
      data.spacetime.time = data_msg.spacetime.time;
      data.spacetime.space = data_msg.spacetime.space;
    }

    if (data_msg.search != null) {
      data.search = data_msg.search;
    }

    if (data_msg.artist_albums != null) {
      data_msg.artist_albums.forEach((artist_albums) => {
        data.artist_albums.set(artist_albums.id, artist_albums.albums);
      });
    }

    if (data_msg.artist_tracks != null) {
      data_msg.artist_tracks.forEach((artist_tracks) => {
        data.artist_tracks.set(artist_tracks.id, artist_tracks.tracks);
      });
    }

    if (data_msg.composer_tracks != null) {
      data_msg.composer_tracks.forEach((composer_tracks) => {
        data.composer_tracks.set(composer_tracks.id, composer_tracks.tracks);
      });
    }

    if (data_msg.genre_tracks != null) {
      data_msg.genre_tracks.forEach((genre_tracks) => {
        data.genre_tracks.set(genre_tracks.id, genre_tracks.tracks);
      });
    }

    if (data_msg.albums_order != null) {
      data.albums_order = {
        order: data_msg.albums_order[0],
        ids: data_msg.albums_order[1],
      };

      if (data_msg.albums_order[0][0] === Order.ByAddedDateInverse) {
        generateSections();
        generateRows();
      }

      if (viewState.view == ActiveView.Albums) {
        viewState.order = data_msg.albums_order[0];
        updateTableHeaderState();
        app_state.new_order += 1;
        console.log("new order: ", app_state.new_order);
      }
    }

    if (data_msg.artists_order != null) {
      data.artists_order = {
        order: data_msg.artists_order[0],
        ids: data_msg.artists_order[1],
      };
      if (viewState.view == ActiveView.Artists) {
        viewState.order = data_msg.artists_order[0];
        updateTableHeaderState();
        app_state.new_order += 1;
        console.log("new order: ", app_state.new_order);
      }
    }

    if (data_msg.composers_order != null) {
      data.composers_order = {
        order: data_msg.composers_order[0],
        ids: data_msg.composers_order[1],
      };
      if (viewState.view == ActiveView.Composers) {
        viewState.order = data_msg.composers_order[0];
        updateTableHeaderState();
        app_state.new_order += 1;
        console.log("new order: ", app_state.new_order);
      }
    }

    if (data_msg.genres_order != null) {
      data.genres_order = {
        order: data_msg.genres_order[0],
        ids: data_msg.genres_order[1],
      };
      if (viewState.view == ActiveView.Genres) {
        viewState.order = data_msg.genres_order[0];
        updateTableHeaderState();
        app_state.new_order += 1;
        console.log("new order: ", app_state.new_order);
      }
    }

    if (data_msg.playlists_order != null) {
      data.playlists_order = {
        order: data_msg.playlists_order[0],
        ids: data_msg.playlists_order[1],
      };
      if (viewState.view == ActiveView.Playlists) {
        viewState.order = data_msg.playlists_order[0];
        updateTableHeaderState();
        app_state.new_order += 1;
        console.log("new order: ", app_state.new_order);
      }
    }

    if (data_msg.tracks_order != null) {
      data.tracks_order = {
        order: data_msg.tracks_order[0],
        ids: data_msg.tracks_order[1],
      };
      if (
        viewState.view == ActiveView.Tracks ||
        viewState.view == ActiveView.Composers ||
        viewState.view == ActiveView.Genres ||
        viewState.view == ActiveView.Playlists
      ) {
        viewState.order = data_msg.tracks_order[0];
        updateTableHeaderState();
        app_state.new_order += 1;
        console.log("new order: ", app_state.new_order);
      }
    }

    app_state.new_data += 1;
  });

  listen("db_state", (event) => {
    console.log("DBState: ", event.payload);
    let db_state_msg = event.payload as DBState;

    if (db_state_msg.albums_max) {
      db.albums_max = db_state_msg.albums_max;
    }

    if (db_state_msg.artists_max) {
      db.artists_max = db_state_msg.artists_max;
    }

    if (db_state_msg.genres_max) {
      db.genres_max = db_state_msg.genres_max;
    }

    if (db_state_msg.playlists_max) {
      db.playlists_max = db_state_msg.playlists_max;
    }

    if (db_state_msg.tracks_max) {
      db.tracks_max = db_state_msg.tracks_max;
    }

    if (db_state_msg.media_path) {
      db.media_path = db_state_msg.media_path;
    }
  });

  listen("audio_state", (event) => {
    let state = event.payload as AudioState;

    if (state.is_playing !== null) {
      audio_state.is_playing = state.is_playing;
    }

    if (state.is_muted !== null) {
      audio_state.is_muted = state.is_muted;
    }

    if (state.volume !== null && state.volume >= 0) {
      console.log("is volume: ", state);
      audio_state.volume = state.volume;
    }

    if (state.output !== null) {
      console.log("is output: ", state);
      audio_state.output = state.output;
    }

    if (state.position !== null) {
      audio_state.position = state.position;
    }

    if (state.shuffle_mode !== null) {
      //console.log("is shuffle_mode: ", state);
      audio_state.shuffle_mode = state.shuffle_mode;
    }

    if (state.repeat_mode !== null) {
      //console.log("is repeat_mode: ", state);
      audio_state.repeat_mode = state.repeat_mode;
    }

    if (state.current_track !== null) {
      console.log("is current_track: ", state);
      audio_state.current_track = state.current_track;
    }

    if (state.queue !== null) {
      console.log("is queue: ", state);
      audio_state.queue = state.queue;
    }

    if (state.history !== null) {
      console.log("is history: ", state);
      audio_state.history = state.history;
    }
  });

  listen("backend_message", (event) => {
    let message = event.payload as BackendMessage;
    console.log(message);

    if (message.notification !== null) {
      console.log(message.notification);
      notificationState.notification = message.notification;
      notificationState.visible = true;
    }

    if (message.error !== null) {
      console.log(message.error);
      // TODO: display error
    }

    if (message.progress !== null) {
      console.log(message.progress);
      progress.data = message.progress;
      progress.active = true;
    }
  });

  invoke("audiorequest", { request: "Init" });
  invoke("dbrequest", { request: "Init" });
  /**
   * IMPORTANT!
   * configrequest is fulfilled so fast that config_state listener does not capture it...
   * Instead fire configrequest upon first data.
   */
  //invoke("configrequest", { request: "Get" });
}
