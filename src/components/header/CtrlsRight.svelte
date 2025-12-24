<script lang="ts">
  import { queueModalState, searchModalState } from "../../state.svelte";
  import { ListMusic, Search } from "@lucide/svelte";

  const toggleQueueModal = (event: any) => {
    const buttonRect = event.target.getBoundingClientRect();
    queueModalState.visible = !queueModalState.visible;
    queueModalState.position = (buttonRect.left + buttonRect.right) / 2;
  };

  const toggleSearchModal = () => {
    searchModalState.visible = !searchModalState.visible;
  };
</script>

<div class="controls-right" data-tauri-drag-region>
  <div class="control">
    <button class="no-bg" onclick={toggleQueueModal}>
      <ListMusic color={"var(--icons)"} />
    </button>
  </div>
  <div class="control">
    <button class="no-bg" onclick={toggleSearchModal}>
      <Search color={"var(--icons)"} />
    </button>
  </div>
</div>

<style>
  .controls-right {
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
