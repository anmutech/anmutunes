<script lang="ts">
  import {
    ContextView,
    data,
    playlistSelectModalState,
  } from "../../state.svelte";
  import { create_playlist, update_playlist } from "../../actions.svelte";
  import { type Playlist, type Track } from "../../defs";
  import Sortable from "../Sortable/Sortable.svelte";
  import Select from "../Select.svelte";
  import Modal from "./Modal.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import Spaced from "../Spaced.svelte";

  let playlist_options = $derived.by(() => {
    let options: {
      name: string;
      value: number;
      playlist: Playlist | undefined;
    }[] = [
      {
        name: translations.playlistselect.createnewplaylist,
        value: 0,
        playlist: undefined,
      },
    ];

    data.playlists_order.ids.forEach((id) => {
      let playlist = data.playlists.get(id);
      if (playlist) {
        options.push({ value: id, name: playlist.name, playlist });
      }
    });

    return options;
  });

  let selected_playlist: number = $state(0);
  let track_ids = $derived(playlistSelectModalState.track_ids);
  let new_name: HTMLInputElement | undefined = $state();

  function save() {
    if (selected_playlist === 0) {
      if (new_name) {
        let playlist: Playlist = {
          id: 0,
          name: new_name?.value,
          description: "",
          tracks: track_ids,
        };

        create_playlist(playlist);
      }
    } else {
      let playlist = playlist_options[selected_playlist].playlist;
      if (playlist) {
        playlist.tracks.push(...track_ids);
        update_playlist(playlist);
      }
    }
    playlistSelectModalState.visible = false;
  }

  function close() {
    playlistSelectModalState.visible = false;
  }

  function finalize_callback(items: { id: number; track: Track }[]) {
    let new_track_ids = [];

    for (let i = 0; i < items.length; i++) {
      new_track_ids.push(items[i].track.id);
    }

    playlistSelectModalState.track_ids = new_track_ids;
  }

  function remove_callback(item: { id: number; track: Track }) {
    track_ids.splice(item.id, 1);
  }
</script>

<Modal visible={playlistSelectModalState.visible} close_callback={close}>
  <div class="content">
    <h3>{translations.contextmenu.addplaylist}</h3>
    <div class="container">
      <div class="fields centered">
        <Select
          active={playlist_options[selected_playlist].name}
          options={playlist_options}
          select_value={(value: number) => {
            for (let i = 0; i < playlist_options.length; i++) {
              if (playlist_options[i].value === value) {
                selected_playlist = i;
                break;
              }
            }
          }}
        />
      </div>
      {#if selected_playlist === 0}
        <div class="fields">
          <div class="field">
            <label for="name"
              >{translations.playlistselect.namenewplaylist}:</label
            >
            <input type="text" id="name" bind:this={new_name} />
          </div>
        </div>
      {/if}
      <div class="fields">
        <div class="field">
          <div class="sortable-list">
            <Sortable
              {track_ids}
              enable_playing={false}
              enable_remove={true}
              contextview={ContextView.None}
              onclick_callback={() => {}}
              {finalize_callback}
              {remove_callback}
            />
          </div>
        </div>
      </div>
      <Spaced equal={true} withBackground={false} withGap={true} wide={true}>
        <button onclick={save}>{translations.common.save}</button>
        <button onclick={close}>{translations.common.abort}</button>
      </Spaced>
    </div>
  </div>
</Modal>

<style>
  .content {
    position: fixed;
    top: calc(var(--full-header-height) + 1rem);
    left: 50%;
    transform: translateX(-50%);
    z-index: 1;
    border: var(--border);
    border-radius: var(--radius-small);
    background-color: var(--background);
    width: 40%;
    max-height: calc(
      100dvh - (var(--full-header-height) + var(--footer-height) + 2rem)
    );
    overflow: auto;
    padding: 1rem;
    box-shadow: var(--box-shadow);
  }

  h3 {
    margin-top: 0;
    text-align: center;
  }

  .container {
    display: grid;
    grid-template-rows: repeat(auto);
    gap: 1rem;
  }

  .fields {
    display: grid;
    grid-template-rows: repeat(auto);
    gap: 1rem;
  }

  .fields.centered {
    justify-items: center;
  }

  .field {
    display: grid;
    grid-template-columns: 1fr 2fr;
    gap: 0.5rem;
    text-align: center;
    align-content: center;
  }

  .field .sortable-list {
    grid-column: span 2;
  }

  .sortable-list {
    overflow: auto;
    width: 100%;
    max-height: 30rem;
    background-color: var(--background-active);
    color: var(--text);
    border-radius: var(--radius-small);
    border: none;
    cursor: pointer;
    margin: 0;
  }

  /*.controls {
    display: grid;
    grid-template-columns: 1fr 1fr;
  }

  .controls button {
    align-items: center;
  }*/
</style>
