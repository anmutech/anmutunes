<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { importModalState } from "../../state.svelte";
  import Modal from "./Modal.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import Spaced from "../Spaced.svelte";

  function importLibrary() {
    close();
    invoke("dbrequest", {
      request: {
        ImportLibrary: importModalState.path,
      },
    });
  }

  function close() {
    importModalState.visible = false;
  }

  /**
   * TODO:
   * Better layout, buttons side by side.
   * visual feedback how many tracks are already imported, and how many playlists.
   */
</script>

<Modal visible={importModalState.visible} close_callback={close}>
  <div class="content">
    <h3>{translations.import.importlibrary}</h3>
    <Spaced equal={true} withBackground={false} withGap={true} wide={true}>
      <button onclick={importLibrary}>{translations.common.yes}</button>
      <button onclick={close}>{translations.common.no}</button>
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
  }

  h3 {
    margin-top: 0;
  }
</style>
