<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    audio_state,
    ContextView,
    queueModalState,
  } from "../../state.svelte";
  import type { Track } from "../../defs";
  import Sortable from "../Sortable/Sortable.svelte";
  import Unsortable from "../Sortable/Unsortable.svelte";
  import { update_queue } from "../../actions.svelte";
  import Modal from "./Modal.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import Spaced from "../Spaced.svelte";

  function finalize_queue_callback(items: { id: number; track: Track }[]) {
    let new_queue = [];

    for (let i = 0; i < items.length; i++) {
      const item = items[i];
      new_queue.push(item.track.id);
    }

    update_queue(new_queue);
  }

  function remove_queue_callback(item: { id: number; track: Track }) {
    invoke("audiorequest", { request: { QueueRemove: [item.id] } });
  }

  function onclick_queue_callback(item: { id: number; track: Track }) {
    invoke("audiorequest", { request: { QueueJump: item.id } });
  }

  function onclick_history_callback(item: { id: number; track: Track }) {
    // item id is inverse to actual index in history
    let index = audio_state.history.length - item.id - 1;
    invoke("audiorequest", { request: { HistoryJump: index } });
  }

  function empty(e: Event) {
    e.stopPropagation();
    if (queueModalState.queue_hist) {
      invoke("audiorequest", {
        request: { QueueRemove: [...audio_state.queue.keys()] },
      });
    } else {
      invoke("audiorequest", {
        request: "HistoryRemove",
      });
    }
  }
</script>

<Modal
  visible={queueModalState.visible}
  close_callback={() => {
    queueModalState.visible = false;
  }}
>
  <div
    class="modal-overlay modal-content"
    style="left: {queueModalState.position}px;"
  >
    <div class="queuebuttons">
      <Spaced equal={true} withBackground={true} withGap={false} wide={true}>
        <button
          class={queueModalState.queue_hist ? "active" : ""}
          onclick={() => {
            queueModalState.queue_hist = true;
          }}
        >
          {translations.queue.queue}
        </button>
        <button
          class={queueModalState.queue_hist ? "" : "active"}
          onclick={() => {
            queueModalState.queue_hist = false;
          }}
        >
          {translations.queue.history}
        </button>
      </Spaced>
      {#if (queueModalState.queue_hist && audio_state.queue.length != 0) || (!queueModalState.queue_hist && audio_state.history.length != 0)}
        <Spaced equal={true} withBackground={false} withGap={true} wide={true}>
          <button onclick={empty} class="double"
            >{translations.common.emptylist}</button
          >
        </Spaced>
      {/if}
    </div>
    {#if queueModalState.queue_hist}
      {#if audio_state.queue.length == 0}
        <div class="modal-item-empty">{translations.common.listisempty}</div>
      {:else}
        {#key audio_state.queue}
          <div class="list">
            <Sortable
              track_ids={audio_state.queue}
              finalize_callback={finalize_queue_callback}
              enable_playing={false}
              enable_remove={true}
              remove_callback={remove_queue_callback}
              onclick_callback={onclick_queue_callback}
              contextview={ContextView.Queue}
            />
          </div>
        {/key}
      {/if}
    {:else if audio_state.history.length == 0}
      <div class="modal-item-empty">{translations.common.listisempty}</div>
    {:else}
      {#key audio_state.history}
        <div class="list">
          <Unsortable
            track_ids={audio_state.history.toReversed()}
            enable_playing={false}
            onclick_callback={onclick_history_callback}
            contextview={ContextView.History}
          />
        </div>
      {/key}
    {/if}
  </div>
</Modal>

<style>
  .modal-overlay {
    position: fixed;
    top: calc(var(--full-header-height) + 1rem);
    transform: translateX(-50%);
    z-index: 1;
    border: var(--border);
    border-radius: var(--radius-small);
    background-color: var(--background);
    width: 30vw; /* TODO: requires calculation depending on window size and left value */
    max-height: calc(
      100dvh - (var(--full-header-height) + var(--footer-height) + 2rem)
    );
    overflow-y: auto;
    box-shadow: var(--box-shadow);
  }

  .modal-item-empty {
    display: block;
    padding: 8px 16px;
    width: 100%;
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--text-dim);
    cursor: default;
  }

  .queuebuttons {
    display: grid;
    grid-auto-rows: auto;
    gap: 0.5rem;
    padding: 0.5rem;
  }

  button.double:hover {
    color: var(--warn);
  }
</style>
