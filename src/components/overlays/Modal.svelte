<script lang="ts">
  import type { Snippet } from "svelte";
  import { closeContextMenu } from "../../actions.svelte";

  let {
    visible,
    close_callback,
    children,
  }: {
    visible: boolean;
    close_callback: () => void;
    children: Snippet;
  } = $props();

  /**
   * TODO:
   * tried to use dialog, but closedby="any" does not work correctly.
   * Even closedby="none", which should only allow closing via JS, gets closed by pressing Escape.
   *
   * Additionally there were problems using context menu on top of dialog.
   * For now switched to this workaround.
   */

  function prevent_outer(e: Event) {
    e.stopPropagation();
    closeContextMenu();
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div onclick={close_callback} class="outer">
    <div onclick={prevent_outer} class="inner">
      {@render children()}
    </div>
  </div>
{/if}

<style>
  .outer {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(1, 1, 1, 0.1);
    border-radius: var(--radius-app);
    z-index: 1;
  }
</style>
