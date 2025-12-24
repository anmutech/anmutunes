<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { audio_state } from "../../state.svelte";
  import { FastForward, Pause, Play, Rewind } from "@lucide/svelte";

  function playpauseClick() {
    // TODO: debounce buttons?

    if (audio_state.is_playing) {
      //audio_state.is_playing = false;
      invoke("audiorequest", { request: { PlayPause: false } });
    } else {
      //audio_state.is_playing = true;
      invoke("audiorequest", { request: { PlayPause: true } });
    }
  }

  function volumeChange(event: Event) {
    if (event.target) {
      let volume = Number((event.target as HTMLInputElement).value);
      invoke("audiorequest", { request: { Volume: volume } });
    }
  }

  let previous = false;
  let timeout: any;

  function prev() {
    if (previous) {
      clearTimeout(timeout);
      invoke("audiorequest", { request: "Prev" });
    } else {
      invoke("audiorequest", { request: { Seek: 0 } });
      previous = true;
    }
    timeout = setTimeout(() => {
      previous = false;
    }, 2000);
  }

  function next() {
    invoke("audiorequest", { request: "Next" });
  }
</script>

<div class="controls-left" data-tauri-drag-region>
  <div class="control">
    <button class="no-bg" onclick={prev}>
      <Rewind color={"var(--icons)"} />
    </button>
    <button class="no-bg" onclick={playpauseClick}>
      {#if audio_state.is_playing}
        <Pause color={"var(--icons)"} />
      {:else}
        <Play color={"var(--icons)"} />
      {/if}
    </button>
    <button class="no-bg" onclick={next}>
      <FastForward color={"var(--icons)"} />
    </button>
  </div>
  <div class="control">
    <input
      type="range"
      name=""
      id=""
      min="0"
      max="100"
      value={audio_state.volume}
      onchange={volumeChange}
    />
  </div>
</div>

<style>
  .controls-left {
    display: grid;
    grid-template-columns: auto auto;
    width: 100%;
    align-content: center;
    justify-content: space-evenly;
  }

  .control {
    display: flex;
    gap: 0.5rem;
  }
</style>
