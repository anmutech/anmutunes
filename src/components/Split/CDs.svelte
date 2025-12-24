<script lang="ts">
  import { type Track, type CD, ActiveView } from "../../defs";
  import { data, app_state, viewState } from "../../state.svelte";
  import SplitTrack from "./Track.svelte";

  let {
    track_ids,
    coverColor,
  }: { track_ids: number[] | undefined; coverColor: string } = $props();

  const tracks: Track[] = $derived.by(() => {
    let tracks: Track[] = [];
    track_ids?.forEach((track_id) => {
      let track = data.tracks.get(track_id);
      if (track !== undefined) {
        tracks.push(track);
      }
    });
    return tracks;
  });

  const cds: CD[] = $derived.by(() => {
    let cd_map: Map<number, CD> = new Map();

    tracks.forEach((track) => {
      let cd = cd_map.get(track.disc_number);

      if (cd !== undefined) {
        cd.tracks.push(track);
      } else {
        let cd: CD = { index: track.disc_number, tracks: [track] };
        cd_map.set(track.disc_number, cd);
      }
    });

    let cds: CD[] = [];
    cd_map
      .keys()
      .toArray()
      .sort()
      .forEach((cd_id) => {
        let cd = cd_map.get(cd_id);
        if (cd !== undefined) {
          cds.push(cd);
        }
      });

    return cds;
  });

  let column_count = $derived.by(() => {
    if (viewState.view === ActiveView.Artists) {
      return Math.max(app_state.max_columns_tracks - 1, 1);
    }
    return app_state.max_columns_tracks;
  });

  /**
   * TODO:
   * if the album contains very few tracks, or few tracks per CD, do not use app_state.max_columns_track?
   * @media query to style the split
   * Columns need to be stacked at some point.
   * Both columns should be able to shrink even more for small windows.
   */
</script>

{#key cds}
  {#if cds.length != 0}
    {#if cds.length > 1}
      {#each cds as cd}
        <h4>CD {cd.index}</h4>
        <div class="tracks" style={`column-count: ${column_count}`}>
          {#each cd.tracks as track}
            <SplitTrack {track} {coverColor}></SplitTrack>
          {/each}
        </div>
      {/each}
    {:else}
      <div class="tracks" style={`column-count: ${column_count}`}>
        {#each cds[0].tracks as track}
          <SplitTrack {track} {coverColor}></SplitTrack>
        {/each}
      </div>
    {/if}
  {/if}
{/key}

<style>
  .tracks {
    width: 100%;
  }
</style>
