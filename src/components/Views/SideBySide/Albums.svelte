<script lang="ts">
  import SvelteVirtualList from "@humanspeak/svelte-virtual-list";
  import { ActiveView, DataType } from "../../../defs";
  import { data, viewState } from "../../../state.svelte";
  import Split from "../../Split/Split.svelte";

  let { type, id } = $derived.by(() => {
    switch (viewState.view) {
      case ActiveView.Artists:
        return { type: DataType.Artist, id: viewState.selected.artist };
      default:
        return { type: DataType.None, id: 0 };
    }
  });

  let track_ids = $derived(data.artist_tracks.get(id));
  let album_ids: number[] = $derived.by(() => {
    let album_ids = data.artist_albums.get(id);
    if (album_ids) {
      return album_ids;
    } else {
      return [] as number[];
    }
  });

  let tracks_albums = $derived.by(() => {
    let tracks_albums: Map<number, number[]> = new Map();
    track_ids?.forEach((track_id) => {
      let track = data.tracks.get(track_id);
      if (track && !album_ids.includes(track.album_id)) {
        album_ids.push(track.album_id);
        if (!tracks_albums.get(track.album_id)) {
          tracks_albums.set(track.album_id, [track_id]);
        } else {
          tracks_albums.get(track.album_id)?.push(track_id);
        }
      }
    });

    return tracks_albums;
  });
  /**
   * TODO:
   * get tracks and albums for artist.
   * For each track we need to test which album it is part of
   * Then sort them into albums.
   * For albums we do not need to do anything.
   *
   * Make sure albums are sorted by release date, if possible...
   */
</script>

{#key tracks_albums}
  <div class="grid">
    <div class="info">
      <h2>{data.artists.get(id)?.name}</h2>
    </div>
    <SvelteVirtualList items={album_ids}>
      {#snippet renderItem(item)}
        <Split
          album={item}
          track_ids={tracks_albums.get(item)}
          className={"artists"}
        ></Split>
      {/snippet}
    </SvelteVirtualList>
  </div>
{/key}

<style>
  .grid {
    width: 100%;
    height: calc(100vh - (var(--full-header-height) + var(--footer-height)));
    display: grid;
    grid-template-rows: auto 1fr;
  }

  h2 {
    margin: 0;
  }

  .info {
    padding: 0.5rem 2rem;
  }
</style>
