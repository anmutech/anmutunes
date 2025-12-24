<script lang="ts">
  import { DataType, type Track } from "../../defs";
  import { audio_state, ContextView, data } from "../../state.svelte";
  import { Volume2 } from "@lucide/svelte";
  import { buildContextMenu, play_by_id } from "../../actions.svelte";

  let { track, coverColor }: { track: Track; coverColor: string } = $props();

  function handleContextMenu(event: MouseEvent) {
    buildContextMenu(event, DataType.Track, track.id, ContextView.Any);
  }

  let track_length =
    String(Math.floor(track.total_time / 60000)) +
    ":" +
    String(Math.floor((track.total_time / 1000) % 60)).padStart(2, "0");

  // TODO: based on coverColor calculate hover background-color for track
  let className = $derived.by(() => {
    let className =
      audio_state.current_track == track.id ? "track playing" : "track";

    if (coverColor === "var(--background)") {
      className += " artists";
    }
    return className;
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class={className}
  onclick={() => {
    play_by_id(track.id, DataType.Track);
  }}
  oncontextmenu={handleContextMenu}
>
  <div class="track-number">
    {#if audio_state.current_track == track.id}
      <Volume2 size={20} />
    {:else}
      {track.track_number}
    {/if}
  </div>
  <div class="track-name">
    {track.name}
    {#if track.album_artist_id !== track.artist_id}
      <span>- {data.artists.get(track.artist_id)?.name}</span>
    {/if}
  </div>
  <div class="track-length">
    {track_length}
  </div>
</div>

<style>
  .track {
    display: grid;
    width: 100%;
    grid-template-columns: 2rem auto auto;
    gap: 1rem;
    cursor: pointer;
    padding: 5px;
    break-inside: avoid-column;
    border-radius: var(--radius-small);
  }

  .track:hover {
    background-color: var(--background);
  }

  .track.artists:hover {
    background-color: var(--background-hover);
  }

  .track-number {
    display: flex;
    justify-content: end;
    align-items: center;
  }

  .track-name {
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: pre;
  }

  .track-name span {
    color: var(--text-dim);
  }

  .track-length {
    text-align: right;
  }
</style>
