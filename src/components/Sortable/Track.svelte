<script lang="ts">
  import { DataType, type Track } from "../../defs";
  import { audio_state, ContextView, data } from "../../state.svelte";
  import { Volume2 } from "@lucide/svelte";
  import { buildContextMenu } from "../../actions.svelte";
  import X from "../../graphics/x.svelte";

  let {
    track,
    coverColor,
    enable_playing,
    enable_remove,
    remove_callback,
    onclick_callback,
    contextview,
  }: {
    track: Track;
    coverColor: string;
    enable_playing: boolean;
    enable_remove: boolean;
    remove_callback: () => void;
    onclick_callback: () => void;
    contextview: ContextView;
  } = $props();

  function handleContextMenu(event: MouseEvent) {
    buildContextMenu(event, DataType.Track, track.id, contextview);
  }

  let track_length = $derived(
    String(Math.floor(track.total_time / 60000)) +
      ":" +
      String(Math.floor((track.total_time / 1000) % 60)).padStart(2, "0")
  );

  // TODO: based on coverColor calculate hover background-color for track
  let className = $derived.by(() => {
    let className =
      enable_playing && audio_state.current_track == track.id
        ? "track playing"
        : "track";
    if (coverColor === "var(--background)") {
      className += " artists";
    }
    if (enable_remove) {
      className += " removeable";
    }
    return className;
  });

  let artist = $derived(data.artists.get(track.artist_id));
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class={className} oncontextmenu={handleContextMenu}>
  {#if enable_playing && audio_state.current_track == track.id}
    <Volume2 />
  {/if}
  <div class="track-name" onclick={onclick_callback}>
    {track.name} - {artist?.name}
  </div>
  <div class="track-length" onclick={onclick_callback}>
    {track_length}
  </div>
  {#if enable_remove}
    <div class="remove" onclick={remove_callback}>
      <X />
    </div>
  {/if}
</div>

<style>
  .track {
    display: grid;
    width: 100%;
    grid-template-columns: auto 5rem;
    gap: 1rem;
    cursor: pointer;
    padding: 6px 12px;
    border-radius: var(--radius-small);
  }

  .track.removeable {
    grid-template-columns: auto 5rem 1rem;
  }

  .playing {
    grid-template-columns: 1rem auto 5rem;
  }

  .playing.removeable {
    grid-template-columns: 1rem auto 5rem 1rem;
  }

  .track:hover {
    background-color: var(--background);
  }

  .track.artists:hover {
    background-color: var(--background-hover);
  }

  .track-number {
    text-align: right;
  }

  .track-name {
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: pre;
  }

  .track-length {
    text-align: right;
  }

  .remove {
    display: flex;
  }
</style>
