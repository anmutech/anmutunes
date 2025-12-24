<script lang="ts">
  let {
    active,
    options,
    select_value,
  }: {
    active: string;
    options: { name: string; value: any }[];
    select_value: (option: any) => void;
  } = $props();

  let select_open = $state(false);
</script>

<div class="select">
  <button
    class="select-button"
    onclick={() => {
      select_open = !select_open;
    }}
  >
    {active}
  </button>
  {#if select_open}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="outer"
      onclick={() => {
        select_open = false;
      }}
    ></div>
    <div class="select-options">
      {#each options as option}
        <button
          onclick={() => {
            select_value(option.value);
            select_open = false;
          }}
        >
          {option.name}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .outer {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border-radius: var(--radius-app);
  }

  .select {
    position: relative;
    background: var(--background-button);
    border-radius: var(--radius-small);
    height: fit-content;
    width: fit-content;
  }

  .select-button {
    padding: 6px;
    width: 100%;
    justify-content: left;
    display: grid;
    grid-template-columns: auto 20px; /* Create space for after */
  }

  .select-button::after {
    position: absolute;
    content: "";
    top: calc(50% - 2.5px);
    right: 10px;
    border: 5px solid transparent;
    border-color: var(--text) transparent transparent transparent;
  }

  .select-options {
    position: absolute;
    top: 100%;
    width: fit-content;
    min-width: 100%;
    border-radius: var(--radius-small);
    background: var(--background-button);
    overflow: auto;
    max-height: calc(400%);
    z-index: 1;
    box-shadow: var(--box-shadow);
  }

  .select-options button {
    justify-content: left;
    width: 100%;
  }
</style>
