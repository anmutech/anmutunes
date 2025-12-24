<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { translations } from "../../../localisation/localisation.svelte";
</script>

<div class="tutorial">
  <h1>{translations.tutorial.tutorial}</h1>
  <small>{translations.tutorial.ccmessage}</small>
  <small>
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
  <div class="video-container">
    <!-- svelte-ignore a11y_media_has_caption -->
    <video autoplay controls preload="metadata">
      <source src="anmutunes.mp4" />
    </video>
  </div>
</div>

<style>
  .tutorial {
    display: flex;
    flex-direction: column;
    width: 100%;
    text-align: center;
    overflow: auto;
  }

  .tutorial .video-container {
    padding: 2rem;
  }

  video {
    max-width: 100%;
    border-radius: var(--radius-small);
    border: var(--border);
  }
</style>
