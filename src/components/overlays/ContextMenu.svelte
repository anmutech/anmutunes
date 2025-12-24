<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { ActiveView, DataType } from "../../defs";
  import {
    app_state,
    ContextAction,
    contextMenuState,
    data,
    deleteModalState,
    editModalState,
    playlistSelectModalState,
    viewState,
  } from "../../state.svelte";
  import { play_by_id, play_random_by_id } from "../../actions.svelte";
  import { translations } from "../../localisation/localisation.svelte";

  let context_menu: HTMLDivElement | undefined = $state();

  let position = $derived.by(() => {
    if (context_menu) {
      const viewportWidth = window.innerWidth;
      const viewportHeight = window.innerHeight;

      let top = contextMenuState.position.y;
      let left = contextMenuState.position.x;

      let menuheight = context_menu.offsetHeight;
      let menuwidth = context_menu.offsetWidth;

      if (left + menuwidth > viewportWidth) {
        left -= menuwidth;
        if (left < 0) {
          left = 0;
        }
      }

      if (top + menuheight > viewportHeight) {
        top -= menuheight;
        if (top < 0) {
          top = 0;
        }
      }

      return "top: " + top + "px; left: " + left + "px;";
    }
  });

  function runAction(action: ContextAction) {
    switch (action) {
      case ContextAction.Play:
        play_by_id(contextMenuState.context.id, contextMenuState.context.type);
        break;
      case ContextAction.PlayRandom:
        play_random_by_id(
          contextMenuState.context.id,
          contextMenuState.context.type
        );
        break;
      case ContextAction.PlayNext:
        invoke("dbrequest", {
          request: {
            QueueInsert: [
              contextMenuState.context.type,
              [contextMenuState.context.id],
              0,
              viewState.order,
            ],
          },
        });
        break;
      case ContextAction.AddQueue:
        invoke("dbrequest", {
          request: {
            QueueInsert: [
              contextMenuState.context.type,
              [contextMenuState.context.id],
              null,
              viewState.order,
            ],
          },
        });
        break;
      case ContextAction.AddPlaylist:
        switch (contextMenuState.context.type) {
          case DataType.Album:
            let album_tracks = data.albums.get(
              contextMenuState.context.id
            )?.tracks;
            if (album_tracks) {
              playlistSelectModalState.track_ids = album_tracks;
              playlistSelectModalState.visible = true;
            }
            break;
          case DataType.Track:
            playlistSelectModalState.track_ids = [contextMenuState.context.id];
            playlistSelectModalState.visible = true;
            break;
          case DataType.Artist:
            let artist_tracks = data.artist_tracks.get(
              contextMenuState.context.id
            );
            if (artist_tracks) {
              playlistSelectModalState.track_ids = artist_tracks;
              playlistSelectModalState.visible = true;
            }
            break;
          case DataType.Composer:
            let composer_tracks = data.artist_tracks.get(
              contextMenuState.context.id
            );
            if (composer_tracks) {
              playlistSelectModalState.track_ids = composer_tracks;
              playlistSelectModalState.visible = true;
            }
            break;
          case DataType.Genre:
            let genre_tracks = data.genre_tracks.get(
              contextMenuState.context.id
            );
            if (genre_tracks) {
              playlistSelectModalState.track_ids = genre_tracks;
              playlistSelectModalState.visible = true;
            }
            break;
          case DataType.Video:
            break;
          case DataType.Playlist:
            let playlist = data.playlists.get(contextMenuState.context.id);
            if (playlist) {
              playlistSelectModalState.track_ids = playlist.tracks;
              playlistSelectModalState.visible = true;
            }
            break;
          default:
            break;
        }
        break;
      case ContextAction.Rating:
        break;
      case ContextAction.Info:
        break;
      case ContextAction.Edit:
        editModalState.visible = true;
        editModalState.id = contextMenuState.context.id;
        editModalState.type = contextMenuState.context.type;
        break;
      case ContextAction.Delete:
        deleteModalState.context = contextMenuState.context;
        deleteModalState.visible = true;
        break;
      case ContextAction.ShowArtist:
        switch (contextMenuState.context.type) {
          case DataType.Album:
            let album = data.albums.get(contextMenuState.context.id);
            if (album) {
              viewState.selected.artist = album.artist_id;
              viewState.view = ActiveView.Artists;
            }
            break;
          case DataType.Track:
            let track = data.tracks.get(contextMenuState.context.id);
            if (track) {
              viewState.selected.artist = track.artist_id;
              viewState.view = ActiveView.Artists;
            }
            break;
          default:
            break;
        }
        break;
      case ContextAction.ShowGenre:
        switch (contextMenuState.context.type) {
          case DataType.Album:
            let album = data.albums.get(contextMenuState.context.id);
            if (album) {
              if (album.genre_id !== 0) {
                viewState.selected.genre = album.genre_id;
                viewState.view = ActiveView.Genres;
              }
              // TODO: else show not found?
            }
            break;
          case DataType.Track:
            let track = data.tracks.get(contextMenuState.context.id);
            if (track) {
              if (track.genre_id !== 0) {
                viewState.selected.genre = track.genre_id;
                viewState.view = ActiveView.Genres;
              }
              // TODO: else show not found?
            }
            break;
          default:
            break;
        }
        break;
      case ContextAction.ShowAlbum:
        switch (contextMenuState.context.type) {
          case DataType.Album:
            viewState.selected.album = contextMenuState.context.id;
            app_state.open_split = contextMenuState.context.id;
            viewState.view = ActiveView.Albums;
            break;
          case DataType.Track:
            let track = data.tracks.get(contextMenuState.context.id);
            if (track) {
              if (track.album_id !== 0) {
                viewState.selected.album = track.album_id;
                app_state.open_split = track.album_id;
                viewState.view = ActiveView.Albums;
              }
              // TODO: else show not found?
            }
            break;
          default:
            break;
        }
        break;
      case ContextAction.ShowPlaylist:
        // Only relevant for search result
        if (contextMenuState.context.type === DataType.Playlist) {
          viewState.selected.playlist = contextMenuState.context.id;
          viewState.view = ActiveView.Playlists;
        }
        break;
      case ContextAction.ShowComposer:
        if (contextMenuState.context.type === DataType.Track) {
          let track = data.tracks.get(contextMenuState.context.id);
          // TODO: implement composer_id for track:
          // viewState.selected.composer = track.composer_id;
          // viewState.view = ActiveView.Composers;
        }
        break;
      case ContextAction.ShowLyrics:
        break;
      case ContextAction.OpenPath:
        if (contextMenuState.context.type === DataType.Track) {
          invoke("dbrequest", {
            request: {
              OpenContainingDir: [
                contextMenuState.context.type,
                contextMenuState.context.id,
              ],
            },
          });
        }
        break;
      case ContextAction.ShowSimilar:
        break;
      case ContextAction.ExtractCover:
        switch (contextMenuState.context.type) {
          case DataType.Album:
            invoke("dbrequest", {
              request: {
                ExtractCover: contextMenuState.context.id,
              },
            });
            break;
          default:
            break;
        }
        break;
      default:
        break;
    }
  }

  function get_action_name(action: ContextAction) {
    switch (action) {
      case ContextAction.Play:
        return translations.contextmenu.play;
      case ContextAction.PlayRandom:
        return translations.contextmenu.playrandom;
      case ContextAction.PlayNext:
        return translations.contextmenu.playnext;
      case ContextAction.AddQueue:
        return translations.contextmenu.addqueue;
      case ContextAction.AddPlaylist:
        return translations.contextmenu.addplaylist;
      case ContextAction.ShowArtist:
        return translations.contextmenu.showartist;
      case ContextAction.ShowGenre:
        return translations.contextmenu.showgenre;
      case ContextAction.ShowAlbum:
        return translations.contextmenu.showalbum;
      case ContextAction.ShowComposer:
        return translations.contextmenu.showcomposer;
      case ContextAction.ShowPlaylist:
        return translations.contextmenu.showplaylist;
      case ContextAction.ShowLyrics:
        return translations.contextmenu.showlyrics;
      case ContextAction.OpenPath:
        return translations.contextmenu.openpath;
      case ContextAction.ShowSimilar:
        return translations.contextmenu.showsimilar;
      case ContextAction.Rating:
        return translations.contextmenu.rating;
      case ContextAction.Info:
        return translations.contextmenu.info;
      case ContextAction.Edit:
        return translations.contextmenu.edit;
      case ContextAction.Delete:
        return translations.contextmenu.delete;
      case ContextAction.ExtractCover:
        return translations.contextmenu.extractcover;
      default:
        break;
    }
  }

  /**
   * TODO:
   * close on scroll?
   */
</script>

{#if contextMenuState.visible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="context-menu" style={position} bind:this={context_menu}>
    {#each contextMenuState.actions as action}
      <button
        class="context-menu-item"
        onclick={() => {
          runAction(action);
        }}
      >
        {get_action_name(action)}
      </button>
    {/each}
  </div>
{/if}

<style>
  .context-menu {
    position: absolute;
    display: grid;
    background-color: var(--background-button);
    border: var(--border);
    border-radius: var(--radius-small);
    max-height: 100%;
    z-index: 2; /* Make sure context is above everything */
    overflow: auto;
    box-shadow: var(--box-shadow);
  }

  .context-menu-item {
    padding: 6px 12px;
    justify-content: left;
  }

  .context-menu-item:hover {
    background-color: var(--background-hover);
  }
</style>
