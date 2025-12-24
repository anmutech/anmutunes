<script lang="ts">
  import Album from "./Album.svelte";
  import { app_state } from "../../../state.svelte";
  import Split from "../../Split/Split.svelte";

  let { albums }: { albums: number[] } = $props();

  /**
   * TODO:
   *
   * Only once, when app_state.open_split changes
   *
   * if (albums.includes(app_state.open_split)) {
   *    row_visible = #test
   *    split_visible = #test
   *    if (!split_visible || !row_visible) {
   *      scrollToRow();
   *    }
   * }
   */
</script>

<div>
  <div class="row">
    {#each albums as album_id}
      <Album {album_id}></Album>
    {/each}
  </div>
  {#if albums.includes(app_state.open_split)}
    <Split
      album={app_state.open_split}
      track_ids={undefined}
      className={"recents"}
    ></Split>
  {/if}
</div>

<style>
  .row {
    display: grid;
    grid-template-columns: repeat(
      var(--max-columns-albums),
      calc(100% / var(--max-columns-albums))
    );
    margin-bottom: 1rem;
    padding: 0 1rem;
  }
</style>
