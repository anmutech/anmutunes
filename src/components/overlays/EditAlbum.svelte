<script lang="ts">
  import { update_album } from "../../actions.svelte";
  import { translations } from "../../localisation/localisation.svelte";
  import { data, editModalState } from "../../state.svelte";
  import Spaced from "../Spaced.svelte";

  let { id }: { id: number } = $props();
  let album = $derived(data.albums.get(id));
  let artist = $derived(data.artists.get(album?.artist_id as number));
  let genre = $derived(data.genres.get(album?.genre_id as number));

  // TODO: implement way to edit cover as well
  let name: HTMLInputElement | undefined = $state();
  let sort_album: HTMLInputElement | undefined = $state();
  let artist_name: HTMLInputElement | undefined = $state();
  let genre_name: HTMLInputElement | undefined = $state();
  let year: HTMLInputElement | undefined = $state();
  let release_date: HTMLInputElement | undefined = $state();

  function save() {
    if (album) {
      album.name = name ? name.value : album.name;
      album.sort_album = sort_album ? sort_album.value : album.sort_album;
      album.year = year ? year.valueAsNumber : album.year;
      album.release_date = release_date
        ? release_date.value
        : album.release_date;

      let artistName = artist ? artist.name : "";
      artistName = artist_name ? artist_name.value : artistName;

      let genreName = genre ? genre.name : "";
      genreName = genre_name ? genre_name.value : genreName;

      update_album(album, artistName, genreName);
    }
    close();
  }

  function close() {
    editModalState.visible = false;
  }
</script>

{#if album}
  <h3>{translations.edit.editalbum}</h3>
  <div class="container">
    <div class="album">
      <div class="row">
        <label for="title">{translations.common.title}:</label>
        <input type="text" id="name" value={album.name} bind:this={name} />
      </div>
      <div class="row">
        <label for="sort_album">{translations.edit.sort_album}:</label>
        <input
          type="text"
          id="sort_album"
          value={album.sort_album}
          bind:this={sort_album}
        />
      </div>
      <div class="row">
        <label for="artist">{translations.common.artist_one}:</label>
        <input
          type="text"
          id="artist"
          value={artist?.name}
          bind:this={artist_name}
        />
      </div>
      <div class="row">
        <label for="genre">{translations.common.genre_one}:</label>
        <input
          type="text"
          id="genre"
          value={genre?.name}
          bind:this={genre_name}
        />
      </div>
      <div class="row">
        <label for="year">{translations.common.year}:</label>
        <input type="number" id="year" value={album.year} bind:this={year} />
      </div>
      <div class="row">
        <label for="release_date">{translations.common.release_long}:</label>
        <input
          type="text"
          id="release_date"
          value={album.release_date}
          bind:this={release_date}
        />
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

  .album {
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
