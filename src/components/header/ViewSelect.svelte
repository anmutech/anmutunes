<script lang="ts">
  import { Settings } from "@lucide/svelte";
  import { ActiveView } from "../../defs";
  import {
    config_state,
    data,
    settingsModalState,
    updateTableHeaderState,
    viewState,
  } from "../../state.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import Spaced from "../Spaced.svelte";
</script>

{#if !config_state.is_new}
  <div class="viewselect">
    <button
      class="settings no-bg"
      onclick={() => {
        settingsModalState.visible = true;
      }}
    >
      <Settings size={18} />
    </button>
    <Spaced equal={false} withBackground={true} withGap={false} wide={false}>
      <button
        onclick={() => {
          viewState.view = ActiveView.Recents;
        }}
        class={viewState.view == ActiveView.Recents ? "active" : ""}
      >
        {translations.viewselect.recents}
      </button>
      <button
        onclick={() => {
          viewState.view = ActiveView.Tracks;
          viewState.order = data.tracks_order.order;
          updateTableHeaderState();
        }}
        class={viewState.view == ActiveView.Tracks ? "active" : ""}
      >
        {translations.viewselect.tracks}
      </button>
      <button
        onclick={() => {
          viewState.view = ActiveView.Albums;
          viewState.order = data.albums_order.order;
          updateTableHeaderState();
        }}
        class={viewState.view == ActiveView.Albums ? "active" : ""}
      >
        {translations.viewselect.albums}
      </button>
      <button
        onclick={() => {
          viewState.view = ActiveView.Artists;
          viewState.order = data.artists_order.order;
          updateTableHeaderState();
        }}
        class={viewState.view == ActiveView.Artists ? "active" : ""}
      >
        {translations.viewselect.artists}
      </button>
      <button
        onclick={() => {
          viewState.view = ActiveView.Composers;
          viewState.order = data.composers_order.order;
          updateTableHeaderState();
        }}
        class={viewState.view == ActiveView.Composers ? "active" : ""}
      >
        {translations.viewselect.composers}
      </button>
      <button
        onclick={() => {
          viewState.view = ActiveView.Genres;
          viewState.order = data.genres_order.order;
          updateTableHeaderState();
        }}
        class={viewState.view == ActiveView.Genres ? "active" : ""}
      >
        {translations.viewselect.genres}
      </button>
      <button
        onclick={() => {
          viewState.view = ActiveView.Playlists;
          viewState.order = data.playlists_order.order;
          updateTableHeaderState();
        }}
        class={viewState.view == ActiveView.Playlists ? "active" : ""}
      >
        {translations.viewselect.playlists}
      </button>
    </Spaced>
  </div>
{/if}

<style>
  .viewselect {
    position: relative;
    align-content: center;
    grid-column-start: 1;
    grid-column-end: 4;
    border-top: var(--border);
    border-bottom: var(--border);
  }

  .settings {
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
  }
</style>
