<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { translations } from "../localisation/localisation.svelte";
  import { data, db } from "../state.svelte";
  import { millisecondsToReadableString } from "../state.svelte";

  let time = $derived.by(() => {
    if (data.spacetime.time) {
      return millisecondsToReadableString(data.spacetime.time);
    }
  });

  let space = $derived.by(() => {
    if (data.spacetime.space) {
      return (data.spacetime.space / 1073742000).toFixed(2);
    }
  });
</script>

{#key [db.albums_max, data.spacetime]}
  <footer class="footer">
    {db.albums_max}
    {db.albums_max === 1
      ? translations.common.album_one
      : translations.common.album_other}, {time}, {space} GiB &copy; 2025-{translations
      .common.present}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_missing_attribute -->
    <a
      onclick={() => {
        openUrl("https://github.com/anmutech");
      }}
      title="https://github.com/anmutech"
    >
      anmutech
    </a>
  </footer>
{/key}

<style>
  .footer {
    grid-area: var(--grid-area);
    text-align: center;
    align-content: center;
    border-top: var(--border);
    height: var(--footer-height);
  }
</style>
