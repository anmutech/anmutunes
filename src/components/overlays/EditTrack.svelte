<script lang="ts">
  import { translations } from "../../localisation/localisation.svelte";
  import { data, editModalState } from "../../state.svelte";
  //import { ChevronLeft, ChevronRight } from "@lucide/svelte";
  import Spaced from "../Spaced.svelte";
  import { update_tracks } from "../../actions.svelte";

  let { id }: { id: number } = $props();
  let track = $derived(data.tracks.get(id));
  let album = $derived(data.albums.get(track?.album_id as number));
  let artist = $derived(data.artists.get(track?.artist_id as number));
  let genre = $derived(data.genres.get(track?.genre_id as number));

  /**
   * TODO:
   * implement way to use prev/next to edit multiple tracks
   * implement suggestions for artist_name, album_name, genre_name
   */

  /*function prev() {
    if (track && album) {
      let index = album.tracks.indexOf(track.id);
      if (index > 0) {
        //save(); TODO: store changed until modal is closed with save()
        editModalState.id = album.tracks[index - 1];
      }
    }
  }

  function next() {
    if (track && album) {
      let index = album.tracks.indexOf(track.id);
      if (index !== -1 && index < album.tracks.length - 1) {
        //save(); TODO: store changed until modal is closed with save()
        editModalState.id = album.tracks[index + 1];
      }
    }
  }*/

  // TODO: implement way to edit location as well?
  let name: HTMLInputElement | undefined = $state();
  let artist_name: HTMLInputElement | undefined = $state();
  let album_name: HTMLInputElement | undefined = $state();
  let disc_number: HTMLInputElement | undefined = $state();
  let track_number: HTMLInputElement | undefined = $state();
  let genre_name: HTMLInputElement | undefined = $state();

  function save() {
    if (track) {
      track.name = name ? name.value : track.name;
      track.disc_number = disc_number
        ? disc_number.valueAsNumber
        : track.disc_number;
      track.track_number = track_number
        ? track_number.valueAsNumber
        : track.track_number;

      let artistName = artist ? artist.name : "";
      if (artist_name) {
        artistName = artist_name.value;
      }

      let albumName = album ? album.name : "";
      if (album_name) {
        albumName = album_name.value;
      }

      let genreName = genre ? genre.name : "";
      if (genre_name) {
        genreName = genre_name.value;
      }

      update_tracks([track], [artistName], [albumName], [genreName]);
    }
    close();
  }

  function close() {
    editModalState.visible = false;
  }
</script>

{#if track}
  <h3>{translations.edit.edittrack}</h3>
  <div class="container">
    <div class="track">
      <div class="row">
        <label for="artist">{translations.common.title}:</label>
        <input type="text" id="artist" value={track.name} bind:this={name} />
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
        <label for="title">{translations.common.album_one}:</label>
        <input
          type="text"
          id="name"
          value={album?.name}
          bind:this={album_name}
        />
      </div>
      <div class="row">
        <label for="title">{translations.common.cdnumber_long}:</label>
        <input
          type="number"
          id="name"
          value={track.disc_number}
          bind:this={disc_number}
        />
      </div>
      <div class="row">
        <label for="title">{translations.common.tracknumber_long}:</label>
        <input
          type="number"
          id="name"
          value={track.track_number}
          bind:this={track_number}
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
    </div>
    <Spaced equal={true} withBackground={false} withGap={true} wide={true}>
      <!--<button>
        <ChevronLeft onclick={prev} />
      </button>
      <button>
        <ChevronRight onclick={next} />
      </button>-->
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

  .track {
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
