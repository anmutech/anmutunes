<script lang="ts">
  import {
    initState,
    queueModalState,
    viewState,
    app_state,
    generateRows,
    data,
    importModalState,
    mediaPathModalState,
    config_state,
    db,
  } from "../state.svelte";
  import Header from "../components/header/Header.svelte";
  import Recents from "../components/Views/Recents/Recents.svelte";
  import Footer from "../components/footer.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import ContextMenu from "../components/overlays/ContextMenu.svelte";
  import Queue from "../components/overlays/Queue.svelte";
  import Search from "../components/overlays/Search.svelte";
  import Edit from "../components/overlays/Edit.svelte";
  import { ActiveView, Theme } from "../defs";
  import Albums from "../components/Views/Albums/Albums.svelte";
  import Tracks from "../components/Views/Tracks/Tracks.svelte";
  import { onMount } from "svelte";
  import SideBySide from "../components/Views/SideBySide/SideBySide.svelte";
  import Import from "../components/overlays/Import.svelte";
  import MediaPath from "../components/overlays/MediaPath.svelte";
  import PlaylistSelect from "../components/overlays/PlaylistSelect.svelte";
  import Settings from "../components/overlays/Settings.svelte";
  import Error from "../components/overlays/Error.svelte";
  import First from "../components/Views/First/First.svelte";
  import Loading from "../components/Views/Loading.svelte";
  import {
    changeSystemTheme,
    closeContextMenu,
    set_custom_colors,
  } from "../actions.svelte";
  import ColorEdit from "../components/overlays/ColorEdit.svelte";
  import {
    changeLanguage,
    initI18N,
  } from "../localisation/localisation.svelte";
  import Delete from "../components/overlays/Delete.svelte";
  import Empty from "../components/Views/Empty.svelte";
  import Notification from "../components/overlays/Notification.svelte";
  import Tutorial from "../components/overlays/Tutorial.svelte";

  let main: HTMLElement | undefined = $state();

  initI18N();

  listen("tauri://drag-drop", (event) => {
    const payload = event.payload as { paths: string[] };

    console.log("Payload: ", payload);

    if (config_state.is_new) {
      config_state.media_path = payload.paths[0];
    } else if (mediaPathModalState.visible) {
      mediaPathModalState.path = payload.paths[0];
    } else {
      if (payload.paths.length == 1 && payload.paths[0].endsWith(".xml")) {
        importModalState.visible = true;
        importModalState.path = payload.paths[0];
      } else {
        invoke("dbrequest", {
          request: {
            AddToLibrary: payload.paths,
          },
        });
      }
    }
  });

  initState();

  // Webview on Windows has issue where initState is not received by backend
  setTimeout(initState, 1000);

  function handleResize() {
    if (main) {
      // TODO: better calculation?
      let prev_albums = app_state.max_columns_albums;
      let prev_tracks = app_state.max_columns_tracks;

      app_state.max_columns_albums = Math.floor((main.clientWidth - 20) / 235);
      app_state.max_columns_tracks = Math.floor((main.clientWidth - 400) / 500);

      // Only update if actually changed
      if (
        prev_albums !== app_state.max_columns_albums ||
        prev_tracks !== app_state.max_columns_tracks
      ) {
        app_state.resized += 1;
        document.documentElement.style.setProperty(
          "--max-columns-albums",
          String(app_state.max_columns_albums)
        );
        generateRows();
        queueModalState.visible = false;
      }
    }
  }

  window.addEventListener("resize", handleResize);

  onMount(() => {
    handleResize();
  });

  $effect(() => {
    switch (config_state.theme) {
      case Theme.System:
        changeSystemTheme();
        break;
      case Theme.Dark:
        document.documentElement.setAttribute("data-theme", "dark");
        break;
      case Theme.Light:
        document.documentElement.setAttribute("data-theme", "light");
        break;
      case Theme.Custom:
        document.documentElement.setAttribute("data-theme", "custom");
        set_custom_colors();
      default:
        break;
    }

    if (config_state.language) {
      changeLanguage();
    }
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
{#key app_state.resized}
  <main bind:this={main} onclick={closeContextMenu}>
    <Header --grid-area="header" />
    <Queue />
    <Search />
    <ContextMenu />
    <Edit />
    <Import />
    <PlaylistSelect />
    <Settings />
    <MediaPath />
    <ColorEdit />
    <Tutorial />
    <Error />
    <Delete />
    <Notification />
    {#if viewState.view == ActiveView.Loading}
      <Loading --size="70mm" />
    {:else if viewState.view == ActiveView.First}
      <First />
    {:else if db.tracks_max == 0}
      <Empty />
    {:else if viewState.view == ActiveView.Recents}
      {#key app_state.new_albums}
        <Recents --grid-area="content" />
      {/key}
    {:else if viewState.view == ActiveView.Tracks}
      {#key app_state.new_tracks}
        <Tracks --grid-area="content" track_ids={data.tracks_order.ids} />
      {/key}
    {:else if viewState.view == ActiveView.Albums}
      {#key app_state.new_albums}
        <Albums --grid-area="content" />
      {/key}
    {:else}
      <SideBySide --grid-area="content" />
    {/if}
    <Footer --grid-area="footer" />
  </main>
{/key}

<style>
  main {
    height: 100%;
    display: grid;
    grid-template-areas:
      "header"
      "content"
      "footer";
    grid-template-rows:
      var(--full-header-height)
      auto var(--footer-height);
  }
</style>
