<script lang="ts">
  import {
    config_state,
    custom_colors_backup,
    customColorModalState,
  } from "../../state.svelte";
  import { set_config } from "../../actions.svelte";
  import Modal from "./Modal.svelte";
  import ColorPicker from "./ColorPicker.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import Spaced from "../Spaced.svelte";

  let active = $state(0);

  function save() {
    set_config();
    close();
  }

  function abort() {
    config_state.custom_colors = custom_colors_backup;
    close();
  }

  function close() {
    customColorModalState.visible = false;
  }
</script>

<Modal visible={customColorModalState.visible} close_callback={abort}>
  <div class="content">
    <div class="buttons">
      <button
        class={active === 0 ? "active" : ""}
        onclick={() => {
          active = 0;
        }}
      >
        {translations.coloredit.background}
      </button>
      <button
        class={active === 1 ? "active" : ""}
        onclick={() => {
          active = 1;
        }}
      >
        {translations.coloredit.background_active}
      </button>
      <button
        class={active === 2 ? "active" : ""}
        onclick={() => {
          active = 2;
        }}
      >
        {translations.coloredit.background_hover}
      </button>
      <button
        class={active === 3 ? "active" : ""}
        onclick={() => {
          active = 3;
        }}
      >
        {translations.coloredit.background_button}
      </button>
      <button
        class={active === 4 ? "active" : ""}
        onclick={() => {
          active = 4;
        }}
      >
        {translations.coloredit.border}
      </button>
      <button
        class={active === 5 ? "active" : ""}
        onclick={() => {
          active = 5;
        }}
      >
        {translations.coloredit.accent}
      </button>
      <button
        class={active === 6 ? "active" : ""}
        onclick={() => {
          active = 6;
        }}
      >
        {translations.coloredit.warn}
      </button>
      <button
        class={active === 7 ? "active" : ""}
        onclick={() => {
          active = 7;
        }}
      >
        {translations.coloredit.text}
      </button>
      <button
        class={active === 8 ? "active" : ""}
        onclick={() => {
          active = 8;
        }}
      >
        {translations.coloredit.text_dim}
      </button>
      <button
        class={active === 9 ? "active" : ""}
        onclick={() => {
          active = 9;
        }}
      >
        {translations.coloredit.text_highlight}
      </button>
      <button
        class={active === 10 ? "active" : ""}
        onclick={() => {
          active = 10;
        }}
      >
        {translations.coloredit.shadow}
      </button>
    </div>
    {#key active}
      <ColorPicker {active} />
    {/key}
    <div class="controls">
      <Spaced equal={true} withBackground={false} withGap={true} wide={true}>
        <button onclick={save}>
          {translations.common.save}
        </button>
        <button onclick={abort}>
          {translations.common.abort}
        </button>
      </Spaced>
    </div>
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
    grid-template-columns: auto 1fr;
    gap: 1rem;
  }

  .buttons button {
    width: 100%;
    justify-content: left;
  }

  .controls {
    grid-column-start: 1;
    grid-column-end: 3;
  }
</style>
