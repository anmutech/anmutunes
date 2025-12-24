<script lang="ts">
  import { data, app_state } from "../../../state.svelte";
  import SvelteVirtualList from "@humanspeak/svelte-virtual-list";
  import Row from "./Row.svelte";
  import { translations } from "../../../localisation/localisation.svelte";

  function scroll() {
    // TODO: make this more conditional, only scroll to album/split if required/out of view.
    console.log("scroll");
    let index = 0;

    data.rows.forEach((row, i) => {
      row.forEach((entry) => {
        if (typeof entry === "number" && entry === app_state.open_split) {
          index = i;
        }
      });
    });

    if (listRef) {
      listRef.scroll({
        index: index,
        smoothScroll: false,
        align: "top",
      });
    }
  }

  let listRef = $state<SvelteVirtualList<number[] | string[]>>();
  setTimeout(scroll, 2000);
  /**
   * TODO:
   * store scroll position onDestroy so we can recover when the view is reopened.
   * For resizing we should store first album_id in view, only relevant if data.rows changes.
   *
   * If a split is open and in view, we should ignore the first album_id on resize and use the split
   *
   * Better way than using scroll to get back to previous position?
   */

  // not precise since we have header and album rows, but prevents listRef.scroll from stopping wrong
  let rowHeightEstimate = 260;

  function title_translation(title: string) {
    switch (title) {
      case "today":
        return translations.sections.today;
      case "lastweek":
        return translations.sections.lastweek;
      case "lastmonth":
        return translations.sections.lastmonth;
      case "last3months":
        return translations.sections.lastthreemonths;
      case "last6months":
        return translations.sections.lastsixmonths;
      case "thisyear":
        return translations.sections.thisyear;
      default:
        return title;
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="main-content"
  onclick={() => {
    app_state.open_split = -1;
  }}
>
  <SvelteVirtualList
    items={data.rows}
    defaultEstimatedItemHeight={rowHeightEstimate}
    bind:this={listRef}
  >
    {#snippet renderItem(item, index)}
      {#key app_state.max_columns_albums}
        {#if typeof item[0] === "string"}
          <h2
            class={index === 0
              ? "section-heading"
              : "section-heading not-first"}
          >
            {title_translation(item[0])}
          </h2>
        {:else}
          <Row albums={item as number[]}></Row>
        {/if}
      {/key}
    {/snippet}
  </SvelteVirtualList>
</div>

<style>
  .main-content {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
  }

  .section-heading {
    position: relative;
    padding: 0 2rem;
  }

  .section-heading.not-first {
    padding-top: 30px;
  }

  .section-heading.not-first::before {
    content: "";
    position: absolute;
    top: 0;
    left: 50%;
    width: calc(100% - 4rem);
    height: 1px;
    background-color: var(--border-color);
    transform: translateX(-50%);
  }
</style>
