<script lang="ts">
  import { ActiveView } from "../../../defs";
  import { app_state, viewState } from "../../../state.svelte";
  import Albums from "./Albums.svelte";
  import Tracks from "./Tracks.svelte";
  import Playlist from "./Playlist.svelte";
</script>

{#if viewState.view === ActiveView.Artists}
  {#key [viewState.selected.artist, app_state.new_albums]}
    <Albums />
  {/key}
{:else if viewState.view === ActiveView.Playlists}
  {#key [viewState.selected.playlist, app_state.new_playlists]}
    <Playlist />
  {/key}
{:else}
  {#key [viewState.selected.composer, viewState.selected.genre, app_state.new_composers, app_state.new_genres]}
    <Tracks />
  {/key}
{/if}

<style></style>
