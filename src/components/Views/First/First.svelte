<script lang="ts">
  import { ChevronLeft, ChevronRight } from "@lucide/svelte";
  import Setup from "./Setup.svelte";
  import Tutorial from "./Tutorial.svelte";
  import { set_config } from "../../../actions.svelte";
  import { config_state } from "../../../state.svelte";

  // Once the user dismissed this view, set is_new to false using set_config.

  let page = $state(0);

  function next() {
    if (page < 6) {
      page += 1;
    } else {
      config_state.is_new = false;
      set_config();
    }
  }

  function prev() {
    if (page > 0) {
      page -= 1;
    }
  }
</script>

<div class="content">
  {#if page < 6}
    <Setup {page} />
  {:else}
    <Tutorial />
  {/if}
  <div class="buttons">
    {#if page === 0}
      <div></div>
    {:else}
      <button onclick={prev}>
        <ChevronLeft size={50} />
      </button>
    {/if}
    <button onclick={next}><ChevronRight size={50} /></button>
  </div>
</div>

<style>
  .content {
    position: relative;
    display: grid;
    grid-template-rows: auto min-content;
    height: 100%;
    width: 100%;
    overflow: auto;
  }

  .buttons {
    position: absolute;
    bottom: 0;
    width: 100%;
    display: flex;
    justify-content: space-between;
    background: none;
  }
</style>
