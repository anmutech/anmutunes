<script lang="ts">
  import List from "./List.svelte";
  import Content from "./Content.svelte";
</script>

<div class="main-content">
  <List />
  <Content />
</div>

<style>
  .main-content {
    display: grid;
    grid-template-columns: 1fr 3fr;
    height: 100%;
    width: 100%;
  }
</style>
