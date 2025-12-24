<script lang="ts">
  import { data, editModalState } from "../../state.svelte";
  import { create_playlist, update_playlist } from "../../actions.svelte";
  import { type Playlist } from "../../defs";
  import { translations } from "../../localisation/localisation.svelte";
  import Spaced from "../Spaced.svelte";

  let { id }: { id: number } = $props();
  let playlist = $derived(data.playlists.get(id));

  let name: HTMLInputElement | undefined = $state();
  let description: HTMLTextAreaElement | undefined = $state();

  function save() {
    if (playlist) {
      if (name) {
        playlist.name = name.value;
        playlist.description = description
          ? description.value
          : playlist.description;
        update_playlist(playlist);
      }
    } else {
      if (name) {
        let new_playlist: Playlist = {
          id: 0,
          name: name.value,
          description: description ? description.value : "",
          tracks: [],
        };

        create_playlist(new_playlist);
      }
    }
    close();
  }

  function close() {
    editModalState.visible = false;
  }
</script>

<h3>
  {playlist ? translations.edit.editplaylist : translations.edit.newplaylist}
</h3>
<div class="container">
  <div class="playlist">
    <div class="row">
      <label for="name">{translations.common.name}:</label>
      <input
        type="text"
        id="name"
        value={playlist ? playlist.name : ""}
        bind:this={name}
      />
    </div>
    <div class="row">
      <label for="description">{translations.common.description}:</label>
      <textarea
        id="description"
        value={playlist ? playlist.description : ""}
        bind:this={description}
      ></textarea>
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

<style>
  h3 {
    margin-top: 0;
    text-align: center;
  }

  .container {
    display: grid;
    grid-template-rows: repeat(auto);
    gap: 1rem;
  }

  .playlist {
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
