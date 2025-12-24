<script lang="ts">
  import { ActiveView, DataType } from "../../../defs";
  import { app_state, data, viewState } from "../../../state.svelte";
  import Tracks from "../Tracks/Tracks.svelte";

  let { type, id } = $derived.by(() => {
    switch (viewState.view) {
      case ActiveView.Composers:
        return { type: DataType.Composer, id: viewState.selected.composer };
      case ActiveView.Genres:
        return { type: DataType.Genre, id: viewState.selected.genre };
      default:
        return { type: DataType.None, id: 0 };
    }
  });

  let track_ids = $derived.by(() => {
    const positionMap = new Map();
    data.tracks_order.ids.forEach((num, index) => {
      positionMap.set(num, index);
    });
    switch (type) {
      case DataType.Composer:
        return data.composer_tracks.get(id)?.sort((a, b) => {
          return positionMap.get(a) - positionMap.get(b);
        });
      case DataType.Genre:
        return data.genre_tracks.get(id)?.sort((a, b) => {
          return positionMap.get(a) - positionMap.get(b);
        });
      default:
        return undefined;
    }
  });
</script>

{#key app_state.new_order}
  <div class="grid">
    <div class="info">
      {#if viewState.view == ActiveView.Composers}
        <h2>{data.composers.get(id)?.name}</h2>
      {:else}
        <h2>{data.genres.get(id)?.name}</h2>
      {/if}
    </div>
    {#if track_ids}
      <Tracks {track_ids} />
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

  h2 {
    margin: 0;
  }

  .info {
    padding: 0.5rem;
    padding-left: 2rem;
    padding-right: 2rem;
  }
</style>
