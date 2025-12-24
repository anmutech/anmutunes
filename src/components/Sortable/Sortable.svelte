<script lang="ts">
  import { type Track } from "../../defs";
  import { ContextView, data } from "../../state.svelte";
  import SortableTrack from "./Track.svelte";
  import { flip } from "svelte/animate";
  import { dndzone, type DndEvent } from "svelte-dnd-action";
  import { translations } from "../../localisation/localisation.svelte";
  // TODO: some way to get virtual scroll compatible with dnd?

  let {
    track_ids,
    finalize_callback,
    enable_playing,
    enable_remove,
    remove_callback,
    onclick_callback,
    contextview,
  }: {
    track_ids: number[];
    finalize_callback: (items: { id: number; track: Track }[]) => void;
    enable_playing: boolean;
    enable_remove: boolean;
    remove_callback: (item: { id: number; track: Track }) => void;
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

  const flipDurationMs = 0;
  const dropTargetStyle = {};

  function handleDndConsider(e: {
    detail: DndEvent<{ id: number; track: Track }>;
  }) {
    items = e.detail.items;
  }

  function handleDndFinalize(e: {
    detail: DndEvent<{ id: number; track: Track }>;
  }) {
    items = e.detail.items;

    finalize_callback(items);
  }
</script>

<div
  class="dropzone"
  use:dndzone={{ items, dropTargetStyle }}
  onconsider={handleDndConsider}
  onfinalize={handleDndFinalize}
>
  {#if items.length !== 0}
    {#each items as item (item.id)}
      <div animate:flip={{ duration: flipDurationMs }}>
        <SortableTrack
          track={item.track}
          coverColor={"var(--background)"}
          {enable_playing}
          {enable_remove}
          remove_callback={() => {
            remove_callback(item);
          }}
          onclick_callback={() => {
            onclick_callback(item);
          }}
          {contextview}
        />
      </div>
    {/each}
  {:else}
    <div class="empty">{translations.common.listisempty}</div>
  {/if}
</div>

<style>
  .dropzone {
    width: 100%;
    padding: 0.5rem;
    height: 100%;
  }

  .empty {
    color: var(--text-dim);
  }
</style>
