<script lang="ts">
  import { DataType, Order } from "../../../defs";
  import { data, app_state, tableHeaderState } from "../../../state.svelte";
  import Album from "./Album.svelte";
  import { toggleOrder } from "../../../actions.svelte";
  import Split from "../../Split/Split.svelte";
  import SvelteVirtualList from "@humanspeak/svelte-virtual-list";
  import { translations } from "../../../localisation/localisation.svelte";

  function toggleTitle() {
    toggleOrder(Order.ByName, Order.ByNameInverse, DataType.Album);
  }
  function toggleReleaseDate() {
    toggleOrder(
      Order.ByReleaseDate,
      Order.ByReleaseDateInverse,
      DataType.Album
    );
  }
  function toggleAddedDate() {
    toggleOrder(Order.ByAddedDate, Order.ByAddedDateInverse, DataType.Album);
  }
  function toggleAlbumArtist() {
    toggleOrder(
      Order.ByAlbumArtist,
      Order.ByAlbumArtistInverse,
      DataType.Album
    );
  }
  function toggleGenre() {
    toggleOrder(Order.ByGenre, Order.ByGenreInverse, DataType.Album);
  }

  function scroll() {
    if (listRef) {
      listRef.scroll({
        index: data.albums_order.ids.indexOf(app_state.open_split),
        smoothScroll: true,
        align: "top",
      });
    }
  }

  let listRef = $state<SvelteVirtualList<number>>();
  setTimeout(scroll, 300);

  /**
   * TODO:
   * make column widths adjustable.
   * store scroll position onDestroy so we can recover when the view is reopened.
   */

  let albumHeightEstimate = 60; // defined as 60px in ./Album.svelte
</script>

<div class="main-content">
  <div class="table-head album-row">
    <button class="table-heading no-bg">{translations.common.cover}</button>
    <button
      onclick={toggleTitle}
      class={"table-heading no-bg " + tableHeaderState.name}
    >
      {translations.common.title}
    </button>
    <button
      onclick={toggleAlbumArtist}
      class={"table-heading no-bg " + tableHeaderState.albumArtist}
    >
      {translations.common.albumartist}
    </button>
    <button
      onclick={toggleGenre}
      class={"table-heading no-bg " + tableHeaderState.genre}
    >
      {translations.common.genre_one}
    </button>
    <button
      onclick={toggleReleaseDate}
      class={"table-heading no-bg " + tableHeaderState.releaseDate}
    >
      {translations.common.release}
    </button>
    <button
      onclick={toggleAddedDate}
      class={"table-heading no-bg " + tableHeaderState.addedDate}
    >
      {translations.common.added}
    </button>
    <button class="table-heading no-bg"
      ># {translations.common.track_other}</button
    >
  </div>
  <SvelteVirtualList
    items={data.albums_order.ids}
    defaultEstimatedItemHeight={albumHeightEstimate}
    bind:this={listRef}
  >
    {#snippet renderItem(item)}
      <Album album_id={item} />
      {#if item == app_state.open_split}
        <Split album={item} track_ids={undefined} className={"albums"} />
      {/if}
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

  .album-row {
    display: grid;
    grid-template-columns: 70px 1.5fr 1fr 1fr 1fr 1fr 90px;
  }
</style>
