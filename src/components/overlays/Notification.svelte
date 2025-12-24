<script lang="ts">
  import { Notification } from "../../defs";
  import { translations } from "../../localisation/localisation.svelte";
  import { notificationState } from "../../state.svelte";
  import Spaced from "../Spaced.svelte";
  import Modal from "./Modal.svelte";

  function close() {
    notificationState.visible = false;
  }
</script>

<Modal visible={notificationState.visible} close_callback={() => {}}>
  <div class="content">
    <div>
      {#if notificationState.notification == Notification.LibraryImport}
        {translations.notification.libraryimport.first} <br />
        {translations.notification.libraryimport.second} <br /><br />
        {translations.notification.libraryimport.third}
      {/if}
    </div>
    <Spaced equal={true} withBackground={false} withGap={true} wide={true}>
      <button onclick={close}>{translations.common.close}</button>
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
    display: grid;
    gap: 1rem;
  }
</style>
