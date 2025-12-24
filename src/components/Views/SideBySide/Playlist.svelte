<script lang="ts">
  import { DataType, type Playlist, type Track } from "../../../defs";
  import {
    app_state,
    ContextView,
    data,
    viewState,
  } from "../../../state.svelte";
  import { play_by_id, update_playlist } from "../../../actions.svelte";
  import Sortable from "../../Sortable/Sortable.svelte";
  import SvelteVirtualList from "@humanspeak/svelte-virtual-list";
  import SortableTrack from "../../Sortable/Track.svelte";
  import { translations } from "../../../localisation/localisation.svelte";

  let playlist = $derived(data.playlists.get(viewState.selected.playlist));
  let track_ids = $derived(playlist?.tracks);
  let tracks = $derived.by(() => {
    if (track_ids && track_ids.length > 1024) {
      let tracks = [];
      for (let i = 0; i < track_ids.length; i++) {
        let track = data.tracks.get(track_ids[i]);
        if (track) {
          tracks.push(track);
        }
      }
      return tracks;
    } else {
      return [];
    }
  });

  function finalize_callback(items: { id: number; track: Track }[]) {
    if (playlist) {
      // TODO: test if actually changed. Don't need to update if doesn't change.
      let new_playlist: Playlist = {
        id: playlist.id,
        name: playlist.name,
        description: "",
        tracks: [],
      };

      for (let i = 0; i < items.length; i++) {
        new_playlist.tracks.push(items[i].track.id);
      }

      // This causes a delay before the drop animation fulfills (on long playlists).
      // Should stay this way as feedback to the user when the new track order is stored.
      update_playlist(new_playlist);
    }
  }

  function remove_callback(item: { id: number; track: Track }) {
    if (playlist && track_ids) {
      let track_id = track_ids[item.id];
      if (track_id === item.track.id) {
        console.log("Found the item to remove");
        track_ids.splice(item.id, 1);
        let new_playlist: Playlist = {
          id: playlist.id,
          name: playlist.name,
          description: "",
          tracks: track_ids,
        };

        update_playlist(new_playlist);
      }
    }
  }

  function onclick_callback(item: { id: number; track: Track }) {
    play_by_id(item.track.id, DataType.Track);
  }

  // TODO: test what a reasonable limit looks like before loading a playlist takes too long
  // Maybe do chunking of 100 (1000?) or so tracks and put this into virtual scroll?
  // Best would be a proper implementation supporting sorting in a virtual scroll.
</script>

{#key app_state.new_tracks}
  <div class="grid">
    <div class="padded">
      <h2>{playlist?.name}</h2>
      {#if playlist?.description.length != 0}
        <p>{playlist?.description}</p>
      {/if}
      {#if track_ids && track_ids.length > 1024}
        <h5>
          {translations.playlist.cannotmodify1024}
        </h5>
      {/if}
    </div>
    {#if track_ids && track_ids.length < 1024}
      <div class="scrollable">
        <Sortable
          {track_ids}
          {finalize_callback}
          enable_playing={true}
          enable_remove={true}
          {remove_callback}
          {onclick_callback}
          contextview={ContextView.Any}
        />
      </div>
    {:else if track_ids}
      <div class="padded">
        <SvelteVirtualList items={tracks}>
          {#snippet renderItem(item, index)}
            <SortableTrack
              track={item}
              coverColor={"var(--background)"}
              enable_playing={true}
              enable_remove={true}
              remove_callback={() => {
                remove_callback({ id: index, track: item });
              }}
              onclick_callback={() => {
                onclick_callback({ id: index, track: item });
              }}
              contextview={ContextView.Any}
            />
          {/snippet}
        </SvelteVirtualList>
      </div>
    {/if}
  </div>
{/key}

<style>
  .grid {
    width: 100%;
    height: calc(100vh - (var(--full-header-height) + var(--footer-height)));
    display: grid;
    grid-template-rows: auto 1fr;
  }

  h2,
  h5 {
    margin: 0;
  }

  .scrollable {
    overflow: auto;
  }

  .padded {
    padding: 0.5rem;
  }
</style>
