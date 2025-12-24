<script lang="ts">
  import { onDestroy } from "svelte";
  import {
    buildContextMenu,
    get_cover,
    toggleSplit,
  } from "../../../actions.svelte";
  import { DataType, type Artist, type Cover, type Genre } from "../../../defs";
  import { app_state, ContextView, data } from "../../../state.svelte";
  import Notes from "../../../graphics/notes.svelte";

  let { album_id }: { album_id: number } = $props();

  let album = $derived(data.albums.get(album_id));
  let cover: Cover | undefined = $state();
  let coverTimeout: number | undefined;

  $effect(() => {
    if (album) {
      coverTimeout = setTimeout(() => {
        cover = get_cover(album.cover_id);
        //cover = data.covers.get(album.cover_id);
      }, 100);
    }
  });

  $effect(() => {
    if (app_state.new_covers && album) {
      coverTimeout = setTimeout(() => {
        cover = get_cover(album.cover_id);
        //cover = data.covers.get(album.cover_id);
      }, 100);
    }
  });

  let album_artist: Artist | undefined = $derived.by(() => {
    if (album) {
      return data.artists.get(album.artist_id);
    }
  });
  let genre: Genre | undefined = $derived.by(() => {
    if (album) {
      return data.genres.get(album.genre_id);
    }
  });

  function handleContextMenu(event: MouseEvent) {
    buildContextMenu(event, DataType.Album, album_id, ContextView.Any);
  }

  onDestroy(() => {
    clearTimeout(coverTimeout);
  });

  /**
   * TODO:
   * scroll into view on open split similar to recents view.
   */
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="table-row album-row"
  onclick={() => {
    toggleSplit(album_id);
  }}
  oncontextmenu={handleContextMenu}
>
  <div class="table-data cover">
    {#if cover}
      <img src={cover.data} alt="" />
    {:else}
      <Notes border={"var(--border)"} border_radius={"var(--radius-tiny)"} />
    {/if}
  </div>
  <div class="table-data name">{album?.name}</div>
  <div class="table-data album-artist">{album_artist?.name}</div>
  <div class="table-data genre">{genre?.name}</div>
  <div class="table-data release-date">{album?.release_date}</div>
  <div class="table-data added-date">{album?.date_added}</div>
  <div class="table-data num-tracks">{album?.tracks.length}</div>
</div>

<style>
  .album-row {
    display: grid;
    grid-template-columns: 70px 1.5fr 1fr 1fr 1fr 1fr 90px;
    width: 100%;
    cursor: pointer;
    height: 60px;
  }

  .num-tracks {
    text-align: right;
  }

  .cover img {
    width: 100%;
    aspect-ratio: 1;
    border: var(--border);
    border-radius: var(--radius-tiny);
    background-color: var(--background-active);
  }
</style>
