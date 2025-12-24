import { invoke } from "@tauri-apps/api/core";
import {
  ActiveView,
  DataType,
  Order,
  Theme,
  type Album,
  type Artist,
  type Composer,
  type Genre,
  type Playlist,
  type Track,
} from "./defs";
import {
  app_state,
  audio_state,
  config_state,
  ContextAction,
  contextMenuState,
  ContextView,
  customColorModalState,
  data,
  deleteModalState,
  editModalState,
  importModalState,
  mediaPathModalState,
  playlistSelectModalState,
  queueModalState,
  requested_covers,
  searchModalState,
  settingsModalState,
  viewState,
} from "./state.svelte";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function play_by_id(id: number, type: DataType) {
  invoke("dbrequest", {
    request: {
      Play: [type, [id], null],
    },
  });
}

export function play_random_by_id(id: number, type: DataType) {
  invoke("audiorequest", { request: { Shuffle: true } });
  invoke("dbrequest", {
    request: {
      Play: [type, [id], null],
    },
  });
}

export function play_by_ids(ids: [number], type: DataType) {
  invoke("dbrequest", {
    request: {
      Play: [type, ids, null],
    },
  });
}

export function play_random_by_ids(ids: [number], type: DataType) {
  invoke("audiorequest", { request: { Shuffle: true } });
  invoke("dbrequest", {
    request: {
      Play: [type, ids, null],
    },
  });
}

export function get_data_order(type: DataType, order: Order[]) {
  if (order.length == 0) {
    switch (viewState.view) {
      case ActiveView.Recents:
        order = [Order.ByAddedDateInverse];
        break;
      default:
        order = [Order.ByName];
        break;
    }
  }
  invoke("dbrequest", {
    request: {
      GetDataOrder: [type, order],
    },
  });
}

export function update_tracks(
  tracks: [Track],
  artistNames: [string],
  albumNames: [string],
  genreNames: [string]
) {
  invoke("dbrequest", {
    request: {
      UpdateTracks: [tracks, artistNames, albumNames, genreNames],
    },
  });
}

export function update_album(
  album: Album,
  artistName: string,
  genreName: string
) {
  invoke("dbrequest", {
    request: {
      UpdateAlbum: [album, artistName, genreName],
    },
  });
}

export function update_artist(artist: Artist) {
  invoke("dbrequest", {
    request: {
      UpdateArtist: artist,
    },
  });
}

export function update_composer(composer: Composer) {
  invoke("dbrequest", {
    request: {
      UpdateComposer: composer,
    },
  });
}

export function update_genre(genre: Genre) {
  invoke("dbrequest", {
    request: {
      UpdateGenre: genre,
    },
  });
}

export function update_playlist(playlist: Playlist) {
  invoke("dbrequest", {
    request: {
      UpdatePlaylist: playlist,
    },
  });
}

export function create_playlist(playlist: Playlist) {
  invoke("dbrequest", {
    request: {
      NewPlaylist: playlist,
    },
  });
}

export function update_queue(queue: number[]) {
  invoke("audiorequest", {
    request: {
      QueueMove: queue,
    },
  });
}

export function get_cover(cover_id: number) {
  // TODO: create timeout before calling backend to load multiple covers at once?
  let cover = data.covers.get(cover_id);

  if (cover) {
    return cover;
  } else if (cover_id !== 0 && !requested_covers.includes(cover_id)) {
    requested_covers.push(cover_id);
    invoke("dbrequest", {
      request: {
        GetCoversById: [cover_id],
      },
    });
  }
}

export function reset_custom_colors_light() {
  config_state.custom_colors = {
    background: "#ffffff",
    background_active: "#eae1e1",
    background_hover: "#efe8e8",
    background_button: "#f7f3f3",
    border_color: "#d7d1d1",
    accent_input: "#d7d1d1",
    warn: "#aa0000",
    text: "#222222",
    text_dim: "#787878",
    text_highlight: "#3b3b3b",
    shadow: "#00000020",
  };
  set_custom_colors();
  set_config();
}

export function reset_custom_colors_dark() {
  config_state.custom_colors = {
    background: "#2b2b2b",
    background_active: "#222222",
    background_hover: "#1d1d1d",
    background_button: "#363636",
    border_color: "#161616",
    accent_input: "#161616",
    warn: "#aa0000",
    text: "#ffffff",
    text_dim: "#787878",
    text_highlight: "#d7d7d7",
    shadow: "#00000030",
  };
  set_custom_colors();
  set_config();
}

export function set_custom_colors() {
  let styleSheet = document.styleSheets[0];
  for (let i = 0; i < styleSheet.cssRules.length; i++) {
    const ruleselector = styleSheet.cssRules[i] as CSSStyleRule;
    if (ruleselector.selectorText === '[data-theme="custom"]') {
      ruleselector.style.setProperty(
        "--background",
        config_state.custom_colors.background
      );
      ruleselector.style.setProperty(
        "--background-active",
        config_state.custom_colors.background_active
      );
      ruleselector.style.setProperty(
        "--background-hover",
        config_state.custom_colors.background_hover
      );
      ruleselector.style.setProperty(
        "--background-button",
        config_state.custom_colors.background_button
      );
      ruleselector.style.setProperty(
        "--border-color",
        config_state.custom_colors.border_color
      );
      ruleselector.style.setProperty(
        "--accent-input",
        config_state.custom_colors.accent_input
      );
      ruleselector.style.setProperty("--warn", config_state.custom_colors.warn);
      ruleselector.style.setProperty("--text", config_state.custom_colors.text);
      ruleselector.style.setProperty(
        "--text-dim",
        config_state.custom_colors.text_dim
      );
      ruleselector.style.setProperty(
        "--text-highlight",
        config_state.custom_colors.text_highlight
      );
      ruleselector.style.setProperty(
        "--shadow",
        config_state.custom_colors.shadow
      );
    }
  }
}

export function toggleOrder(up: Order, down: Order, type: DataType) {
  let up_index = viewState.order.indexOf(up);
  let down_index = viewState.order.indexOf(down);

  if (up_index != -1) {
    get_data_order(type, [down]);
  } else if (down_index != -1) {
    get_data_order(type, [up]);
  } else {
    get_data_order(type, [up]);
  }
}

export function toggleSplit(album_id: number) {
  if (album_id == app_state.open_split) {
    app_state.open_split = -1;
  } else {
    app_state.open_split = album_id;
  }
}

export function closeContextMenu() {
  contextMenuState.visible = false;
}

export function buildContextMenu(
  event: MouseEvent,
  type: DataType,
  id: number,
  view: ContextView
) {
  event.preventDefault();

  // TODO: depending on contextview exclude or include actions

  switch (type) {
    case DataType.Album:
      contextMenuState.actions = [
        ContextAction.Play,
        ContextAction.PlayRandom,
        ContextAction.PlayNext,
        ContextAction.AddQueue,
        ContextAction.AddPlaylist,
        ContextAction.Edit,
        ContextAction.ShowArtist,
        ContextAction.ShowAlbum,
        ContextAction.ShowGenre,
        ContextAction.ExtractCover,
      ];
      break;
    case DataType.Track:
      contextMenuState.actions = [
        ContextAction.Play,
        ContextAction.PlayNext,
        ContextAction.AddQueue,
        ContextAction.AddPlaylist,
        ContextAction.Edit,
        ContextAction.OpenPath,
        ContextAction.ShowArtist,
        ContextAction.ShowComposer,
        ContextAction.ShowAlbum,
        ContextAction.ShowGenre,
      ];
      break;
    case DataType.Playlist:
      contextMenuState.actions = [
        ContextAction.Play,
        ContextAction.PlayRandom,
        ContextAction.PlayNext,
        ContextAction.AddQueue,
        ContextAction.AddPlaylist,
        ContextAction.Edit,
        ContextAction.ShowPlaylist,
      ];
      break;
    case DataType.Artist:
      contextMenuState.actions = [
        ContextAction.Play,
        ContextAction.PlayRandom,
        ContextAction.PlayNext,
        ContextAction.AddQueue,
        ContextAction.AddPlaylist,
        ContextAction.Edit,
      ];
      break;
    case DataType.Composer:
      contextMenuState.actions = [
        ContextAction.Play,
        ContextAction.PlayRandom,
        ContextAction.PlayNext,
        ContextAction.AddQueue,
        ContextAction.AddPlaylist,
        ContextAction.Edit,
      ];
      break;
    case DataType.Genre:
      contextMenuState.actions = [
        ContextAction.Play,
        ContextAction.PlayRandom,
        ContextAction.PlayNext,
        ContextAction.AddQueue,
        ContextAction.AddPlaylist,
        ContextAction.Edit,
      ];
      break;
    default:
      break;
  }

  if (config_state.allow_delete_from_db) {
    contextMenuState.actions.push(ContextAction.Delete);
  }

  contextMenuState.context = { id, type };
  contextMenuState.position = { x: event.clientX, y: event.clientY };
  contextMenuState.visible = true;
}

const appWindow = getCurrentWindow();

async function toggleMaximize() {
  if (await appWindow.isMaximized()) {
    console.log("unmax");
    appWindow.unmaximize();
  } else {
    console.log("max");
    appWindow.maximize();
  }
}

/*
  TODO: currently the theme promise result is "light" even when gnome is dark.
  import { getCurrentWindow } from "@tauri-apps/api/window";
  const theme = getCurrentWindow().theme();
  console.log(theme);
 */
appWindow.onThemeChanged(({ payload: theme }) => {
  if (config_state.theme === Theme.System) {
    // Possible values for theme: 'light' | 'dark'
    document.documentElement.setAttribute("data-theme", theme);
  }
});

export async function changeSystemTheme() {
  let theme = await getCurrentWindow().theme();
  document.documentElement.setAttribute(
    "data-theme",
    theme != null ? theme : "dark"
  );
}

export function set_config() {
  invoke("configrequest", { request: { Set: config_state } });
}

document.addEventListener("keydown", (event) => {
  /*
  See here for reference:
  https://support.apple.com/de-de/guide/itunes/itns1019/windows
  https://support.apple.com/de-de/guide/itunes/itns1019/12.13/windows/10

  // Arrows: "ArrowRight", "ArrowLeft", "ArrowUp", or "ArrowDown"
  */

  let text_input_visible = false;

  if (
    editModalState.visible ||
    playlistSelectModalState.visible ||
    searchModalState.visible
  ) {
    text_input_visible = true;
  }

  if (event.ctrlKey || event.metaKey) {
    // Quit app on Ctrl/Command + Q
    if (event.key === "q") {
      appWindow.close();
    }
    // Future: close window, minimize to tray
    if (event.key === "w") {
      appWindow.close();
    }
    // Open search on Ctrl/Command + F
    if (event.key === "f") {
      searchModalState.visible = !searchModalState.visible;
    }

    // Change volume by 10
    // I think vlc supports higher than 100, be we stick to 0-100.
    if (event.key === "ArrowUp") {
      let volume = audio_state.volume > 90 ? 100 : audio_state.volume + 10;
      invoke("audiorequest", { request: { Volume: volume } });
    }
    if (event.key === "ArrowDown" && !event.shiftKey) {
      let volume = audio_state.volume > 10 ? audio_state.volume - 10 : 0;
      invoke("audiorequest", { request: { Volume: volume } });
    }

    // New playlist
    if (event.key === "n") {
      editModalState.type = DataType.Playlist;
      editModalState.id = 0;
      editModalState.visible = true;
    }

    // TODO: Edit behaviour?
    if (event.key === "z") {
      // TODO: restore previous state in edit modal?
    }

    if (event.key === "t") {
      // TODO: implement visualizer
    }

    if (event.key === ",") {
      // TODO: implement settings
    }
  }

  if (event.shiftKey && !text_input_visible) {
    if (event.ctrlKey || event.metaKey) {
      // TODO: instead of fixed value, increase for successive presses.
      if (event.key === "ArrowLeft") {
        let position =
          audio_state.position < 10000 ? 0 : audio_state.position - 10000;
        invoke("audiorequest", { request: { Seek: position } });
      } else if (event.key === "ArrowRight") {
        // TODO: use current track.total_time to bound position?
        let position = audio_state.position + 10000;
        invoke("audiorequest", { request: { Seek: position } });
      }
      if (event.key === "ArrowDown") {
        // Silence
        invoke("audiorequest", { request: { Volume: 0 } });
      }
      // TODO: f and m not working for some reason...
      if (event.key === "f") {
        console.log("maximize?");
        toggleMaximize();
      }
      if (event.key === "m") {
        // TODO: implement mini player
        console.log("toggle mini player");
      }
    } else {
      if (event.key === "ArrowLeft") {
        invoke("audiorequest", { request: "Prev" });
      } else if (event.key === "ArrowRight") {
        invoke("audiorequest", { request: "Next" });
      }
    }
  }

  if (event.key === "Escape") {
    // TODO: replace by capturing Escape in every modal, which automatically has correct hierarchy
    // Respect ordering, what can be on top of what
    if (contextMenuState.visible) {
      contextMenuState.visible = false;
    } else if (mediaPathModalState.visible) {
      mediaPathModalState.visible = false;
    } else if (customColorModalState.visible) {
      customColorModalState.visible = false;
    } else {
      editModalState.visible = false;
      importModalState.visible = false;
      playlistSelectModalState.visible = false;
      queueModalState.visible = false;
      searchModalState.visible = false;
      settingsModalState.visible = false;
      deleteModalState.visible = false;
    }
  }

  // Change audio play state on spacebar
  if (event.key === " " && !text_input_visible) {
    event.preventDefault(); // prevent scrolling
    invoke("audiorequest", {
      request: { PlayPause: !audio_state.is_playing },
    });
  }
});
