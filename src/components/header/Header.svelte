<script lang="ts">
  import CtrlsRight from "./CtrlsRight.svelte";
  import CtrlsLeft from "./CtrlsLeft.svelte";
  import CenterContent from "./CenterContent.svelte";
  import ViewSelect from "./ViewSelect.svelte";
  import { app_state } from "../../state.svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import X from "../../graphics/x.svelte";
  import Max from "../../graphics/max.svelte";
  import Min from "../../graphics/min.svelte";

  const appWindow = getCurrentWindow();

  async function toggleMaximize() {
    console.log("toggleMaximize");
    if (await appWindow.isMaximized()) {
      console.log("unmax");
      appWindow.unmaximize();
    } else {
      console.log("max");
      appWindow.maximize();
    }
  }
</script>

<nav>
  <CtrlsLeft />
  {#key app_state.new_data}
    <CenterContent />
  {/key}
  <CtrlsRight />
  <ViewSelect />
</nav>
<div class="controls">
  <button class="no-bg" onclick={appWindow.minimize} title="minimize">
    <Min />
  </button>
  <button class="no-bg" onclick={toggleMaximize} title="maximize">
    <Max />
  </button>
  <button class="no-bg" onclick={appWindow.close} title="close">
    <X />
  </button>
</div>

<style>
  nav {
    display: grid;
    grid-template-columns: 1fr 2fr 1fr;
    grid-template-rows: var(--header-height) var(--viewselect-height);
    gap: var(--header-gap);
    margin-top: var(--header-gap);
    height: var(--full-header-height);
  }

  .controls {
    display: flex;
    position: fixed;
    top: 0;
    right: 0;
  }

  .controls button {
    width: var(--header-controls);
    height: var(--header-controls);
    padding: 8px;
  }

  .controls button:last-child {
    /* Ensures that the button does not overflow on hover */
    border-radius: var(--radius-small) var(--radius-app) var(--radius-small)
      var(--radius-small);
  }
</style>
