<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { Language, Theme, View } from "../../../defs";
  import { translations } from "../../../localisation/localisation.svelte";
  import {
    config_state,
    translateLanguage,
    translateStartupView,
    translateTheme,
  } from "../../../state.svelte";
  import Select from "../../Select.svelte";
  import { locale } from "@tauri-apps/plugin-os";

  let { page }: { page: number } = $props();

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

<div class="setup">
  <h1>{translations.setup.setup}</h1>
  {#if page === 0}
    <div class="container">
      <h4>{translations.settings.language}</h4>
      <Select
        active={translateLanguage(config_state.language)}
        options={Object.values(Language).map((value) => {
          return { name: translateLanguage(value), value };
        })}
        select_value={(value: Language) => {
          config_state.language = value;
        }}
      />
      <div>
        {#await llm_translation then llm}
          {#if llm}
            {translations.llminfo.long} <br />
            {translations.llminfo.notice}
          {/if}
        {/await}
      </div>
    </div>
  {:else if page === 1}
    <div class="container">
      <div>
        <div>
          <h4>{translations.setup.mediapath}</h4>
          {config_state.media_path}
        </div>
        <small>
          {translations.setup.mediapath_explain} <br />
          <!-- TODO: actually implement scanning media path
          {#if !config_state.manage_folders}
            {translations.setup.mediapath_inactive_explain}
          {/if}
          -->
        </small>
      </div>
      <div>
        <input
          id="manage"
          type="checkbox"
          checked={config_state.manage_folders}
          onclick={() => {
            config_state.manage_folders = !config_state.manage_folders;
          }}
        />
        <label for="manage">{translations.settings.managefolders}</label>
      </div>
      <small>
        {#if config_state.manage_folders}
          {translations.settings.managefolders_explain_active}
        {:else}
          {translations.settings.managefolders_explain_inactive}
        {/if}
      </small>
    </div>
  {:else if page === 2}
    <div class="container">
      <div>
        <input
          id="deletedb"
          type="checkbox"
          checked={config_state.allow_delete_from_db}
          onclick={() => {
            config_state.allow_delete_from_db =
              !config_state.allow_delete_from_db;
            config_state.allow_delete_files = false;
          }}
        />
        <label for="deletedb">{translations.settings.allowdelete}</label>
      </div>
      <small>
        {#if config_state.allow_delete_from_db}
          {translations.settings.allowdelete_active}
        {:else}
          {translations.settings.allowdelete_inactive}
        {/if}
      </small>
      {#if config_state.allow_delete_from_db}
        <div>
          <input
            id="deletefiles"
            type="checkbox"
            checked={config_state.allow_delete_files}
            onclick={() => {
              config_state.allow_delete_files =
                !config_state.allow_delete_files;
            }}
          />
          <label for="deletefiles"
            >{translations.settings.allowdeletefiles}</label
          >
        </div>
        <small>
          {#if config_state.allow_delete_files}
            {translations.settings.allowdeletefiles_active}
          {:else}
            {translations.settings.allowdeletefiles_inactive}
          {/if}
        </small>
      {/if}
    </div>
  {:else if page === 3}
    <div class="container">
      <div>
        <!--<input
          id="allowupdate"
          type="checkbox"
          checked={config_state.look_for_updates}
          onclick={() => {
            config_state.look_for_updates = !config_state.look_for_updates;
          }}
        />
        <label for="allowupdates">
          {translations.settings.searchforupdates}
        </label>-->
        {translations.settings.updatesarehere}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <!-- svelte-ignore a11y_missing_attribute -->
        <a
          onclick={() => {
            openUrl("https://github.com/anmutech/anmutunes/releases");
          }}
          title="https://github.com/anmutech/anmutunes/releases"
        >
          anmutunes Releases
        </a>
      </div>
      <!--<small>
        {#if config_state.look_for_updates}
          {translations.setup.searchforupdates_active_explain}
        {:else}
          {translations.setup.searchforupdates_inactive_explain}
        {/if}
      </small>-->
    </div>
  {:else if page === 4}
    <div class="container">
      <h4>{translations.settings.theme.theme}</h4>
      <Select
        active={translateTheme(config_state.theme)}
        options={Object.values(Theme).map((value) => {
          return { name: translateTheme(value), value };
        })}
        select_value={(value: Theme) => {
          config_state.theme = value;
        }}
      />
    </div>
  {:else if page === 5}
    <div class="container">
      <h4>{translations.settings.startupview}</h4>
      <Select
        active={translateStartupView(config_state.startup_view)}
        options={Object.values(View).map((value) => {
          return { name: translateStartupView(value), value };
        })}
        select_value={(value: View) => {
          config_state.startup_view = value;
        }}
      />
    </div>
  {/if}
</div>

<style>
  .setup {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    text-align: center;
  }

  h4 {
    margin: 0.5rem;
  }

  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: auto;
    margin-top: 2rem;
  }
</style>
