<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { DataType } from "../../defs";
  import { translations } from "../../localisation/localisation.svelte";
  import { config_state, data, deleteModalState } from "../../state.svelte";
  import Modal from "./Modal.svelte";
  import Spaced from "../Spaced.svelte";

  function close() {
    deleteModalState.visible = false;
  }

  function delete_content(including_files: boolean) {
    invoke("dbrequest", {
      request: {
        DeleteById: [
          deleteModalState.context.type,
          [deleteModalState.context.id],
          including_files,
        ],
      },
    });
    deleteModalState.visible = false;
  }

  let { type, name, message } = $derived.by(() => {
    switch (deleteModalState.context.type) {
      case DataType.Album:
        return {
          type: translations.common.album_one,
          name: data.albums.get(deleteModalState.context.id)?.name,
          message: translations.deletemodal.albumsandtracks,
        };
      case DataType.Artist:
        return {
          type: translations.common.artist_one,
          name: data.artists.get(deleteModalState.context.id)?.name,
          message: translations.deletemodal.artistandtracks,
        };
      case DataType.Composer:
        return {
          type: translations.common.composer_one,
          name: data.composers.get(deleteModalState.context.id)?.name,
          message: translations.deletemodal.composerandtracks,
        };
      case DataType.Genre:
        return {
          type: translations.common.genre_one,
          name: data.genres.get(deleteModalState.context.id)?.name,
          message: translations.deletemodal.genreandtracks,
        };
      case DataType.Track:
        return {
          type: translations.common.track_one,
          name: data.tracks.get(deleteModalState.context.id)?.name,
          message: translations.deletemodal.track,
        };
      case DataType.Playlist:
        return {
          type: translations.common.playlist_one,
          name: data.playlists.get(deleteModalState.context.id)?.name,
          message: translations.deletemodal.playlist,
        };
      default:
        // Cover makes no sense and should never happen. Video is not implemented and should never happen.
        return {
          type: "None",
          name: "None",
          message: "",
        };
    }
  });
</script>

<Modal visible={deleteModalState.visible} close_callback={close}>
  <div class="content">
    <h3>{type}: {name}</h3>
    <p>{message}</p>
    <Spaced equal={true} withBackground={false} withGap={true} wide={true}>
      <button
        onclick={() => {
          delete_content(false);
        }}
      >
        {translations.common.delete}
      </button>
      {#if config_state.allow_delete_files && deleteModalState.context.type != DataType.Playlist}
        <button
          onclick={() => {
            delete_content(true);
          }}
        >
          {translations.deletemodal.deleteincludingfiles}
        </button>
      {/if}
      <button onclick={close}>{translations.common.abort}</button>
    </Spaced>
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
    text-align: center;
  }

  h3 {
    margin-top: 0;
  }
</style>
