<script lang="ts">
  import { app_state, data, ContextView } from "../../../state.svelte";
  import { type Artist, type Album, type Cover, DataType } from "../../../defs";
  import {
    buildContextMenu,
    closeContextMenu,
    get_cover,
    play_by_id,
    toggleSplit,
  } from "../../../actions.svelte";
  import { onDestroy } from "svelte";
  import Notes from "../../../graphics/notes.svelte";

  function handleContextMenu(event: MouseEvent) {
    buildContextMenu(event, DataType.Album, album_id, ContextView.Any);
  }

  let { album_id }: { album_id: number } = $props();
  let album_container: HTMLDivElement;

  let active = $derived.by(() => {
    return app_state.open_split == album_id ? "active" : "";
  });

  function overlayplay(e: Event) {
    // Click on album causes toggleSplit(album_id);
    // Need stopPropagation to prevent this.
    e.stopPropagation();
    closeContextMenu();
    play_by_id(album_id, DataType.Album);
  }

  let albumData: Album | undefined = data.albums.get(album_id);
  let artistData: Artist | undefined = $state();
  let coverData: Cover | undefined = $state();
  let coverTimeout: number | undefined;

  // TODO: go through all components and use $effect where it makes sense.
  $effect(() => {
    if (albumData) {
      artistData = data.artists.get(albumData.artist_id);
      coverTimeout = setTimeout(() => {
        coverData = get_cover(albumData.cover_id);
        //coverData = data.covers.get(albumData.cover_id);
      }, 100);
    }
  });

  $effect(() => {
    if (app_state.new_covers && albumData) {
      coverTimeout = setTimeout(() => {
        coverData = get_cover(albumData.cover_id);
        //coverData = data.covers.get(albumData.cover_id);
      }, 100);
    }
  });

  onDestroy(() => {
    clearTimeout(coverTimeout);
  });
</script>

<div bind:this={album_container} class="album-container">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class={"album " + active}
    id={"album" + String(album_id)}
    onclick={(e) => {
      // Click on recents main causes app_state.open_split = -1;
      // Need stopPropagation to prevent this.
      e.stopPropagation();
      closeContextMenu();
      toggleSplit(album_id);
    }}
    oncontextmenu={handleContextMenu}
  >
    <div class="album-cover">
      {#if coverData}
        <img src={coverData.data} alt="" />
      {:else}
        <Notes border={"var(--border)"} border_radius={"var(--radius-small)"} />
      {/if}
      <div class="overlay-icon" onclick={overlayplay}>
        <svg width="50" height="50" viewBox="0 0 24 24">
          <circle cx="12" cy="12" r="12" fill="var(--background)" stroke="none"
          ></circle>
          <path d="M8 6v12l11-6z" fill="var(--text)" />
        </svg>
      </div>
    </div>
    <div class="album-info">
      <h4>{albumData?.name}</h4>
      <h5>{artistData?.name}</h5>
    </div>
  </div>
</div>

<style>
  .album-container {
    padding: 0 1rem;
  }

  .album {
    cursor: pointer;
    position: relative;
  }

  .active::after {
    content: "";
    position: absolute;
    bottom: -32px;
    left: 50%;
    transform: translateX(-50%);
    border-left: 15px solid transparent;
    border-right: 15px solid transparent;
    border-bottom: 20px solid var(--background-hover); /* TODO: use calculated album cover color like in split*/
  }

  .album-cover {
    position: relative;
    width: 100%;
    aspect-ratio: 1;
    overflow: hidden;
  }

  .overlay-icon {
    position: absolute;
    top: calc(50% - 25px);
    left: calc(50% - 25px);
    display: flex;
    justify-content: center;
    align-items: center;
    opacity: 0;
    transition: opacity 0.1s ease-in-out;
  }

  .overlay-icon:hover {
    opacity: 1;
  }

  .overlay-icon svg {
    fill: white;
  }

  .album-cover img {
    width: 100%;
    height: 100%;
    border: var(--border);
    border-radius: var(--radius-small);
    background-color: var(--background-active);
  }

  .album-info {
    margin-top: 10px;
  }

  .album-info h4,
  .album-info h5 {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin: 0;
    padding: 0;
  }
</style>
