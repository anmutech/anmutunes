<script lang="ts">
  import { config_state, mediaPathModalState } from "../../state.svelte";
  import { set_config } from "../../actions.svelte";
  import Modal from "./Modal.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import Spaced from "../Spaced.svelte";

  function setMediaPath() {
    config_state.media_path = mediaPathModalState.path;
    set_config();
    close();
  }

  function close() {
    mediaPathModalState.path = "";
    mediaPathModalState.visible = false;
  }

  /**
   * TODO:
   * Better layout, buttons side by side.
   * visual feedback how many tracks are already imported, and how many playlists.
   */
</script>

<Modal visible={mediaPathModalState.visible} close_callback={close}>
  <div class="content">
    <div>
      <h3>{translations.mediapath.currentmediapath}:</h3>
      <div>{config_state.media_path}</div>
    </div>

    {#if mediaPathModalState.path === ""}
      {#if config_state.manage_folders}
        <div>
          {translations.mediapath.explain}
        </div>
      {/if}
      <div>{translations.mediapath.draganddroppath}</div>
      <Spaced equal={true} withBackground={false} withGap={true} wide={true}>
        <button onclick={close}>{translations.common.abort}</button>
      </Spaced>
    {:else}
      <div>
        <h3>{translations.mediapath.newmediapath}:</h3>
        <div>{mediaPathModalState.path}</div>
      </div>
      {#if config_state.manage_folders}
        <div>
          {translations.mediapath.explain}
        </div>
      {/if}
      <Spaced equal={true} withBackground={false} withGap={true} wide={true}>
        <button onclick={setMediaPath}>{translations.common.save}</button>
        <button onclick={close}>{translations.common.abort}</button>
      </Spaced>
    {/if}
  </div>
</Modal>

<style>
  .content {
    position: fixed;
    top: calc(var(--full-header-height) + 2rem);
    left: 50%;
    transform: translateX(-50%);
    z-index: 2; /* Show on top of Settings */
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
    display: grid;
    gap: 1rem;
  }

  h3 {
    margin-top: 0;
  }
</style>
