<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    app_state,
    audio_state,
    ContextView,
    data,
    millisecondsToReadableString,
    progress,
  } from "../../state.svelte";
  import {
    type Album,
    type Artist,
    type Cover,
    type Track,
    DataType,
    ProgressInfo,
    RepeatMode,
  } from "../../defs";
  import { Repeat, Repeat1, Shuffle } from "@lucide/svelte";
  import { buildContextMenu, get_cover } from "../../actions.svelte";
  import Anmutunes from "../../graphics/anmutunes.svelte";
  import Notes from "../../graphics/notes.svelte";
  import Loading from "../Views/Loading.svelte";
  import { translations } from "../../localisation/localisation.svelte";

  function positionClick(event: Event) {
    if (event.target) {
      let position = Number((event.target as HTMLInputElement).value);
      invoke("audiorequest", { request: { Seek: position } });
    }
  }

  function toggleShuffle() {
    invoke("audiorequest", { request: { Shuffle: !audio_state.shuffle_mode } });
  }

  function switchRepeat() {
    switch (audio_state.repeat_mode) {
      case RepeatMode.RepeatNone:
        invoke("audiorequest", { request: { Repeat: RepeatMode.RepeatQueue } });
        break;
      case RepeatMode.RepeatQueue:
        invoke("audiorequest", { request: { Repeat: RepeatMode.RepeatTrack } });
        break;
      case RepeatMode.RepeatTrack:
        invoke("audiorequest", { request: { Repeat: RepeatMode.RepeatNone } });
      default:
        break;
    }
  }

  const currentTrack: Track | undefined = $derived.by(() => {
    return data.tracks.get(audio_state.current_track);
  });
  const albumData: Album | undefined = $derived.by(() => {
    if (currentTrack) {
      return data.albums.get(currentTrack.album_id);
    }
    return undefined;
  });
  const artistData: Artist | undefined = $derived.by(() => {
    if (currentTrack) {
      return data.artists.get(currentTrack.artist_id);
    }
    return undefined;
  });
  let coverData: Cover | undefined = $derived.by(() => {
    if (albumData) {
      return get_cover(albumData.cover_id);
      //return data.covers.get(albumData.cover_id);
    }
    return undefined;
  });

  $effect(() => {
    if (app_state.new_covers && albumData) {
      coverData = get_cover(albumData.cover_id);
      //coverData = data.covers.get(albumData.cover_id);
    }
  });

  const total_time: number = $derived(
    currentTrack ? currentTrack.total_time : 1
  );

  function handleContextMenu(event: MouseEvent) {
    if (currentTrack) {
      buildContextMenu(event, DataType.Track, currentTrack.id, ContextView.Any);
    }
  }

  // TODO: deactivate value setting while user drags slider
  /*
      TODO: 
      display progress
      
      types:
        LibraryImport,
        FileImport,
        CoverExtract,

      export const progress = $state({
        active: false,
        data: {
          info: ProgressInfo.None,
          value: null,
          done: false,
        },
      });
       */

  $effect(() => {
    if (progress.active) {
      if (progress.data.done) {
        setTimeout(() => {
          progress.active = false;
        }, 2000);
      }
    }
  });

  let progressTexts = $derived.by(() => {
    switch (progress.data.info) {
      case ProgressInfo.CoverExtract:
        return {
          work: translations.centercontent.coverextract_work,
          done: translations.centercontent.coverextract_done,
        };
      case ProgressInfo.LibraryImport:
        return {
          work: translations.centercontent.libraryimport_work,
          done: translations.centercontent.libraryimport_done,
        };
      case ProgressInfo.FileImport:
        return {
          work: translations.centercontent.fileimport_work,
          done: translations.centercontent.fileimport_done,
        };
      case ProgressInfo.Delete:
        return {
          work: translations.centercontent.delete_work,
          done: translations.centercontent.delete_done,
        };
      case ProgressInfo.UpdateTracks:
        return {
          work: translations.centercontent.updatetracks_work,
          done: translations.centercontent.updatetracks_done,
        };
      case ProgressInfo.UpdateAlbum:
        return {
          work: translations.centercontent.updatealbum_work,
          done: translations.centercontent.updatealbum_done,
        };
      case ProgressInfo.None:
      default:
        return { work: "", done: "" };
    }
  });
</script>

<div class="center-content" class:playing={currentTrack != undefined}>
  {#if progress.active}
    <div class="progress">
      <Loading --size="100%" />
      <h4>
        {#if progress.data.done}
          {progressTexts.done}
        {:else}
          {progressTexts.work}
          {#if progress.data.value !== null}
            {"(" + progress.data.value + ")"}
          {/if}
        {/if}
      </h4>
    </div>
  {:else if currentTrack == undefined}
    <div class="logo" data-tauri-drag-region>
      <Anmutunes />
    </div>
  {:else}
    <div class="cover">
      {#if coverData}
        <img class="cover-img" src={coverData.data} alt="" />
      {:else}
        <Notes border={"none"} border_radius={"var(--radius-small)"} />
      {/if}
    </div>
    <div class="track-controls">
      <div class="controls-mid" data-tauri-drag-region>
        <div class="controls shuffle">
          <button class="no-bg" onclick={toggleShuffle}>
            <Shuffle
              color={audio_state.shuffle_mode
                ? "var(--icons)"
                : "var(--icons-dim)"}
            />
          </button>
          <div class="time">
            {millisecondsToReadableString(audio_state.position)}
          </div>
        </div>
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="track-info" oncontextmenu={handleContextMenu}>
          {#if currentTrack}
            <div>{currentTrack.name}</div>
            <div>
              {artistData?.name} - {albumData?.name}
            </div>
          {/if}
        </div>
        <div class="controls repeat">
          {#if audio_state.repeat_mode == RepeatMode.RepeatNone}
            <button class="no-bg" onclick={switchRepeat}>
              <Repeat color="var(--icons-dim)" />
            </button>
          {:else if audio_state.repeat_mode == RepeatMode.RepeatQueue}
            <button class="no-bg" onclick={switchRepeat}>
              <Repeat color="var(--icons)" />
            </button>
          {:else if audio_state.repeat_mode == RepeatMode.RepeatTrack}
            <button class="no-bg" onclick={switchRepeat}>
              <Repeat1 color="var(--icons)" />
            </button>
          {/if}
          <div class="time">
            {`-${millisecondsToReadableString(total_time - audio_state.position)}`}
          </div>
        </div>
      </div>
      <div class="scrub">
        <input
          type="range"
          min="0"
          max={total_time}
          value={audio_state.position}
          onclick={positionClick}
        />
      </div>
    </div>
  {/if}
</div>

<style>
  .center-content {
    width: 100%;
    height: 100%;
  }

  .playing {
    display: grid;
    grid-template-columns: var(--header-height) calc(
        100% - var(--header-height)
      );
  }

  .progress {
    display: grid;
    grid-template-columns: var(--header-height) auto;
    width: max-content;
    margin: auto;
  }

  .progress h4 {
    text-align: center;
    margin: auto;
  }

  .logo {
    width: 100%;
    height: var(--header-height);
    align-content: center;
  }

  .cover {
    height: var(--header-height);
    padding: 1px;
  }

  .cover-img {
    height: 100%;
    width: 100%;
    border-radius: var(--radius-small);
    background-color: var(--background-active);
  }

  .track-controls {
    display: grid;
    height: var(--header-height);
    grid-template-rows: 80% 20%;
  }

  .controls-mid {
    display: grid;
    grid-template-columns: auto auto auto;
    justify-content: space-between;
    gap: 1rem;
  }

  .track-info {
    /* 
    Dirty way of constraining width in px,
    which is required for ellipsis to work on contained divs
    See: https://stackoverflow.com/questions/17779293/css-text-overflow-ellipsis-not-working/52977310#52977310
    */
    width: calc(100%);
    margin: auto;
    overflow: hidden;
    cursor: pointer;
  }

  .track-info div {
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .scrub input {
    width: 100%;
  }

  .controls {
    display: grid;
    grid-template-rows: auto auto;
    gap: 1.2rem;
  }

  .controls button {
    width: fit-content;
  }

  .controls.repeat {
    justify-items: right;
  }

  .time {
    cursor: default;
  }
</style>
