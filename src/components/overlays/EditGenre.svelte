<script lang="ts">
  import { update_genre } from "../../actions.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import { data, editModalState } from "../../state.svelte";
  import Spaced from "../Spaced.svelte";

  let { id }: { id: number } = $props();
  let genre = $derived(data.genres.get(id));

  let name: HTMLInputElement | undefined = $state();

  function save() {
    if (genre) {
      genre.name = name ? name.value : genre.name;
      update_genre(genre);
    }
    close();
  }

  function close() {
    editModalState.visible = false;
  }
</script>

{#if genre}
  <h3>{translations.edit.editgenre}</h3>
  <div class="container">
    <div class="genre">
      <div class="row">
        <label for="title">{translations.common.genre_one}:</label>
        <input type="text" id="name" value={genre.name} bind:this={name} />
      </div>
    </div>
    <Spaced equal={true} withBackground={false} withGap={true} wide={true}>
      <button onclick={save}>
        {translations.common.save}
      </button>
      <button onclick={close}>
        {translations.common.abort}
      </button>
    </Spaced>
  </div>
{/if}

<style>
  h3 {
    margin-top: 0;
    text-align: center;
  }

  .container {
    display: grid;
    grid-template-rows: auto 2rem;
    gap: 1rem;
  }

  .genre {
    display: grid;
    grid-template-rows: repeat(auto);
    gap: 1rem;
  }

  .row {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: 1rem;
  }

  .row label {
    align-content: center;
    text-align: right;
  }
</style>
