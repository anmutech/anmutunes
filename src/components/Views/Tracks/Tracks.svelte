<script lang="ts">
  import { DataType, Order } from "../../../defs";
  import { tableHeaderState, audio_state } from "../../../state.svelte";
  import Track from "./Track.svelte";
  import { toggleOrder } from "../../../actions.svelte";
  import SvelteVirtualList from "@humanspeak/svelte-virtual-list";
  import { translations } from "../../../localisation/localisation.svelte";

  let { track_ids }: { track_ids: number[] } = $props();

  /**
   * TODO:
   * make column widths adjustable.
   * store scroll position onDestroy so we can recover when the view is reopened.
   */

  function toggleName() {
    toggleOrder(Order.ByName, Order.ByNameInverse, DataType.Track);
  }
  function toggleArtist() {
    toggleOrder(Order.ByArtist, Order.ByArtistInverse, DataType.Track);
  }
  function toggleAlbumArtist() {
    toggleOrder(
      Order.ByAlbumArtist,
      Order.ByAlbumArtistInverse,
      DataType.Track
    );
  }
  function toggleAlbum() {
    toggleOrder(Order.ByAlbum, Order.ByAlbumInverse, DataType.Track);
  }
  function toggleGenre() {
    toggleOrder(Order.ByGenre, Order.ByGenreInverse, DataType.Track);
  }
  function toggleTime() {
    toggleOrder(Order.ByTime, Order.ByTimeInverse, DataType.Track);
  }

  function scroll() {
    if (listRef) {
      listRef.scroll({
        index: track_ids.indexOf(audio_state.current_track),
        smoothScroll: true,
        align: "top",
      });
    }
  }

  let listRef = $state<SvelteVirtualList<number>>();
  setTimeout(scroll, 300);
</script>

<div class="main-content">
  <div class="table-head track-row">
    <button
      onclick={toggleName}
      class={"table-heading no-bg " + tableHeaderState.name}
    >
      {translations.common.title}
    </button>
    <button class="table-heading no-bg"
      >{translations.common.tracknumber}</button
    >
    <button class="table-heading no-bg">{translations.common.cdnumber}</button>
    <button
      onclick={toggleArtist}
      class={"table-heading no-bg " + tableHeaderState.artist}
    >
      {translations.common.artist_one}
    </button>
    <button
      onclick={toggleAlbumArtist}
      class={"table-heading no-bg " + tableHeaderState.albumArtist}
    >
      {translations.common.albumartist}
    </button>
    <button
      onclick={toggleAlbum}
      class={"table-heading no-bg " + tableHeaderState.album}
    >
      {translations.common.album_one}
    </button>
    <button
      onclick={toggleGenre}
      class={"table-heading no-bg " + tableHeaderState.genre}
    >
      {translations.common.genre_one}
    </button>
    <button
      onclick={toggleTime}
      class={"table-heading no-bg " + tableHeaderState.time}
    >
      {translations.common.tracklength}
    </button>
  </div>
  <SvelteVirtualList items={track_ids} bind:this={listRef}>
    {#snippet renderItem(item)}
      <Track track_id={item} />
    {/snippet}
  </SvelteVirtualList>
</div>

<style>
  .main-content {
    height: 100%;
    width: 100%;
    display: grid;
    grid-template-rows: 27px auto;
  }

  .track-row {
    display: grid;
    grid-template-columns: 2fr 70px 50px 1fr 1fr 1fr 1fr 90px;
  }
</style>
