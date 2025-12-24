<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    config_state,
    customColorModalState,
    mediaPathModalState,
    settingsModalState,
    translateLanguage,
    translateStartupView,
    translateTheme,
    tutorialModalState,
  } from "../../state.svelte";
  import { Language, Theme, View } from "../../defs";
  import Select from "../Select.svelte";
  import {
    reset_custom_colors_dark,
    reset_custom_colors_light,
    set_config,
  } from "../../actions.svelte";
  import Modal from "./Modal.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import Spaced from "../Spaced.svelte";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { locale } from "@tauri-apps/plugin-os";

  let active_setting = $state(0);

  let llm_translation = $derived.by(async () => {
    let llm_translation = true;
    switch (config_state.language) {
      case Language.German:
      case Language.English:
        llm_translation = false;
        break;
      case Language.System:
        let oslocale = await locale();
        console.log("oslocale", oslocale);
        switch (oslocale) {
          case "de":
          case "de-DE":
          case "en":
            llm_translation = false;
            break;
          default:
            llm_translation = true;
        }
        break;
      default:
        llm_translation = true;
    }
    console.log("translation llm: ", llm_translation);

    return llm_translation;
  });
</script>

<Modal
  visible={tutorialModalState.visible}
  close_callback={() => {
    tutorialModalState.visible = false;
  }}
>
  <div class="content">
    <h2>{translations.tutorial.tutorial}</h2>
    <small>
      {translations.tutorial.ccmessage}
      {translations.tutorial.attributions}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <!-- svelte-ignore a11y_missing_attribute -->
      <a
        onclick={() => {
          openUrl(
            "https://github.com/anmutech/anmutunes/blob/main/static/ATTRIBUTIONS.txt"
          );
        }}
        title="https://github.com/anmutech/anmutunes/blob/main/static/ATTRIBUTIONS.txt"
      >
        anmutunes Attributions
      </a>
    </small>
    <!-- svelte-ignore a11y_media_has_caption -->
    <video autoplay controls preload="metadata">
      <source src="anmutunes.mp4" />
    </video>
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
    max-height: calc(
      100dvh - (var(--full-header-height) + var(--footer-height) + 2rem)
    );
    width: max-content;
    max-width: calc(100dvw - 2rem);
    aspect-ratio: 4/3;
    overflow: auto;
    padding: 1rem;
    text-align: center;
    box-shadow: var(--box-shadow);
    display: grid;
    gap: 0.5rem;
  }

  h2 {
    margin: 0;
  }

  video {
    max-width: 100%;
    border-radius: var(--radius-small);
    border: var(--border);
  }
</style>
