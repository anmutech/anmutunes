<script lang="ts">
  import { translations } from "../../localisation/localisation.svelte";
</script>

<div class="main-content">
  <h2>{translations.emptyview.nothinghere}</h2>
  <h4>
    {translations.emptyview.importlibrary.first} <br />
    {translations.emptyview.importlibrary.second} <br />
    {translations.emptyview.importlibrary.third}
  </h4>
  <h4>
    {translations.emptyview.importfiles.first} <br />
    {translations.emptyview.importfiles.second} <br />
    {translations.emptyview.importfiles.third}
  </h4>
</div>

<style>
  .main-content {
    height: 100%;
    width: 100%;
    align-content: center;
    text-align: center;
  }
</style>
