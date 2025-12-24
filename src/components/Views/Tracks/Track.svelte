<script lang="ts">
  import { buildContextMenu, play_by_id } from "../../../actions.svelte";
  import { DataType, type Album, type Artist, type Genre } from "../../../defs";
  import {
    ContextView,
    data,
    millisecondsToReadableString,
  } from "../../../state.svelte";

  let { track_id } = $props();

  let track = $derived(data.tracks.get(track_id));
  let artist: Artist | undefined = $derived.by(() => {
    if (track) {
      return data.artists.get(track.artist_id);
    }
  });
  let album_artist: Artist | undefined = $derived.by(() => {
    if (track) {
      return data.artists.get(track.album_artist_id);
    }
  });
  let album: Album | undefined = $derived.by(() => {
    if (track) {
      return data.albums.get(track.album_id);
    }
  });
  let genre: Genre | undefined = $derived.by(() => {
    if (track) {
      return data.genres.get(track.genre_id);
    }
  });
  let time = $derived.by(() => {
    if (track) {
      return millisecondsToReadableString(track.total_time);
    }
    return "";
  });

  function play() {
    play_by_id(track_id, DataType.Track);
  }

  function handleContextMenu(event: MouseEvent) {
    buildContextMenu(event, DataType.Track, track_id, ContextView.Any);
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  class="table-row track-row"
  onclick={play}
  oncontextmenu={handleContextMenu}
>
  <div class="table-data name">{track?.name}</div>
  <div class="table-data track-num">{track?.track_number}</div>
  <div class="table-data disc-num">{track?.disc_number}</div>
  <div class="table-data artist">{artist?.name}</div>
  <div class="table-data album-artist">{album_artist?.name}</div>
  <div class="table-data album">{album?.name}</div>
  <div class="table-data genre">{genre?.name}</div>
  <div class="table-data time">{time}</div>
</div>

<style>
  .track-row {
    display: grid;
    grid-template-columns: 2fr 70px 50px 1fr 1fr 1fr 1fr 90px;
    width: 100%;
    cursor: pointer;
  }

  .track-num {
    text-align: right;
  }

  .disc-num {
    text-align: right;
  }

  .time {
    text-align: right;
  }
</style>
