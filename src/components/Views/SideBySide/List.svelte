<script lang="ts">
  import SvelteVirtualList from "@humanspeak/svelte-virtual-list";
  import {
    ContextView,
    data,
    editModalState,
    viewState,
  } from "../../../state.svelte";
  import { ActiveView, DataType } from "../../../defs";
  import { buildContextMenu } from "../../../actions.svelte";
  import { translations } from "../../../localisation/localisation.svelte";

  function scroll() {
    let index = 0;

    switch (type) {
      case DataType.Artist:
        if (viewState.selected.artist) {
          index = data.artists_order.ids.indexOf(viewState.selected.artist);
        }
        break;
      case DataType.Composer:
        if (viewState.selected.composer) {
          index = data.composers_order.ids.indexOf(viewState.selected.composer);
        }
        break;
      case DataType.Genre:
        if (viewState.selected.genre) {
          index = data.genres_order.ids.indexOf(viewState.selected.genre);
        }
        break;
      case DataType.Playlist:
        if (viewState.selected.playlist) {
          index = data.playlists_order.ids.indexOf(viewState.selected.playlist);
        }
        break;
      default:
        break;
    }

    if (listRef) {
      listRef.scroll({
        index,
        smoothScroll: true,
        align: "top",
      });
    }
  }

  let { type, items, selected, div_class } = $derived.by(() => {
    switch (viewState.view) {
      case ActiveView.Artists:
        return {
          type: DataType.Artist,
          items: data.artists_order.ids,
          selected: viewState.selected.artist,
          div_class: "list",
        };
      case ActiveView.Composers:
        return {
          type: DataType.Composer,
          items: data.composers_order.ids,
          selected: viewState.selected.composer,
          div_class: "list",
        };
      case ActiveView.Genres:
        return {
          type: DataType.Genre,
          items: data.genres_order.ids,
          selected: viewState.selected.genre,
          div_class: "list",
        };
      case ActiveView.Playlists:
        return {
          type: DataType.Playlist,
          items: data.playlists_order.ids,
          selected: viewState.selected.playlist,
          div_class: "list playlist",
        };
      default:
        return {
          type: DataType.None,
          items: [],
          selected: 0,
          div_class: "list",
        };
    }
  });

  function openContent(id: number) {
    switch (type) {
      case DataType.Artist:
        viewState.selected.artist = id;
        break;
      case DataType.Composer:
        viewState.selected.composer = id;
        break;
      case DataType.Genre:
        viewState.selected.genre = id;
        break;
      case DataType.Playlist:
        viewState.selected.playlist = id;
        break;
      default:
        break;
    }
  }

  function get_name(item: number) {
    switch (type) {
      case DataType.Artist:
        return data.artists.get(item)?.name;
      case DataType.Composer:
        return data.composers.get(item)?.name;
      case DataType.Genre:
        return data.genres.get(item)?.name;
      case DataType.Playlist:
        return data.playlists.get(item)?.name;
      default:
        break;
    }
  }

  $effect(() => {
    if (type) {
      setTimeout(scroll, 300);
    }
  });

  let listRef = $state<SvelteVirtualList<number>>();
  // TODO: trigger list size recalculation on window height resize to prevent empty space below items.
</script>

<div class={div_class}>
  <SvelteVirtualList {items} bind:this={listRef}>
    {#snippet renderItem(item)}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class={selected === item ? "active row" : "row"}
        onclick={() => {
          openContent(item);
        }}
        oncontextmenu={(event) => {
          buildContextMenu(event, type, item, ContextView.Any);
        }}
      >
        {get_name(item) && get_name(item) !== "" ? get_name(item) : " "}
      </div>
    {/snippet}
  </SvelteVirtualList>
  {#if type === DataType.Playlist}
    <button
      onclick={() => {
        editModalState.id = 0;
        editModalState.type = DataType.Playlist;
        editModalState.visible = true;
      }}
    >
      {translations.common.newplaylist}
    </button>
  {/if}
</div>

<style>
  .list {
    border-right: var(--border);
  }

  .playlist {
    height: 100%;
    display: grid;
    grid-template-rows: auto 1.5rem;
  }

  .row {
    padding: 4px 16px;
    white-space: pre;
    overflow: hidden;
    text-overflow: ellipsis;
    font-weight: bold;
    cursor: pointer;
  }

  .row.active {
    background-color: var(--background-active);
  }

  .row:hover {
    background-color: var(--background-hover);
  }
</style>
