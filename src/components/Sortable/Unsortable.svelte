<script lang="ts">
  import { type Track } from "../../defs";
  import { ContextView, data } from "../../state.svelte";
  import SortableTrack from "./Track.svelte";
  // TODO: some way to get virtual scroll compatible with dnd?

  let {
    track_ids,
    enable_playing,
    onclick_callback,
    contextview,
  }: {
    track_ids: number[];
    enable_playing: boolean;
    onclick_callback: (item: { id: number; track: Track }) => void;
    contextview: ContextView;
  } = $props();

  let items = $derived.by(() => {
    let items = [];
    if (track_ids) {
      for (let i = 0; i < track_ids.length; i++) {
        const track_id = track_ids[i];
        let track = data.tracks.get(track_id);
        if (track) {
          items.push({ id: i, track: track });
        }
      }
    }
    return items;
  });
</script>

{#if track_ids}
  <div class="dropzone">
    {#each items as item (item.id)}
      <div>
        <SortableTrack
          track={item.track}
          coverColor={"var(--background)"}
          {enable_playing}
          enable_remove={false}
          remove_callback={() => {}}
          onclick_callback={() => {
            onclick_callback(item);
          }}
          {contextview}
        />
      </div>
    {/each}
  </div>
{/if}

<style>
  .dropzone {
    width: 100%;
    padding: 0.5rem;
    /* this will allow the dragged element to scroll the list although starting in version 0.9.41 the lib would detect any scrollable parent*/
    overflow: auto;
    height: 100%;
  }
</style>
