<script lang="ts">
  import {
    app_state,
    ContextView,
    data,
    searchModalState,
    viewState,
  } from "../../state.svelte";
  import { ActiveView, DataType } from "../../defs";
  import { buildContextMenu, play_by_id } from "../../actions.svelte";

  let { id, type }: { id: number; type: DataType } = $props();

  function click() {
    switch (type) {
      case DataType.Track:
        play_by_id(id, DataType.Track);
        break;
      case DataType.Album:
        // TODO: jump to album in album view or recents view
        viewState.selected.album = id;
        app_state.open_split = id;
        viewState.view = ActiveView.Recents;
        //play_by_id(id, DataType.Album);
        break;
      case DataType.Artist:
        // TODO: jump to artist in artist view
        viewState.selected.artist = id;
        viewState.view = ActiveView.Artists;
        break;
      case DataType.Composer:
        // TODO: jump to composer in composer view
        viewState.selected.composer = id;
        viewState.view = ActiveView.Composers;
        break;
      case DataType.Genre:
        // TODO: jump to genre in genre view
        viewState.selected.genre = id;
        viewState.view = ActiveView.Genres;
        break;
      case DataType.Video:
        // TODO: implement view?
        break;
      case DataType.Playlist:
        // TODO: jump to playlist in playlist view
        viewState.selected.playlist = id;
        viewState.view = ActiveView.Playlists;
        //play_by_id(id, DataType.Playlist);
        break;
      default:
        break;
    }
    searchModalState.visible = false;
  }

  function context(event: MouseEvent) {
    buildContextMenu(event, type, id, ContextView.Search);
  }
</script>

<button onclick={click} oncontextmenu={context}>
  {#if type == DataType.Track}
    {data.tracks.get(id)?.name}
  {:else if type == DataType.Album}
    {data.albums.get(id)?.name}
  {:else if type == DataType.Artist}
    {data.artists.get(id)?.name}
  {:else if type == DataType.Composer}
    {data.composers.get(id)?.name}
  {:else if type == DataType.Genre}
    {data.genres.get(id)?.name}
  {:else if type == DataType.Playlist}
    {data.playlists.get(id)?.name}
  {/if}
</button>

<style>
  button {
    width: 100%;
    justify-content: left;
  }
</style>
