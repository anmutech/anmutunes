<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    children,
    equal,
    withBackground,
    withGap,
    wide,
  }: {
    children: Snippet;
    equal: boolean;
    withBackground: boolean;
    withGap: boolean;
    wide: boolean;
  } = $props();

  let className = $state("container");

  if (equal) {
    className += " equal";
  }

  if (withBackground) {
    className += " background";
  }

  if (withGap) {
    className += " gap";
  }
</script>

{#if wide}
  <div class={className}>
    {@render children()}
  </div>
{:else}
  <div class="notwide">
    <div class={className}>
      {@render children()}
    </div>
  </div>
{/if}

<style>
  .notwide {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .container {
    display: grid;
    grid-auto-flow: column;
  }

  .equal {
    grid-auto-columns: 1fr;
  }

  .gap {
    gap: 0.5rem;
  }

  .background {
    background: var(--background-button);
    border-radius: var(--radius-small);
  }
</style>
