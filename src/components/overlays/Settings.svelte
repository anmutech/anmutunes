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
  visible={settingsModalState.visible}
  close_callback={() => {
    console.log("modal close called");
    settingsModalState.visible = false;
  }}
>
  <div class="content">
    <div>
      <h2>{translations.settings.settings}</h2>
      <Spaced equal={false} withBackground={true} withGap={false} wide={false}>
        <button
          onclick={() => {
            active_setting = 0;
          }}
          class={active_setting === 0 ? "active" : ""}
        >
          {translations.settings.general}
        </button>
        <!--<button
          onclick={() => {
            active_setting = 1;
          }}
          class={active_setting === 1 ? "active" : ""}
        >
          {translations.settings.playback}
        </button>-->
        <button
          onclick={() => {
            active_setting = 2;
          }}
          class={active_setting === 2 ? "active" : ""}
        >
          {translations.settings.files}
        </button>
        <button
          onclick={() => {
            active_setting = 3;
          }}
          class={active_setting === 3 ? "active" : ""}
        >
          {translations.settings.tools}
        </button>
      </Spaced>
    </div>
    {#if active_setting === 0}
      <div class="general">
        <div class="row">
          <div class="left-col">{translations.settings.theme.theme}:</div>
          <Select
            active={translateTheme(config_state.theme)}
            options={Object.values(Theme).map((value) => {
              // TODO: translate to correct theme translation
              return { name: translateTheme(value), value };
            })}
            select_value={(option: Theme) => {
              config_state.theme = option;
              set_config();
            }}
          />
        </div>
        {#if config_state.theme === Theme.Custom}
          <div class="row">
            <button
              class="right-col"
              onclick={() => {
                customColorModalState.visible = true;
              }}
            >
              {translations.settings.theme.editcolors}
            </button>
            <button class="right-col" onclick={reset_custom_colors_light}>
              {translations.settings.theme.resetlight}
            </button>
            <button class="right-col" onclick={reset_custom_colors_dark}>
              {translations.settings.theme.resetdark}
            </button>
            <!--
            TODO: create function that generates color themes based on seed
            <button class="right-col"> Random </button>
            -->
          </div>
        {/if}
        <div class="row">
          <div class="left-col">{translations.settings.startupview}:</div>
          <Select
            active={translateStartupView(config_state.startup_view)}
            options={Object.values(View).map((value) => {
              // TODO: translate from value to correct viewselect translation
              return { name: translateStartupView(value), value };
            })}
            select_value={(option: View) => {
              config_state.startup_view = option;
              set_config();
            }}
          />
        </div>
        <div class="row">
          <div class="left-col">{translations.settings.language}:</div>
          <Select
            active={translateLanguage(config_state.language)}
            options={Object.values(Language).map((value) => {
              return { name: translateLanguage(value), value };
            })}
            select_value={(option: Language) => {
              config_state.language = option;
              set_config();
            }}
          />
        </div>
        {#await llm_translation then llm}
          {#if llm}
            <div class="row">
              <div class="right-col">
                {translations.llminfo.short} <br />
                {translations.llminfo.notice}
              </div>
            </div>
          {/if}
        {/await}
        <div class="row">
          <div class="left-col">{translations.tutorial.tutorial}:</div>
          <div>
            <button
              onclick={() => {
                tutorialModalState.visible = true;
              }}
            >
              {translations.tutorial.tutorial}
            </button>
          </div>
        </div>
        <div>
          <!--<input
            type="checkbox"
            name=""
            id="autoupdate"
            checked={config_state.look_for_updates}
            onclick={() => {
              config_state.look_for_updates = !config_state.look_for_updates;
              set_config();
            }}
          />
          <label for="autoupdate">
            {translations.settings.searchforupdates}
          </label>-->
          <div>{translations.settings.updatesarehere}</div>
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
      </div>
    {:else if active_setting === 1}
      <div class="playback">{translations.settings.playback}</div>
    {:else if active_setting === 2}
      <div class="files">
        <div class="row">
          <div class="left-col">{translations.settings.mediapath}</div>
          <div class="right-col">{config_state.media_path}</div>
        </div>
        <div class="row">
          <div class="right-col">
            <button
              class="menu-item"
              onclick={() => {
                mediaPathModalState.visible = true;
              }}
            >
              {translations.settings.changemediapath}
            </button>
            <!--<button>Zurücksetzen</button>-->
          </div>
        </div>
        <div class="row">
          <div class="right-col">
            <input
              type="checkbox"
              name=""
              id="automatic"
              checked={config_state.manage_folders}
              onclick={() => {
                config_state.manage_folders = !config_state.manage_folders;
                set_config();
              }}
            />
            <label for="automatic">{translations.settings.managefolders}</label>
            <div>
              {#if config_state.manage_folders}
                <small>
                  {translations.settings.managefolders_explain_active}
                </small>
                <button
                  onclick={() => {
                    invoke("dbrequest", {
                      request: "CopyNotCopied",
                    });
                  }}
                >
                  {translations.settings.copynotcopiedmedia}
                </button>
              {:else}
                <small>
                  {translations.settings.managefolders_explain_inactive}
                </small>
              {/if}
            </div>
          </div>
        </div>
        <div class="row">
          <div class="right-col">
            <input
              type="checkbox"
              name=""
              id="allowdelete"
              checked={config_state.allow_delete_from_db}
              onclick={() => {
                config_state.allow_delete_from_db =
                  !config_state.allow_delete_from_db;
                config_state.allow_delete_files = false;
                set_config();
              }}
            />
            <label for="allowdelete">{translations.settings.allowdelete}</label>
            <div>
              <small>
                {#if config_state.allow_delete_from_db}
                  {translations.settings.allowdelete_active}
                {:else}
                  {translations.settings.allowdelete_inactive}
                {/if}
              </small>
            </div>
          </div>
        </div>
        {#if config_state.allow_delete_from_db}
          <div class="row">
            <div class="right-col">
              <input
                type="checkbox"
                name=""
                id="allowdeletefiles"
                checked={config_state.allow_delete_files}
                onclick={() => {
                  config_state.allow_delete_files =
                    !config_state.allow_delete_files;
                  set_config();
                }}
              />
              <label for="allowdeletefiles"
                >{translations.settings.allowdeletefiles}</label
              >
              <div>
                <small>
                  {#if config_state.allow_delete_files}
                    {translations.settings.allowdeletefiles_active}
                  {:else}
                    {translations.settings.allowdeletefiles_inactive}
                  {/if}
                </small>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {:else if active_setting === 3}
      <div class="tools">
        <div class="row">
          <div class="left-col">
            <button
              class="menu-item"
              onclick={() => {
                invoke("dbrequest", {
                  request: "ExtractCovers",
                });
              }}
            >
              {translations.settings.extractcovers}
            </button>
          </div>
          <div class="right-col">
            <small>
              {translations.settings.extractcovers_explain}
            </small>
          </div>
        </div>
        <!--<div class="row">
          <div class="left-col">
            <button class="menu-item" onclick={() => {}}>
              {translations.settings.showmissingtracks}
            </button>
          </div>
          <div class="right-col">
            <small>
              {translations.settings.showmissingtracks_explain}
            </small>
          </div>
        </div>
        <div class="row">
          <div class="left-col">
            <button class="menu-item" onclick={() => {}}>
              {translations.settings.standardisenames}
            </button>
          </div>
          <div class="right-col">
            <small>
              {translations.settings.standardisenames_explain}
            </small>
          </div>
        </div>
        <div class="row">
          <div class="left-col">
            <button class="menu-item" onclick={() => {}}>
              {translations.settings.extractmetadata}
            </button>
          </div>
          <div class="right-col">
            <small>
              {translations.settings.extractmetadata_explain}
            </small>
          </div>
        </div>-->
      </div>
    {/if}
    <div>
      {translations.settings.support_please} <br />
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <!-- svelte-ignore a11y_missing_attribute -->
      <a
        onclick={() => {
          openUrl(
            "https://www.paypal.com/donate/?hosted_button_id=4LVGM2LL7WFCQ"
          );
        }}
        title="https://www.paypal.com/donate/?hosted_button_id=4LVGM2LL7WFCQ"
      >
        anmutech PayPal
      </a>
    </div>
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
    display: grid;
    grid-template-rows: auto auto auto;
    gap: 2rem;
    overflow: auto;
    padding: 1rem;
    text-align: center;
    box-shadow: var(--box-shadow);
  }

  h2 {
    margin: 0;
    padding-bottom: 0.5rem;
  }

  .row {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: 1rem;
  }

  .row div {
    align-content: center;
  }

  .left-col {
    justify-self: right;
    text-align: right;
  }

  .right-col {
    grid-column-start: 2;
    text-align: left;
  }

  .general {
    display: grid;
    grid-template-rows: auto auto auto auto;
    gap: 1rem;
    margin: 0 auto;
  }

  .files {
    display: grid;
    grid-template-rows: auto auto auto auto auto;
    gap: 1rem;
    margin: 0 auto;
  }

  .files .row {
    grid-template-columns: 2fr 7fr;
  }

  .tools {
    display: grid;
    grid-template-rows: auto auto auto auto;
    gap: 1rem;
    margin: 0 auto;
  }
</style>
