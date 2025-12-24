import { locale } from "@tauri-apps/plugin-os";
import i18next, { type TFunction } from "i18next";
import LanguageDetector from "i18next-browser-languagedetector";
import { Language, type Translation } from "../defs";
import { config_state } from "../state.svelte";
import cs from "./locales/cs.json";
import da from "./locales/da.json";
import de from "./locales/de.json";
import el from "./locales/el.json";
import en from "./locales/en.json";
import es from "./locales/es.json";
import fr from "./locales/fr.json";
import it from "./locales/it.json";
import ja from "./locales/ja.json";
import ko from "./locales/ko.json";
import lb from "./locales/lb.json";
import nl from "./locales/nl.json";
import pl from "./locales/pl.json";
import pt from "./locales/pt.json";
import tr from "./locales/tr.json";
import zh from "./locales/zh.json";

export const translations: Translation = $state({
  common: {
    name: "",
    description: "",
    track_one: "",
    track_other: "",
    title: "",
    artist_one: "",
    artist_other: "",
    composer_one: "",
    composer_other: "",
    album_one: "",
    album_other: "",
    albumartist: "",
    genre_one: "",
    genre_other: "",
    cdnumber: "",
    cdnumber_long: "",
    tracknumber: "",
    tracknumber_long: "",
    tracklength: "",
    playlist_one: "",
    playlist_other: "",
    year: "",
    release: "",
    release_long: "",
    added: "",
    cover: "",
    emptylist: "",
    listisempty: "",
    newplaylist: "",
    mediapath: "",
    present: "",
    save: "",
    abort: "",
    yes: "",
    no: "",
    close: "",
    delete: "",
    system: "",
  },
  viewselect: {
    recents: "",
    tracks: "",
    albums: "",
    artists: "",
    composers: "",
    genres: "",
    playlists: "",
  },
  coloredit: {
    background: "",
    background_active: "",
    background_hover: "",
    background_button: "",
    border: "",
    accent: "",
    warn: "",
    text: "",
    text_dim: "",
    text_highlight: "",
    shadow: "",
  },
  contextmenu: {
    play: "",
    playrandom: "",
    playnext: "",
    addqueue: "",
    addplaylist: "",
    showartist: "",
    showgenre: "",
    showalbum: "",
    showcomposer: "",
    showplaylist: "",
    showlyrics: "",
    openpath: "",
    showsimilar: "",
    rating: "",
    info: "",
    edit: "",
    delete: "",
    extractcover: "",
  },
  edit: {
    cannotedit: "",
    editalbum: "",
    editartist: "",
    editcomposer: "",
    editgenre: "",
    editplaylist: "",
    newplaylist: "",
    edittrack: "",
    sort_album: "",
  },
  import: {
    importlibrary: "",
  },
  mediapath: {
    currentmediapath: "",
    draganddroppath: "",
    newmediapath: "",
    explain: "",
  },
  playlistselect: {
    createnewplaylist: "",
    namenewplaylist: "",
  },
  queue: {
    queue: "",
    history: "",
  },
  settings: {
    settings: "",
    general: "",
    playback: "",
    files: "",
    tools: "",
    theme: {
      theme: "",
      system: "",
      dark: "",
      light: "",
      custom: "",
      editcolors: "",
      resetlight: "",
      resetdark: "",
    },
    startupview: "",
    language: "",
    searchforupdates: "",
    mediapath: "",
    changemediapath: "",
    managefolders: "",
    managefolders_explain_active: "",
    managefolders_explain_inactive: "",
    copynotcopiedmedia: "",
    allowdelete: "",
    allowdelete_active: "",
    allowdelete_inactive: "",
    allowdeletefiles: "",
    allowdeletefiles_active: "",
    allowdeletefiles_inactive: "",
    extractcovers: "",
    extractcovers_explain: "",
    showmissingtracks: "",
    showmissingtracks_explain: "",
    standardisenames: "",
    standardisenames_explain: "",
    extractmetadata: "",
    extractmetadata_explain: "",
    support_please: "",
    updatesarehere: "",
  },
  setup: {
    setup: "",
    mediapath: "",
    mediapath_explain: "",
    mediapath_inactive_explain: "",
    searchforupdates_active_explain: "",
    searchforupdates_inactive_explain: "",
  },
  tutorial: {
    tutorial: "",
    ccmessage: "",
    attributions: "",
  },
  sections: {
    today: "",
    lastweek: "",
    lastmonth: "",
    lastthreemonths: "",
    lastsixmonths: "",
    thisyear: "",
  },
  times: {
    year_one: "",
    year_other: "",
    year_short: "",
    month_one: "",
    month_other: "",
    month_short: "",
    week_one: "",
    week_other: "",
    week_short: "",
    day_one: "",
    day_other: "",
    day_short: "",
    hour_one: "",
    hour_other: "",
    hour_short: "",
    minute_one: "",
    minute_other: "",
    minute_short: "",
    second_one: "",
    second_other: "",
    second_short: "",
  },
  centercontent: {
    coverextract_work: "",
    coverextract_done: "",
    libraryimport_work: "",
    libraryimport_done: "",
    fileimport_work: "",
    fileimport_done: "",
    delete_work: "",
    delete_done: "",
    updatetracks_work: "",
    updatetracks_done: "",
    updatealbum_work: "",
    updatealbum_done: "",
  },
  deletemodal: {
    albumsandtracks: "",
    artistandtracks: "",
    composerandtracks: "",
    genreandtracks: "",
    track: "",
    playlist: "",
    deleteincludingfiles: "",
  },
  notification: {
    libraryimport: {
      first: "",
      second: "",
      third: "",
    },
  },
  emptyview: {
    nothinghere: "",
    importlibrary: {
      first: "",
      second: "",
      third: "",
    },
    importfiles: {
      first: "",
      second: "",
      third: "",
    },
  },
  playlist: {
    cannotmodify1024: "",
  },
  llminfo: {
    long: "",
    short: "",
    notice: "",
  },
});

function fillTranslations(err: any, t: TFunction<"translation", undefined>) {
  console.log("filling translations");
  translations.common = {
    name: t("common.name"),
    description: t("common.description"),
    track_one: t("common.track_one"),
    track_other: t("common.track_other"),
    title: t("common.title"),
    artist_one: t("common.artist_one"),
    artist_other: t("common.artist_other"),
    composer_one: t("common.composer_one"),
    composer_other: t("common.composer_other"),
    album_one: t("common.album_one"),
    album_other: t("common.album_other"),
    albumartist: t("common.albumartist"),
    genre_one: t("common.genre_one"),
    genre_other: t("common.genre_other"),
    cdnumber: t("common.cdnumber"),
    cdnumber_long: t("common.cdnumber_long"),
    tracknumber: t("common.tracknumber"),
    tracknumber_long: t("common.tracknumber_long"),
    tracklength: t("common.tracklength"),
    playlist_one: t("common.playlist_one"),
    playlist_other: t("common.playlist_other"),
    year: t("common.year"),
    release: t("common.release"),
    release_long: t("common.release_long"),
    added: t("common.added"),
    cover: t("common.cover"),
    emptylist: t("common.emptylist"),
    listisempty: t("common.listisempty"),
    newplaylist: t("common.newplaylist"),
    mediapath: t("common.mediapath"),
    present: t("common.present"),
    save: t("common.save"),
    abort: t("common.abort"),
    yes: t("common.yes"),
    no: t("common.no"),
    close: t("common.close"),
    delete: t("common.delete"),
    system: t("common.system"),
  };
  translations.viewselect = {
    recents: t("viewselect.recents"),
    tracks: t("viewselect.tracks"),
    albums: t("viewselect.albums"),
    artists: t("viewselect.artists"),
    composers: t("viewselect.composers"),
    genres: t("viewselect.genres"),
    playlists: t("viewselect.playlists"),
  };
  translations.coloredit = {
    background: t("coloredit.background"),
    background_active: t("coloredit.background_active"),
    background_hover: t("coloredit.background_hover"),
    background_button: t("coloredit.background_button"),
    border: t("coloredit.border"),
    accent: t("coloredit.accent"),
    warn: t("coloredit.warn"),
    text: t("coloredit.text"),
    text_dim: t("coloredit.text_dim"),
    text_highlight: t("coloredit.text_highlight"),
    shadow: t("coloredit.shadow"),
  };
  translations.contextmenu = {
    play: t("contextmenu.play"),
    playrandom: t("contextmenu.playrandom"),
    playnext: t("contextmenu.playnext"),
    addqueue: t("contextmenu.addqueue"),
    addplaylist: t("contextmenu.addplaylist"),
    showartist: t("contextmenu.showartist"),
    showgenre: t("contextmenu.showgenre"),
    showalbum: t("contextmenu.showalbum"),
    showcomposer: t("contextmenu.showcomposer"),
    showplaylist: t("contextmenu.showplaylist"),
    showlyrics: t("contextmenu.showlyrics"),
    openpath: t("contextmenu.openpath"),
    showsimilar: t("contextmenu.showsimilar"),
    rating: t("contextmenu.rating"),
    info: t("contextmenu.info"),
    edit: t("contextmenu.edit"),
    delete: t("contextmenu.delete"),
    extractcover: t("contextmenu.extractcover"),
  };
  translations.edit = {
    cannotedit: t("edit.cannotedit"),
    editalbum: t("edit.editalbum"),
    editartist: t("edit.editartist"),
    editcomposer: t("edit.editcomposer"),
    editgenre: t("edit.editgenre"),
    editplaylist: t("edit.editplaylist"),
    newplaylist: t("edit.newplaylist"),
    edittrack: t("edit.edittrack"),
    sort_album: t("edit.sort_album"),
  };
  translations.import = {
    importlibrary: t("import.importlibrary"),
  };
  translations.mediapath = {
    currentmediapath: t("mediapath.currentmediapath"),
    draganddroppath: t("mediapath.draganddroppath"),
    newmediapath: t("mediapath.newmediapath"),
    explain: t("mediapath.explain"),
  };
  translations.playlistselect = {
    createnewplaylist: t("playlistselect.createnewplaylist"),
    namenewplaylist: t("playlistselect.namenewplaylist"),
  };
  translations.queue = {
    queue: t("queue.queue"),
    history: t("queue.history"),
  };
  translations.settings = {
    settings: t("settings.settings"),
    general: t("settings.general"),
    playback: t("settings.playback"),
    files: t("settings.files"),
    tools: t("settings.tools"),
    theme: {
      theme: t("settings.theme.theme"),
      system: t("settings.theme.system"),
      dark: t("settings.theme.dark"),
      light: t("settings.theme.light"),
      custom: t("settings.theme.custom"),
      editcolors: t("settings.theme.editcolors"),
      resetlight: t("settings.theme.resetlight"),
      resetdark: t("settings.theme.resetdark"),
    },
    startupview: t("settings.startupview"),
    language: t("settings.language"),
    searchforupdates: t("settings.searchforupdates"),
    mediapath: t("settings.mediapath"),
    changemediapath: t("settings.changemediapath"),
    managefolders: t("settings.managefolders"),
    managefolders_explain_active: t("settings.managefolders_explain_active"),
    managefolders_explain_inactive: t(
      "settings.managefolders_explain_inactive"
    ),
    copynotcopiedmedia: t("settings.copynotcopiedmedia"),
    allowdelete: t("settings.allowdelete"),
    allowdelete_active: t("settings.allowdelete_active"),
    allowdelete_inactive: t("settings.allowdelete_inactive"),
    allowdeletefiles: t("settings.allowdeletefiles"),
    allowdeletefiles_active: t("settings.allowdeletefiles_active"),
    allowdeletefiles_inactive: t("settings.allowdeletefiles_inactive"),
    extractcovers: t("settings.extractcovers"),
    extractcovers_explain: t("settings.extractcovers_explain"),
    showmissingtracks: t("settings.showmissingtracks"),
    showmissingtracks_explain: t("settings.showmissingtracks_explain"),
    standardisenames: t("settings.standardisenames"),
    standardisenames_explain: t("settings.standardisenames_explain"),
    extractmetadata: t("settings.extractmetadata"),
    extractmetadata_explain: t("settings.extractmetadata_explain"),
    support_please: t("settings.support_please"),
    updatesarehere: t("settings.updatesarehere"),
  };
  translations.setup = {
    setup: t("setup.setup"),
    mediapath: t("setup.mediapath"),
    mediapath_explain: t("setup.mediapath_explain"),
    mediapath_inactive_explain: t("setup.mediapath_inactive_explain"),
    searchforupdates_active_explain: t("setup.searchforupdates_active_explain"),
    searchforupdates_inactive_explain: t(
      "setup.searchforupdates_inactive_explain"
    ),
  };
  translations.tutorial = {
    tutorial: t("tutorial.tutorial"),
    ccmessage: t("tutorial.ccmessage"),
    attributions: t("tutorial.attributions"),
  };
  translations.sections = {
    today: t("sections.today"),
    lastweek: t("sections.lastweek"),
    lastmonth: t("sections.lastmonth"),
    lastthreemonths: t("sections.lastthreemonths"),
    lastsixmonths: t("sections.lastsixmonths"),
    thisyear: t("sections.thisyear"),
  };
  translations.times = {
    year_one: t("times.year_one"),
    year_other: t("times.year_other"),
    year_short: t("times.year_short"),
    month_one: t("times.month_one"),
    month_other: t("times.month_other"),
    month_short: t("times.month_short"),
    week_one: t("times.week_one"),
    week_other: t("times.week_other"),
    week_short: t("times.week_short"),
    day_one: t("times.day_one"),
    day_other: t("times.day_other"),
    day_short: t("times.day_short"),
    hour_one: t("times.hour_one"),
    hour_other: t("times.hour_other"),
    hour_short: t("times.hour_short"),
    minute_one: t("times.minute_one"),
    minute_other: t("times.minute_other"),
    minute_short: t("times.minute_short"),
    second_one: t("times.second_one"),
    second_other: t("times.second_other"),
    second_short: t("times.second_short"),
  };
  translations.centercontent = {
    coverextract_work: t("centercontent.coverextract_work"),
    coverextract_done: t("centercontent.coverextract_done"),
    libraryimport_work: t("centercontent.libraryimport_work"),
    libraryimport_done: t("centercontent.libraryimport_done"),
    fileimport_work: t("centercontent.fileimport_work"),
    fileimport_done: t("centercontent.fileimport_done"),
    delete_work: t("centercontent.delete_work"),
    delete_done: t("centercontent.delete_done"),
    updatetracks_work: t("centercontent.updatetracks_work"),
    updatetracks_done: t("centercontent.updatetracks_done"),
    updatealbum_work: t("centercontent.updatealbum_work"),
    updatealbum_done: t("centercontent.updatealbum_done"),
  };
  translations.deletemodal = {
    albumsandtracks: t("deletemodal.albumsandtracks"),
    artistandtracks: t("deletemodal.artistandtracks"),
    composerandtracks: t("deletemodal.composerandtracks"),
    genreandtracks: t("deletemodal.genreandtracks"),
    track: t("deletemodal.track"),
    playlist: t("deletemodal.playlist"),
    deleteincludingfiles: t("deletemodal.deleteincludingfiles"),
  };
  translations.notification = {
    libraryimport: {
      first: t("notification.libraryimport.first"),
      second: t("notification.libraryimport.second"),
      third: t("notification.libraryimport.third"),
    },
  };
  translations.emptyview = {
    nothinghere: t("emptyview.nothinghere"),
    importlibrary: {
      first: t("emptyview.importlibrary.first"),
      second: t("emptyview.importlibrary.second"),
      third: t("emptyview.importlibrary.third"),
    },
    importfiles: {
      first: t("emptyview.importfiles.first"),
      second: t("emptyview.importfiles.second"),
      third: t("emptyview.importfiles.third"),
    },
  };
  translations.playlist = {
    cannotmodify1024: t("playlist.cannotmodify1024"),
  };
  translations.llminfo = {
    long: t("llminfo.long"),
    short: t("llminfo.short"),
    notice: t("llminfo.notice"),
  };
}

export async function initI18N() {
  // TODO: move locales into static and use http backend for I18N
  let resources = {
    cs,
    da,
    de,
    el,
    en,
    es,
    fr,
    it,
    ja,
    ko,
    lb,
    nl,
    pl,
    pt,
    tr,
    zh,
  };

  // https://en.wikipedia.org/wiki/IETF_language_tag
  let oslocale = await locale();
  if (oslocale) {
    i18next.init(
      {
        lng: oslocale,
        fallbackLng: "en",
        resources,
      },
      fillTranslations
    );
  } else {
    i18next.use(LanguageDetector).init(
      {
        fallbackLng: "en",
        resources,
      },
      fillTranslations
    );
  }
}

export async function changeLanguage() {
  switch (config_state.language) {
    case Language.System:
      // TODO: correct way to fall back on LanguageDetector?
      let oslocale = await locale();
      if (oslocale) {
        i18next.changeLanguage(oslocale, fillTranslations);
      }
      break;
    case Language.Czech:
      i18next.changeLanguage("cs", fillTranslations);
      break;
    case Language.Danish:
      i18next.changeLanguage("da", fillTranslations);
      break;
    case Language.German:
      i18next.changeLanguage("de", fillTranslations);
      break;
    case Language.Greek:
      i18next.changeLanguage("el", fillTranslations);
      break;
    case Language.English:
      i18next.changeLanguage("en", fillTranslations);
      break;
    case Language.Spanish:
      i18next.changeLanguage("es", fillTranslations);
      break;
    case Language.French:
      i18next.changeLanguage("fr", fillTranslations);
      break;
    case Language.Italian:
      i18next.changeLanguage("it", fillTranslations);
      break;
    case Language.Japanese:
      i18next.changeLanguage("ja", fillTranslations);
      break;
    case Language.Korean:
      i18next.changeLanguage("ko", fillTranslations);
      break;
    case Language.Luxembourgish:
      i18next.changeLanguage("lb", fillTranslations);
      break;
    case Language.Dutch:
      i18next.changeLanguage("nl", fillTranslations);
      break;
    case Language.Polish:
      i18next.changeLanguage("pl", fillTranslations);
      break;
    case Language.Portuguese:
      i18next.changeLanguage("pt", fillTranslations);
      break;
    case Language.Turkish:
      i18next.changeLanguage("tr", fillTranslations);
      break;
    case Language.Chinese:
      i18next.changeLanguage("zh", fillTranslations);
      break;

    default:
      break;
  }
}
