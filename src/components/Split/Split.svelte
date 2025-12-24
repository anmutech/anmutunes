<script lang="ts">
  import {
    buildContextMenu,
    closeContextMenu,
    get_cover,
    play_by_id,
  } from "../../actions.svelte";
  import {
    DataType,
    type Album,
    type Artist,
    type Cover,
    type Genre,
  } from "../../defs";
  import Notes from "../../graphics/notes.svelte";
  import { data, app_state, ContextView } from "../../state.svelte";
  import CDs from "./CDs.svelte";

  let {
    album,
    track_ids, // restricts which tracks to render, if it contains track_ids
    className,
  }: { album: number; track_ids: number[] | undefined; className: string } =
    $props();

  // TODO: revert code, since we are not updating the split when album changes
  const albumData: Album | undefined = $derived.by(() => {
    // TODO: do we need emtpy album?
    if (album === 0) {
      let albumData = {
        id: 0,
        artist_id: 0,
        name: "",
        sort_album: "",
        genre_id: 0,
        year: 0,
        release_date: "",
        date_modified: "",
        date_added: "",
        tracks: track_ids,
        cover_id: 0,
      } as Album;
      return albumData;
    } else {
      return data.albums.get(album);
    }
  });
  const genre: Genre | undefined = $derived.by(() => {
    console.log(albumData);
    if (albumData) {
      console.log(data.genres.get(albumData.genre_id));
      return data.genres.get(albumData.genre_id);
    }
  });
  const artistData: Artist | undefined = $derived.by(() => {
    if (albumData) {
      if (album === 0 && track_ids) {
        let artist_id = data.tracks.get(track_ids[0])?.artist_id;
        return artist_id ? data.artists.get(artist_id) : undefined;
      }
      return data.artists.get(albumData.artist_id);
    }
    return undefined;
  });
  let coverData: Cover | undefined = $derived.by(() => {
    if (albumData) {
      return get_cover(albumData.cover_id);
      //return data.covers.get(albumData.cover_id);
    }
    return undefined;
  });

  $effect(() => {
    if (app_state.new_covers && albumData) {
      coverData = get_cover(albumData.cover_id);
      //coverData = data.covers.get(albumData.cover_id);
    }
  });

  /**
   * TODO:
   * Instead of passing track id to split track we need to pass the complete track.
   * This is required since we want to seperate by CD number,
   * only the track has data about which CD it is part of.
   *
   * Per CD we want a seperate section
   * Each CD section should use column-count.
   */

  // TODO: calculate main color of cover, need to adjust text color accordingly.
  const coverColor =
    className === "artists" ? "var(--background)" : "var(--background-hover)"; //"white"; //"aqua";

  function overlayplay() {
    play_by_id(album, DataType.Album);
  }

  function handleContextMenu(event: MouseEvent) {
    buildContextMenu(event, DataType.Album, album, ContextView.Any);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class={"split " + className}
  style={"background-color:" + coverColor}
  onclick={(e) => {
    // Click on recents main causes app_state.open_split = -1;
    // Need stopPropagation to prevent this.
    e.stopPropagation();
    closeContextMenu();
  }}
>
  <div class="data">
    <h3 class="title">
      {albumData && albumData.name !== "" ? albumData.name : " "}
    </h3>
    <h4 class="artist">
      {artistData && albumData?.name !== "" ? artistData.name : " "}
    </h4>
    {#if albumData}
      <h6 class="year">
        {genre?.name}
        {genre && genre.name !== "" && albumData.year !== 0 ? " - " : ""}
        {albumData.year !== 0 ? albumData.year : ""}
      </h6>
    {/if}
    {#if albumData?.tracks}
      <CDs
        track_ids={track_ids === undefined ? albumData.tracks : track_ids}
        {coverColor}
      />
    {/if}
  </div>
  <div class="cover" oncontextmenu={handleContextMenu}>
    {#if coverData}
      <img class="cover-img" src={coverData.data} alt="" />
    {:else}
      <Notes border={"none"} border_radius={"var(--radius-medium)"} />
    {/if}
    <div class="overlay-icon" onclick={overlayplay}>
      <svg width="100" height="100" viewBox="0 0 24 24">
        <circle cx="12" cy="12" r="12" fill="var(--background)" stroke="none"
        ></circle>
        <path d="M8 6v12l11-6z" fill="var(--text)" />
      </svg>
    </div>
  </div>
</div>

<style>
  .split {
    display: grid;
    width: 100%;
    grid-template-columns: auto 400px;
  }

  .albums {
    margin: 0;
  }

  .recents {
    margin: 2rem 0 1rem 0;
  }

  .data {
    width: 100%;
    padding: 2rem 0 2rem 2rem;
  }

  .title {
    margin: 0;
  }

  .artist {
    margin-top: 0;
    margin-bottom: 0;
  }

  .year {
    margin-top: 0;
  }

  .overlay-icon {
    position: absolute;
    top: calc(50% - 50px);
    left: calc(50% - 50px);
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

  .cover {
    position: relative;
    height: 400px;
    width: 400px;
    padding: 2rem;
    cursor: pointer;
  }

  .cover-img {
    width: 100%;
    height: 100%;
    border-radius: var(--radius-medium);
    background-color: var(--background-active);
  }
</style>
