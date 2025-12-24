<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { app_state, data, searchModalState } from "../../state.svelte";
  import { DataType, type Search } from "../../defs";
  import SearchResult from "./SearchResult.svelte";
  import Modal from "./Modal.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import Spaced from "../Spaced.svelte";

  function search(event: KeyboardEvent) {
    if (event.target) {
      console.log(event);
      if (event.code == "Escape") {
        if (searchModalState.searchterm === "") {
          searchModalState.visible = false;
        }
        (event.target as HTMLInputElement).value = "";
      }
      searchModalState.searchterm = (event.target as HTMLInputElement).value;

      if (searchModalState.searchterm !== "") {
        invoke("dbrequest", {
          request: {
            Search: [
              searchModalState.searchterm,
              searchModalState.datatypes,
              null,
            ],
          },
        });
      } else {
        data.search = {} as Search;
      }
    }
  }

  function focus(element: HTMLInputElement) {
    element.focus();
    element.value = searchModalState.searchterm;
  }

  function toggleDatatype(type: DataType) {
    let index = searchModalState.datatypes.indexOf(type);
    if (index !== -1) {
      searchModalState.datatypes.splice(index, 1);
    } else {
      searchModalState.datatypes.push(type);
    }
  }

  let showResults = $derived.by(() => {
    if (
      (data.search.tracks &&
        data.search.tracks.length !== 0 &&
        searchModalState.datatypes.includes(DataType.Track)) ||
      (data.search.albums &&
        searchModalState.datatypes.includes(DataType.Album)) ||
      (data.search.artists &&
        searchModalState.datatypes.includes(DataType.Artist)) ||
      (data.search.composers &&
        searchModalState.datatypes.includes(DataType.Composer)) ||
      (data.search.genres &&
        searchModalState.datatypes.includes(DataType.Genre)) ||
      (data.search.playlists &&
        searchModalState.datatypes.includes(DataType.Playlist))
    ) {
      return true;
    }
    return false;
  });
</script>

<Modal
  visible={searchModalState.visible}
  close_callback={() => {
    searchModalState.visible = false;
  }}
>
  <div class="content">
    <div class="controls">
      <input type="search" placeholder="Suchen" onkeyup={search} use:focus />
      <Spaced equal={false} withBackground={false} withGap={true} wide={false}>
        <label for="tracks">
          <input
            type="checkbox"
            id="tracks"
            onclick={() => {
              toggleDatatype(DataType.Track);
            }}
            checked={searchModalState.datatypes.includes(DataType.Track)}
          />
          {translations.common.track_other}
        </label>
        <label for="albums">
          <input
            type="checkbox"
            id="albums"
            onclick={() => {
              toggleDatatype(DataType.Album);
            }}
            checked={searchModalState.datatypes.includes(DataType.Album)}
          />
          {translations.common.album_other}
        </label>
        <label for="artists">
          <input
            type="checkbox"
            id="artists"
            onclick={() => {
              toggleDatatype(DataType.Artist);
            }}
            checked={searchModalState.datatypes.includes(DataType.Artist)}
          />
          {translations.common.artist_other}
        </label>
        <label for="composers">
          <input
            type="checkbox"
            id="composers"
            onclick={() => {
              toggleDatatype(DataType.Composer);
            }}
            checked={searchModalState.datatypes.includes(DataType.Composer)}
          />
          {translations.common.composer_other}
        </label>
        <label for="genres">
          <input
            type="checkbox"
            id="genres"
            onclick={() => {
              toggleDatatype(DataType.Genre);
            }}
            checked={searchModalState.datatypes.includes(DataType.Genre)}
          />
          {translations.common.genre_other}
        </label>
        <label for="playlists">
          <input
            type="checkbox"
            id="playlists"
            onclick={() => {
              toggleDatatype(DataType.Playlist);
            }}
            checked={searchModalState.datatypes.includes(DataType.Playlist)}
          />
          {translations.common.playlist_other}
        </label>
      </Spaced>
    </div>
    {#key app_state.new_data}
      {#if showResults}
        <div class="results">
          {#if data.search.tracks && searchModalState.datatypes.includes(DataType.Track)}
            <div>
              <h4>{translations.common.track_other}</h4>
              {#each data.search.tracks as track_id}
                <SearchResult id={track_id} type={DataType.Track} />
              {/each}
            </div>
          {/if}
          {#if data.search.albums && searchModalState.datatypes.includes(DataType.Album)}
            <div>
              <h4>{translations.common.album_other}</h4>
              {#each data.search.albums as album_id}
                <SearchResult id={album_id} type={DataType.Album} />
              {/each}
            </div>
          {/if}
          {#if data.search.artists && searchModalState.datatypes.includes(DataType.Artist)}
            <div>
              <h4>{translations.common.artist_other}</h4>
              {#each data.search.artists as artist_id}
                <SearchResult id={artist_id} type={DataType.Artist} />
              {/each}
            </div>
          {/if}
          {#if data.search.composers && searchModalState.datatypes.includes(DataType.Composer)}
            <div>
              <h4>{translations.common.composer_other}</h4>
              {#each data.search.composers as composer_id}
                <SearchResult id={composer_id} type={DataType.Composer} />
              {/each}
            </div>
          {/if}
          {#if data.search.genres && searchModalState.datatypes.includes(DataType.Genre)}
            <div>
              <h4>{translations.common.genre_other}</h4>
              {#each data.search.genres as genre_id}
                <SearchResult id={genre_id} type={DataType.Genre} />
              {/each}
            </div>
          {/if}
          {#if data.search.playlists && searchModalState.datatypes.includes(DataType.Playlist)}
            <div>
              <h4>{translations.common.playlist_other}</h4>
              {#each data.search.playlists as playlist_id}
                <SearchResult id={playlist_id} type={DataType.Playlist} />
              {/each}
            </div>
          {/if}
        </div>
      {/if}
    {/key}
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
    max-width: calc(100% - 2rem);
    max-height: calc(
      100dvh - (var(--full-header-height) + var(--footer-height) + 2rem)
    );
    width: max-content;
    display: grid;
    grid-auto-flow: row;
    gap: 0.5rem;
    overflow: hidden;
    padding: 1rem;
    box-shadow: var(--box-shadow);
  }

  .controls {
    display: grid;
    grid-template-rows: 1fr auto;
    gap: 0.5rem;
  }

  input[type="search"] {
    padding: 1rem;
  }

  label {
    cursor: pointer;
  }

  .results {
    display: grid;
    max-height: calc(
      100dvh -
        (var(--full-header-height) + 1rem + var(--footer-height) + 1rem + 7rem)
    );
    overflow: auto;
    gap: 0.5rem;
    padding-bottom: 1rem;
  }

  .results div {
    display: grid;
    gap: 0.2rem;
  }

  h4 {
    margin: 0 7px;
    cursor: default;
  }
</style>
