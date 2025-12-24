<script lang="ts">
  import { editModalState } from "../../state.svelte";
  import { DataType } from "../../defs";
  import EditTrack from "./EditTrack.svelte";
  import EditAlbum from "./EditAlbum.svelte";
  import EditPlaylist from "./EditPlaylist.svelte";
  import Modal from "./Modal.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import EditGenre from "./EditGenre.svelte";
  import EditComposer from "./EditComposer.svelte";
  import EditArtist from "./EditArtist.svelte";
</script>

<Modal
  visible={editModalState.visible}
  close_callback={() => {
    editModalState.visible = false;
  }}
>
  <div class="content">
    {#if editModalState.type === DataType.Track}
      <EditTrack id={editModalState.id} />
    {:else if editModalState.type === DataType.Album}
      <EditAlbum id={editModalState.id} />
    {:else if editModalState.type === DataType.Artist}
      <EditArtist id={editModalState.id} />
    {:else if editModalState.type === DataType.Composer}
      <EditComposer id={editModalState.id} />
    {:else if editModalState.type === DataType.Genre}
      <EditGenre id={editModalState.id} />
    {:else if editModalState.type === DataType.Playlist}
      <EditPlaylist id={editModalState.id} />
    {:else}
      {translations.edit.cannotedit}
    {/if}
  </div>
</Modal>

<style>
  .content {
    position: fixed;
    top: calc(var(--full-header-height) + 1rem);
    left: 50%;
    transform: translateX(-50%);
    z-index: 1;
    border: var(--border);
    border-radius: var(--radius-small);
    background-color: var(--background);
    max-width: calc(100% - 8rem);
    max-height: calc(
      100dvh - (var(--full-header-height) + var(--footer-height) + 2rem)
    );
    overflow: auto;
    padding: 1rem;
    box-shadow: var(--box-shadow);
  }
</style>
