mod metadata;

use crate::{
    config::{load_or_setup_config_path, set_config},
    defs::{
        Album, Artist, ArtistAlbums, ArtistTracks, AudioTrack, BackendMessage, Composer,
        ComposerTracks, ConfigState, Cover, DBData, DBPlaylist, DBRequest, DBState, DBTrack, Data,
        DataType, Genre, GenreTracks, Image, Notification, Order, Playlist, Progress, ProgressInfo,
        Search, SpaceTime, Track,
    },
};
use base64::{engine::general_purpose, Engine as _};
use log::{debug, error};
use metadata::extract_metadata;
use quick_xml::{escape::unescape, events::Event, reader::Reader};
use regex::Regex;
use sqlite::{self, Connection, State};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::BufReader,
    path::{Path, PathBuf},
    sync::mpsc::{Receiver, Sender},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Emitter, Manager};
use urlencoding;

static DB_MAJOR: i64 = 1;
static DB_MINOR: i64 = 0;
static DB_PATCH: i64 = 0;

fn get_db_state(conn: &Connection) -> DBState {
    let mut dbstate = DBState {
        tracks_max: None,
        albums_max: None,
        artists_max: None,
        genres_max: None,
        playlists_max: None,
    };

    // TODO: add option for media path for importing. If that path is empty, some safe default?

    let mut stmt = conn
        .prepare("SELECT COUNT(*) as num FROM Artists;")
        .unwrap();
    if let Ok(State::Row) = stmt.next() {
        dbstate.artists_max = Some(stmt.read::<i64, _>("num").unwrap_or_default());
    }

    let mut stmt = conn.prepare("SELECT COUNT(*) as num FROM Genres;").unwrap();
    if let Ok(State::Row) = stmt.next() {
        dbstate.genres_max = Some(stmt.read::<i64, _>("num").unwrap_or_default());
    }

    let mut stmt = conn.prepare("SELECT COUNT(*) as num FROM Albums;").unwrap();
    if let Ok(State::Row) = stmt.next() {
        dbstate.albums_max = Some(stmt.read::<i64, _>("num").unwrap_or_default());
    }

    let mut stmt = conn.prepare("SELECT COUNT(*) as num FROM Tracks;").unwrap();
    if let Ok(State::Row) = stmt.next() {
        dbstate.tracks_max = Some(stmt.read::<i64, _>("num").unwrap_or_default());
    }

    let mut stmt = conn
        .prepare("SELECT COUNT(*) as num FROM Playlists;")
        .unwrap();
    if let Ok(State::Row) = stmt.next() {
        dbstate.playlists_max = Some(stmt.read::<i64, _>("num").unwrap_or_default());
    }

    return dbstate;
}

fn get_or_create_artist_id(conn: &Connection, name: &str, sort_name: &str) -> i64 {
    let mut stmt = conn
        .prepare("SELECT artist_id FROM Artists WHERE name = ?1")
        .unwrap();
    stmt.bind((1, name)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        return stmt.read::<i64, _>("artist_id").unwrap_or_default();
    }

    let mut insert_stmt = conn
        .prepare("INSERT INTO Artists (name, sort_artist) VALUES (?1, ?2)")
        .unwrap();
    insert_stmt.bind((1, name)).unwrap();
    insert_stmt.bind((2, sort_name)).unwrap();
    insert_stmt.next().unwrap();

    let mut stmt = conn
        .prepare("SELECT artist_id FROM Artists WHERE name = ?1")
        .unwrap();
    stmt.bind((1, name)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        return stmt.read::<i64, _>("artist_id").unwrap_or_default();
    }

    return 0;
}

fn get_or_create_composer_id(conn: &Connection, name: &str) -> i64 {
    let mut stmt = conn
        .prepare("SELECT composer_id FROM Composers WHERE name = ?1")
        .unwrap();
    stmt.bind((1, name)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        return stmt.read::<i64, _>("composer_id").unwrap_or_default();
    }

    let mut insert_stmt = conn
        .prepare("INSERT INTO Composers (name) VALUES (?1)")
        .unwrap();
    insert_stmt.bind((1, name)).unwrap();
    insert_stmt.next().unwrap();

    let mut stmt = conn
        .prepare("SELECT composer_id FROM Composers WHERE name = ?1")
        .unwrap();
    stmt.bind((1, name)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        return stmt.read::<i64, _>("composer_id").unwrap_or_default();
    }

    return 0;
}

fn get_or_create_genre_id(conn: &Connection, name: &str) -> i64 {
    let mut stmt = conn
        .prepare("SELECT genre_id FROM Genres WHERE name = ?1")
        .unwrap();
    stmt.bind((1, name)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        return stmt.read::<i64, _>("genre_id").unwrap_or_default();
    }

    let mut insert_stmt = conn
        .prepare("INSERT INTO Genres (name) VALUES (?1)")
        .unwrap();
    insert_stmt.bind((1, name)).unwrap();
    insert_stmt.next().unwrap();

    let mut stmt = conn
        .prepare("SELECT genre_id FROM Genres WHERE name = ?1")
        .unwrap();
    stmt.bind((1, name)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        return stmt.read::<i64, _>("genre_id").unwrap_or_default();
    }

    return 0;
}

fn get_or_create_album_id(
    conn: &Connection,
    name: &str,
    sort_name: &str,
    artist_id: i64,
    genre_id: i64,
    year: i64,
    release_date: &str,
    import_dates: Option<(&str, &str)>,
) -> i64 {
    let mut stmt = conn
        .prepare("SELECT album_id FROM Albums WHERE name = ?1 AND artist_id = ?2")
        .unwrap();
    stmt.bind((1, name)).unwrap();
    stmt.bind((2, artist_id)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        return stmt.read::<i64, _>("album_id").unwrap_or_default();
    }

    if let Some((date_modified, date_added)) = import_dates {
        let mut insert_stmt = conn
        .prepare(
            "INSERT INTO Albums (artist_id, name, sort_album, genre_id, year, release_date, date_modified, date_added) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .unwrap();
        insert_stmt.bind((1, artist_id)).unwrap();
        insert_stmt.bind((2, name)).unwrap();
        insert_stmt.bind((3, sort_name)).unwrap();
        insert_stmt.bind((4, genre_id)).unwrap();
        insert_stmt.bind((5, year)).unwrap();
        insert_stmt.bind((6, release_date)).unwrap();
        insert_stmt.bind((7, date_modified)).unwrap();
        insert_stmt.bind((8, date_added)).unwrap();
        insert_stmt.next().unwrap();
    } else {
        let mut insert_stmt = conn
        .prepare(
            "INSERT INTO Albums (artist_id, name, sort_album, genre_id, year, release_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )
        .unwrap();
        insert_stmt.bind((1, artist_id)).unwrap();
        insert_stmt.bind((2, name)).unwrap();
        insert_stmt.bind((3, sort_name)).unwrap();
        insert_stmt.bind((4, genre_id)).unwrap();
        insert_stmt.bind((5, year)).unwrap();
        insert_stmt.bind((6, release_date)).unwrap();
        insert_stmt.next().unwrap();
    }

    let mut stmt = conn
        .prepare("SELECT album_id FROM Albums WHERE name = ?1 AND artist_id = ?2")
        .unwrap();
    stmt.bind((1, name)).unwrap();
    stmt.bind((2, artist_id)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        return stmt.read::<i64, _>("album_id").unwrap_or_default();
    }

    return 0;
}

fn get_or_create_cover_id(conn: &Connection, album_id: i64, cover: &Image) -> i64 {
    let mut stmt = conn
        .prepare("SELECT cover_id FROM Albums WHERE album_id = ?1")
        .unwrap();
    stmt.bind((1, album_id)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        let cover_id = stmt.read::<i64, _>("cover_id").unwrap_or_default();
        if cover_id != 0 {
            return cover_id;
        } else {
            let b64_string = cover_as_base64(cover);

            let mut insert_stmt = conn
                .prepare("INSERT INTO Covers (album_id, base64) VALUES (?1, ?2)")
                .unwrap();
            insert_stmt.bind((1, album_id)).unwrap();
            insert_stmt.bind((2, b64_string.as_str())).unwrap();
            insert_stmt.next().unwrap();

            let mut stmt = conn
                .prepare("SELECT cover_id FROM Covers WHERE album_id = ?1")
                .unwrap();
            stmt.bind((1, album_id)).unwrap();

            if let Ok(State::Row) = stmt.next() {
                let cover_id = stmt.read::<i64, _>("cover_id").unwrap_or_default();
                let mut insert_stmt = conn
                    .prepare("UPDATE Albums SET cover_id = ?1 WHERE album_id = ?2")
                    .unwrap();
                insert_stmt.bind((1, cover_id)).unwrap();
                insert_stmt.bind((2, album_id)).unwrap();
                insert_stmt.next().unwrap();

                return cover_id;
            }
        }
    }

    return 0;
}

fn fix_umlauts(text: String) -> String {
    // TODO: differences between host systems? Below is working and required for NixOS with BTRFS.
    // Replace umlaut escape sequence with the single byte umlaut, where possible.
    let mut result = "".to_string();
    let mut previous = text.chars().next().unwrap();

    for character in text.chars() {
        match character {
            '\u{308}' => match previous {
                'a' => {
                    result.pop();
                    result.push('ä');
                }
                'A' => {
                    result.pop();
                    result.push('Ä');
                }
                'o' => {
                    result.pop();
                    result.push('ö');
                }
                'O' => {
                    result.pop();
                    result.push('Ö');
                }
                'u' => {
                    result.pop();
                    result.push('ü');
                }
                'U' => {
                    result.pop();
                    result.push('Ü');
                }
                _ => {
                    result.push(character);
                }
            },
            _ => {
                result.push(character);
            }
        }
        previous = character;
    }

    return result;
}

fn extract_tracks_and_playlists(xml_path_string: String, conn: &Connection) {
    // TODO: sanity checks to ensure this is a library.xml file
    // TODO: differences in versions we need to cover?
    let mut reader = Reader::from_file(xml_path_string).unwrap();
    // Do not trim, otherwise spaces between text and special chars are removed...
    //reader.config_mut().trim_text(true);

    let mut depth = 0;
    let mut buf = Vec::new();
    let mut is_tracks_or_playlist_depth = false;
    let mut found_tracks = false;
    let mut found_playlists = false;
    let mut extract = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                if extract {
                    extract = false;

                    if str::from_utf8(e.name().as_ref()).unwrap() == "dict" && found_tracks {
                        extract_and_insert_tracks(&mut reader, &mut depth, conn);
                        found_tracks = false;
                    }

                    if str::from_utf8(e.name().as_ref()).unwrap() == "array" && found_playlists {
                        extract_and_insert_playlists(&mut reader, &mut depth, conn);
                        found_playlists = false;
                    }
                }

                if depth == 2 && str::from_utf8(e.name().as_ref()).unwrap() == "key" {
                    is_tracks_or_playlist_depth = true;
                } else if depth != 2 {
                    is_tracks_or_playlist_depth = false;
                }
                depth += 1;
            }
            Ok(Event::Text(e)) => {
                if is_tracks_or_playlist_depth {
                    if e.decode().unwrap().into_owned() == "Tracks" {
                        println!("Found Tracks at depth: {}", depth);
                        found_tracks = true;
                    }
                    if e.decode().unwrap().into_owned() == "Playlists" {
                        println!("Found Playlists at depth: {}", depth);
                        found_playlists = true;
                    }
                }
            }
            Ok(Event::End(e)) => {
                depth -= 1;
                if is_tracks_or_playlist_depth
                    && str::from_utf8(e.name().as_ref()).unwrap() == "key"
                    && (found_tracks || found_playlists)
                {
                    extract = true;
                    println!("extract true at depth: {}", depth);
                }
            }
            _ => (),
        }
        // clear the buffer to keep memory usage low
        buf.clear();
    }
}

fn extract_and_insert_tracks(
    reader: &mut Reader<BufReader<fs::File>>,
    depth: &mut usize,
    conn: &Connection,
) {
    let initial_depth = depth.clone();
    println!(
        "extract_and_create_tracks called at depth: {}",
        initial_depth
    );

    let mut current_track_id = "".to_string();
    let mut next_is_id = false;
    let mut next_dict_is_track = false;

    let mut buf = Vec::new();
    let mut tracks: Vec<DBTrack> = Vec::new();
    loop {
        // SQLite 3.32.0 can handle up to 32766 variables
        // Imported track has 31, thus max 1056 tracks can be batch imported
        // Leaving some headroom
        if tracks.len() > 1000 {
            insert_tracks_batch(conn, tracks.clone(), true);
            tracks.clear();
        }

        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                *depth += 1;
                if str::from_utf8(e.name().as_ref()).unwrap() == "key" {
                    next_is_id = true;
                }
                if str::from_utf8(e.name().as_ref()).unwrap() == "dict" && next_dict_is_track {
                    next_dict_is_track = false;

                    if let Some(track) = extract_track(reader, depth, &current_track_id, conn) {
                        tracks.push(track);
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if next_is_id {
                    next_is_id = false;
                    current_track_id = e.decode().unwrap().into_owned();
                    /*println!("track id: {}", current_track_id);*/
                    next_dict_is_track = true;
                }
            }
            Ok(Event::End(_)) => {
                *depth -= 1;
                if *depth < initial_depth {
                    /*println!("lower depth than initial -> break");*/
                    break;
                }
            }
            _ => (),
        }
        // clear the buffer to keep memory usage low
        buf.clear();
    }

    // insert remaining tracks
    insert_tracks_batch(conn, tracks, true);
}

fn extract_track(
    reader: &mut Reader<BufReader<fs::File>>,
    depth: &mut usize,
    track_id: &String,
    conn: &Connection,
) -> Option<DBTrack> {
    let initial_depth = depth.clone();

    let mut track = DBTrack {
        orig_track_id: 0,
        name: "".to_string(),
        artist_id: get_or_create_artist_id(conn, "", ""),
        album_artist_id: get_or_create_artist_id(conn, "", ""),
        composer_id: get_or_create_composer_id(conn, ""),
        album_id: 0, // Only create empty album if necessary
        genre_id: get_or_create_genre_id(conn, ""),
        kind: "".to_string(),
        size: 0,
        total_time: 0,
        disc_number: 0,
        disc_count: 0,
        track_number: 0,
        track_count: 0,
        year: 0,
        date_modified: "".to_string(),
        date_added: "".to_string(),
        bit_rate: 0,
        sample_rate: 0,
        release_date: "".to_string(),
        normalization: 0,
        artwork_count: 0,
        sort_name: "".to_string(),
        persistent_id: "".to_string(),
        track_type: "".to_string(),
        purchased: 0,
        has_video: 0,
        hd: 0,
        video_width: 0,
        video_height: 0,
        music_video: 0,
        location: "".to_string(),
        file_folder_count: 0,
        library_folder_count: 0,
    };

    // Create placeholders
    let mut album = "".to_string();
    let mut sort_album = "".to_string();
    let mut artist = "".to_string();
    let mut sort_artist = "".to_string();
    let mut album_artist = "".to_string();

    let mut current_key = "".to_string();
    let mut current_text = "".to_string();
    let mut next_is_key = false;
    let mut next_is_value = false;
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                *depth += 1;
                if str::from_utf8(e.name().as_ref()).unwrap() == "key" {
                    next_is_key = true;
                    next_is_value = false;
                }
                if next_is_value {
                    current_text = "".to_string();
                }
            }
            Ok(Event::Empty(e)) => {
                if next_is_value {
                    match current_key.as_str() {
                        "Purchased" => match str::from_utf8(e.name().as_ref()).unwrap() {
                            "true" => {
                                track.purchased = 1;
                            }
                            "false" => {
                                track.purchased = 0;
                            }
                            _ => {}
                        },
                        "Has Video" => match str::from_utf8(e.name().as_ref()).unwrap() {
                            "true" => {
                                track.has_video = 1;
                            }
                            "false" => {
                                track.has_video = 0;
                            }
                            _ => {}
                        },
                        "HD" => match str::from_utf8(e.name().as_ref()).unwrap() {
                            "true" => {
                                track.hd = 1;
                            }
                            "false" => {
                                track.hd = 0;
                            }
                            _ => {}
                        },
                        "Music Video" => match str::from_utf8(e.name().as_ref()).unwrap() {
                            "true" => {
                                track.music_video = 1;
                            }
                            "false" => {
                                track.music_video = 0;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if next_is_value {
                    /*
                    Do not set next_is_value to false to allow multiple text entries!
                    quick_xml splits Text into multiple lines when it contains special chars.
                    If something comes before or after the special char it is normal text.
                    The special char is stripped of the "&" and ";" and can only be decoded with resolve_char_ref().
                     */

                    let text = &e.decode().unwrap().into_owned();

                    if let Ok(escaped) = unescape(&text) {
                        current_text.push_str(&escaped);
                    } else {
                        let extended = format!("{};", text);
                        if let Ok(escaped) = unescape(&extended) {
                            current_text.push_str(&escaped);
                        }
                    }
                }
                if next_is_key {
                    current_key = e.decode().unwrap().into_owned().to_string();
                }
            }
            Ok(Event::GeneralRef(e)) => {
                if next_is_value {
                    /*
                    Do not set next_is_value to false to allow multiple text entries!
                    quick_xml splits Text into multiple lines when it contains special chars.
                    If something comes before or after the special char it is normal text.
                    The special char is stripped of the "&" and ";" and can only be decoded with resolve_char_ref().
                     */

                    if let Some(text) = e.resolve_char_ref().unwrap() {
                        current_text.push(text);
                    } else if let Ok(escaped) = unescape(&e.decode().unwrap().into_owned()) {
                        current_text.push_str(&escaped);
                    } else {
                        let extended = format!("&{};", &e.decode().unwrap().into_owned());
                        if let Ok(escaped) = unescape(&extended) {
                            current_text.push_str(&escaped);
                        }
                    }
                }
            }
            Ok(Event::End(_)) => {
                if next_is_value {
                    next_is_value = false;

                    match current_key.as_str() {
                        "Track ID" => {
                            track.orig_track_id = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Name" => {
                            track.name = current_text;
                        }
                        "Artist" => {
                            artist = current_text;
                        }
                        "Album Artist" => {
                            album_artist = current_text;
                        }
                        "Composer" => {
                            track.composer_id = get_or_create_composer_id(conn, &current_text);
                        }
                        "Album" => {
                            album = current_text;
                        }
                        "Genre" => {
                            track.genre_id = get_or_create_genre_id(conn, &current_text);
                        }
                        "Kind" => {
                            track.kind = current_text;
                        }
                        "Size" => {
                            track.size = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Total Time" => {
                            track.total_time = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Disc Number" => {
                            track.disc_number = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Disc Count" => {
                            track.disc_count = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Track Number" => {
                            track.track_number = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Track Count" => {
                            track.track_count = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Year" => {
                            track.year = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Date Modified" => {
                            track.date_modified = current_text;
                        }
                        "Date Added" => {
                            track.date_added = current_text;
                        }
                        "Bit Rate" => {
                            track.bit_rate = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Sample Rate" => {
                            track.sample_rate = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Release Date" => {
                            track.release_date = current_text;
                        }
                        "Normalization" => {
                            track.normalization = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Sort Album" => {
                            sort_album = current_text;
                        }
                        "Sort Artist" => {
                            sort_artist = current_text;
                        }
                        "Sort Name" => {
                            track.sort_name = current_text;
                        }
                        "Artwork Count" => {
                            track.artwork_count = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Persistent ID" => {
                            track.persistent_id = current_text;
                        }
                        "Track Type" => {
                            track.track_type = current_text;
                        }
                        "Video Width" => {
                            track.video_width = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Video Height" => {
                            track.video_height = i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Location" => {
                            track.location = fix_umlauts(
                                urlencoding::decode(&current_text).unwrap().to_string(),
                            )
                            .replace("file://", "");
                        }
                        "File Folder Count" => {
                            track.file_folder_count =
                                i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        "Library Folder Count" => {
                            track.library_folder_count =
                                i64::from_str_radix(&current_text, 10).unwrap();
                        }
                        _ => {}
                    }
                    current_text = "".to_string();
                }

                if next_is_key {
                    next_is_key = false;
                    next_is_value = true;
                }

                *depth -= 1;
                if *depth < initial_depth {
                    /*println!("lower depth than initial -> break");*/
                    break;
                }
            }
            _ => {}
        }
        // clear the buffer to keep memory usage low
        buf.clear();
    }

    // Retrieving artist_id requires sort_artist, if present
    track.artist_id = get_or_create_artist_id(conn, artist.as_str(), sort_artist.as_str());

    // Retrieving artist_id requires sort_artist, if present
    //track.artist_id = get_or_create_artist_id(conn, artist.as_str(), sort_artist.as_str());

    // If not album_artist is given, use the artist as standin, even if this might be wrong
    if &album_artist != "" {
        track.album_artist_id = get_or_create_artist_id(conn, album_artist.as_str(), "");
        if &artist == "" {
            track.artist_id = track.album_artist_id;
        }
    } else {
        if &artist == "" {
            track.artist_id = get_or_create_artist_id(conn, "", "");
        }
        track.album_artist_id = track.artist_id;
    }

    track.album_id = get_or_create_album_id(
        conn,
        &album,
        &sort_album,
        track.album_artist_id,
        track.genre_id,
        track.year,
        &track.release_date,
        Some((&track.date_modified, &track.date_added)),
    );

    if track.orig_track_id == i64::from_str_radix(&track_id, 10).unwrap() {
        return Some(track);
    }

    return None;
}

fn insert_tracks_batch(conn: &Connection, tracks: Vec<DBTrack>, import: bool) {
    println!("insert tracks batch");
    if import {
        let mut query = "INSERT INTO Tracks (
                orig_track_id,
                name,
                artist_id,
                album_artist_id,
                composer_id,
                album_id,
                genre_id,
                kind,
                size,
                total_time,
                disc_number,
                disc_count,
                track_number,
                track_count,
                year,
                date_modified,
                date_added,
                bit_rate,
                sample_rate,
                release_date,
                normalization,
                artwork_count,
                sort_name,
                persistent_id,
                track_type,
                purchased,
                has_video,
                music_video,
                location,
                file_folder_count,
                library_folder_count
            ) VALUES"
            .to_string();
        let mut values = " (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            ),".repeat(tracks.len());
        values.pop();
        query.push_str(values.as_str());

        let mut stmt = conn.prepare(query).unwrap();

        for (index, track) in tracks.iter().enumerate() {
            let offset = index * 31;
            stmt.bind((offset + 1, track.orig_track_id)).unwrap();
            stmt.bind((offset + 2, track.name.as_str())).unwrap();
            stmt.bind((offset + 3, track.artist_id)).unwrap();
            stmt.bind((offset + 4, track.album_artist_id)).unwrap();
            stmt.bind((offset + 5, track.composer_id)).unwrap();
            stmt.bind((offset + 6, track.album_id)).unwrap();
            stmt.bind((offset + 7, track.genre_id)).unwrap();
            stmt.bind((offset + 8, track.kind.as_str())).unwrap();
            stmt.bind((offset + 9, track.size)).unwrap();
            stmt.bind((offset + 10, track.total_time)).unwrap();
            stmt.bind((offset + 11, track.disc_number)).unwrap();
            stmt.bind((offset + 12, track.disc_count)).unwrap();
            stmt.bind((offset + 13, track.track_number)).unwrap();
            stmt.bind((offset + 14, track.track_count)).unwrap();
            stmt.bind((offset + 15, track.year)).unwrap();
            stmt.bind((offset + 16, track.date_modified.as_str()))
                .unwrap();
            stmt.bind((offset + 17, track.date_added.as_str())).unwrap();
            stmt.bind((offset + 18, track.bit_rate)).unwrap();
            stmt.bind((offset + 19, track.sample_rate)).unwrap();
            stmt.bind((offset + 20, track.release_date.as_str()))
                .unwrap();
            stmt.bind((offset + 21, track.normalization)).unwrap();
            stmt.bind((offset + 22, track.artwork_count)).unwrap();
            stmt.bind((offset + 23, track.sort_name.as_str())).unwrap();
            stmt.bind((offset + 24, track.persistent_id.as_str()))
                .unwrap();
            stmt.bind((offset + 25, track.track_type.as_str())).unwrap();
            stmt.bind((offset + 26, track.purchased)).unwrap();
            stmt.bind((offset + 27, track.has_video)).unwrap();
            stmt.bind((offset + 28, track.music_video)).unwrap();
            stmt.bind((offset + 29, track.location.as_str())).unwrap();
            stmt.bind((offset + 30, track.file_folder_count)).unwrap();
            stmt.bind((offset + 31, track.library_folder_count))
                .unwrap();
        }

        let res = stmt.next();
        println!("{:?}", res);
        // TODO: implement logging
    } else {
        let mut query = "INSERT INTO Tracks (
                orig_track_id,
                name,
                artist_id,
                album_artist_id,
                composer_id,
                album_id,
                genre_id,
                kind,
                size,
                total_time,
                disc_number,
                disc_count,
                track_number,
                track_count,
                year,
                bit_rate,
                sample_rate,
                release_date,
                has_video,
                music_video,
                location
            ) VALUES"
            .to_string();

        let mut values = " (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            ),"
        .repeat(tracks.len());
        values.pop();
        query.push_str(values.as_str());

        let mut stmt = conn.prepare(query).unwrap();

        for (index, track) in tracks.iter().enumerate() {
            let offset = index * 21;
            stmt.bind((offset + 1, track.orig_track_id)).unwrap();
            stmt.bind((offset + 2, track.name.as_str())).unwrap();
            stmt.bind((offset + 3, track.artist_id)).unwrap();
            stmt.bind((offset + 4, track.album_artist_id)).unwrap();
            stmt.bind((offset + 5, track.composer_id)).unwrap();
            stmt.bind((offset + 6, track.album_id)).unwrap();
            stmt.bind((offset + 7, track.genre_id)).unwrap();
            stmt.bind((offset + 8, track.kind.as_str())).unwrap();
            stmt.bind((offset + 9, track.size)).unwrap();
            stmt.bind((offset + 10, track.total_time)).unwrap();
            stmt.bind((offset + 11, track.disc_number)).unwrap();
            stmt.bind((offset + 12, track.disc_count)).unwrap();
            stmt.bind((offset + 13, track.track_number)).unwrap();
            stmt.bind((offset + 14, track.track_count)).unwrap();
            stmt.bind((offset + 15, track.year)).unwrap();
            stmt.bind((offset + 16, track.bit_rate)).unwrap();
            stmt.bind((offset + 17, track.sample_rate)).unwrap();
            stmt.bind((offset + 18, track.release_date.as_str()))
                .unwrap();
            stmt.bind((offset + 19, track.has_video)).unwrap();
            stmt.bind((offset + 20, track.music_video)).unwrap();
            stmt.bind((offset + 21, track.location.as_str())).unwrap();
        }
        let _ = stmt.next();
    }
}

fn insert_track(conn: &Connection, track: DBTrack, import: bool) {
    if import {
        let mut stmt = conn
            .prepare(
                "INSERT INTO Tracks (
                orig_track_id,
                name,
                artist_id,
                album_artist_id,
                composer_id,
                album_id,
                genre_id,
                kind,
                size,
                total_time,
                disc_number,
                disc_count,
                track_number,
                track_count,
                year,
                date_modified,
                date_added,
                bit_rate,
                sample_rate,
                release_date,
                normalization,
                artwork_count,
                sort_name,
                persistent_id,
                track_type,
                purchased,
                has_video,
                music_video,
                location,
                file_folder_count,
                library_folder_count
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31
            )",
            )
            .unwrap();

        stmt.bind((1, track.orig_track_id)).unwrap();
        stmt.bind((2, track.name.as_str())).unwrap();
        stmt.bind((3, track.artist_id)).unwrap();
        stmt.bind((4, track.album_artist_id)).unwrap();
        stmt.bind((5, track.composer_id)).unwrap();
        stmt.bind((6, track.album_id)).unwrap();
        stmt.bind((7, track.genre_id)).unwrap();
        stmt.bind((8, track.kind.as_str())).unwrap();
        stmt.bind((9, track.size)).unwrap();
        stmt.bind((10, track.total_time)).unwrap();
        stmt.bind((11, track.disc_number)).unwrap();
        stmt.bind((12, track.disc_count)).unwrap();
        stmt.bind((13, track.track_number)).unwrap();
        stmt.bind((14, track.track_count)).unwrap();
        stmt.bind((15, track.year)).unwrap();
        stmt.bind((16, track.date_modified.as_str())).unwrap();
        stmt.bind((17, track.date_added.as_str())).unwrap();
        stmt.bind((18, track.bit_rate)).unwrap();
        stmt.bind((19, track.sample_rate)).unwrap();
        stmt.bind((20, track.release_date.as_str())).unwrap();
        stmt.bind((21, track.normalization)).unwrap();
        stmt.bind((22, track.artwork_count)).unwrap();
        stmt.bind((23, track.sort_name.as_str())).unwrap();
        stmt.bind((24, track.persistent_id.as_str())).unwrap();
        stmt.bind((25, track.track_type.as_str())).unwrap();
        stmt.bind((26, track.purchased)).unwrap();
        stmt.bind((27, track.has_video)).unwrap();
        stmt.bind((28, track.music_video)).unwrap();
        stmt.bind((29, track.location.as_str())).unwrap();
        stmt.bind((30, track.file_folder_count)).unwrap();
        stmt.bind((31, track.library_folder_count)).unwrap();

        let _ = stmt.next();
    } else {
        let mut stmt = conn
            .prepare(
                "INSERT INTO Tracks (
                orig_track_id,
                name,
                artist_id,
                album_artist_id,
                composer_id,
                album_id,
                genre_id,
                kind,
                size,
                total_time,
                disc_number,
                disc_count,
                track_number,
                track_count,
                year,
                bit_rate,
                sample_rate,
                release_date,
                has_video,
                music_video,
                location
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21
            )",
            )
            .unwrap();

        stmt.bind((1, track.orig_track_id)).unwrap();
        stmt.bind((2, track.name.as_str())).unwrap();
        stmt.bind((3, track.artist_id)).unwrap();
        stmt.bind((4, track.album_artist_id)).unwrap();
        stmt.bind((5, track.composer_id)).unwrap();
        stmt.bind((6, track.album_id)).unwrap();
        stmt.bind((7, track.genre_id)).unwrap();
        stmt.bind((8, track.kind.as_str())).unwrap();
        stmt.bind((9, track.size)).unwrap();
        stmt.bind((10, track.total_time)).unwrap();
        stmt.bind((11, track.disc_number)).unwrap();
        stmt.bind((12, track.disc_count)).unwrap();
        stmt.bind((13, track.track_number)).unwrap();
        stmt.bind((14, track.track_count)).unwrap();
        stmt.bind((15, track.year)).unwrap();
        stmt.bind((16, track.bit_rate)).unwrap();
        stmt.bind((17, track.sample_rate)).unwrap();
        stmt.bind((18, track.release_date.as_str())).unwrap();
        stmt.bind((19, track.has_video)).unwrap();
        stmt.bind((20, track.music_video)).unwrap();
        stmt.bind((21, track.location.as_str())).unwrap();

        let _ = stmt.next();
    }
}

fn extract_and_insert_playlists(
    reader: &mut Reader<BufReader<fs::File>>,
    depth: &mut usize,
    conn: &Connection,
) {
    let initial_depth = depth.clone();
    println!(
        "extract_and_insert_playlists called at depth: {}",
        initial_depth
    );

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                *depth += 1;
                if str::from_utf8(e.name().as_ref()).unwrap() == "dict" {
                    if let Some(playlist) = extract_playlist(reader, depth, conn) {
                        insert_playlist(conn, playlist, true);
                    }
                }
            }
            Ok(Event::End(_)) => {
                *depth -= 1;
                if *depth < initial_depth {
                    println!("lower depth than initial -> break");
                    break;
                }
            }
            _ => (),
        }
        // clear the buffer to keep memory usage low
        buf.clear();
    }
}

fn extract_playlist(
    reader: &mut Reader<BufReader<fs::File>>,
    depth: &mut usize,
    conn: &Connection,
) -> Option<DBPlaylist> {
    let initial_depth = depth.clone();

    let mut playlist = DBPlaylist {
        ..Default::default()
    };

    let mut current_key = "".to_string();
    let mut next_is_key = false;
    let mut next_start_is_value = false;
    let mut next_is_value = false;
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                *depth += 1;
                if next_start_is_value {
                    next_start_is_value = true;
                    next_is_value = true;
                }
                if str::from_utf8(e.name().as_ref()).unwrap() == "key" {
                    next_is_key = true;
                    next_is_value = false;
                }
                if next_is_value
                    && current_key.as_str() == "Playlist Items"
                    && str::from_utf8(e.name().as_ref()).unwrap() == "array"
                {
                    next_is_value = false;
                    playlist.tracks = extract_playlist_tracks(reader, depth, conn);
                }
            }
            Ok(Event::Empty(e)) => {
                if next_is_value {
                    next_is_value = false;

                    match current_key.as_str() {
                        "Master" => match str::from_utf8(e.name().as_ref()).unwrap() {
                            "true" => {
                                playlist.master = 1;
                            }
                            "false" => {
                                playlist.master = 0;
                            }
                            _ => {}
                        },
                        "Visible" => match str::from_utf8(e.name().as_ref()).unwrap() {
                            "true" => {
                                playlist.visible = 1;
                            }
                            "false" => {
                                playlist.visible = 0;
                            }
                            _ => {}
                        },
                        "All Items" => match str::from_utf8(e.name().as_ref()).unwrap() {
                            "true" => {
                                playlist.all_items = 1;
                            }
                            "false" => {
                                playlist.all_items = 0;
                            }
                            _ => {}
                        },
                        "Folder" => match str::from_utf8(e.name().as_ref()).unwrap() {
                            "true" => {
                                playlist.folder = 1;
                            }
                            "false" => {
                                playlist.folder = 0;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if next_is_value {
                    next_is_value = false;

                    match current_key.as_str() {
                        "Playlist ID" => {
                            playlist.orig_playlist_id = i64::from_str_radix(
                                &e.decode().unwrap().into_owned().to_string(),
                                10,
                            )
                            .unwrap();
                        }
                        "Name" => {
                            playlist.name = e.decode().unwrap().into_owned().to_string();
                        }
                        "Description" => {
                            playlist.description = e.decode().unwrap().into_owned().to_string();
                        }
                        "Playlist Persistent ID" => {
                            playlist.persistent_id = e.decode().unwrap().into_owned().to_string();
                        }
                        "Parent Persistent ID" => {
                            playlist.parent_persistent_id =
                                e.decode().unwrap().into_owned().to_string();
                        }
                        "Distinguished Kind" => {
                            playlist.distinguished_kind = i64::from_str_radix(
                                &e.decode().unwrap().into_owned().to_string(),
                                10,
                            )
                            .unwrap();
                        }
                        "Smart Info" => {
                            // TODO: correct? field is data, not string as for others
                            playlist.smart_info = e.decode().unwrap().into_owned().to_string();
                        }
                        "Smart Criteria" => {
                            // TODO: correct? field is data, not string as for others
                            playlist.smart_criteria = e.decode().unwrap().into_owned().to_string();
                        }
                        _ => {}
                    }
                }
                if next_is_key {
                    current_key = e.decode().unwrap().into_owned().to_string();
                }
            }
            Ok(Event::End(_)) => {
                next_is_value = false;
                if next_is_key {
                    next_is_key = false;
                    next_start_is_value = true;
                }
                *depth -= 1;
                if *depth < initial_depth {
                    /*println!("lower depth than initial -> break");*/
                    break;
                }
            }
            _ => (),
        }
        // clear the buffer to keep memory usage low
        buf.clear();
    }

    return Some(playlist);
}

fn extract_playlist_tracks(
    reader: &mut Reader<BufReader<fs::File>>,
    depth: &mut usize,
    conn: &Connection,
) -> Vec<i64> {
    println!("extract_playlist_tracks");
    let initial_depth = depth.clone();

    let mut orig_track_ids: Vec<i64> = Vec::new();
    let mut track_ids: Vec<i64> = Vec::new();

    let mut next_is_key = false;
    let mut next_is_value = false;
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                *depth += 1;
                if str::from_utf8(e.name().as_ref()).unwrap() == "key" {
                    next_is_key = true;
                }
            }
            Ok(Event::Text(e)) => {
                if next_is_value {
                    next_is_value = false;

                    orig_track_ids.push(
                        i64::from_str_radix(&e.decode().unwrap().into_owned().to_string(), 10)
                            .unwrap(),
                    );
                }
                if next_is_key {
                    next_is_key = false;
                    next_is_value = true;
                }
            }
            Ok(Event::End(_)) => {
                *depth -= 1;
                if *depth < initial_depth {
                    /*println!("lower depth than initial -> break");*/
                    break;
                }
            }
            _ => (),
        }
        // clear the buffer to keep memory usage low
        buf.clear();
    }

    let mut stmt = conn
        .prepare("SELECT track_id, orig_track_id FROM Tracks")
        .unwrap();

    let mut orig_track_id_map: HashMap<i64, i64> = HashMap::new();
    while let Ok(State::Row) = stmt.next() {
        orig_track_id_map.insert(
            stmt.read::<i64, _>("orig_track_id").unwrap_or_default(),
            stmt.read::<i64, _>("track_id").unwrap_or_default(),
        );
    }

    for orig_track_id in orig_track_ids {
        if let Some(track_id) = orig_track_id_map.get(&orig_track_id) {
            track_ids.push(*track_id);
        } else {
            track_ids.push(-orig_track_id);
        }
    }

    return track_ids;
}

fn insert_playlist(conn: &Connection, playlist: DBPlaylist, import: bool) {
    if import {
        let mut stmt = conn
            .prepare(
                "INSERT INTO Playlists (
                orig_playlist_id,
                name,
                description,
                master,
                playlist_persistent_id,
                parent_persistent_id,
                distinguished_kind,
                visible,
                all_items,
                folder,
                smart_info,
                smart_criteria,
                tracks
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13
            )",
            )
            .unwrap();

        stmt.bind((1, playlist.orig_playlist_id)).unwrap();
        stmt.bind((2, playlist.name.as_str())).unwrap();
        stmt.bind((3, playlist.description.as_str())).unwrap();
        stmt.bind((4, playlist.master)).unwrap();
        stmt.bind((5, playlist.persistent_id.as_str())).unwrap();
        stmt.bind((6, playlist.parent_persistent_id.as_str()))
            .unwrap();
        stmt.bind((7, playlist.distinguished_kind)).unwrap();
        stmt.bind((8, playlist.visible)).unwrap();
        stmt.bind((9, playlist.all_items)).unwrap();
        stmt.bind((10, playlist.folder)).unwrap();
        stmt.bind((11, playlist.smart_info.as_str())).unwrap();
        stmt.bind((12, playlist.smart_criteria.as_str())).unwrap();
        stmt.bind((
            13,
            serde_json::to_string(&playlist.tracks).unwrap().as_str(),
        ))
        .unwrap();

        let _ = stmt.next();
    } else {
        let mut stmt = conn
            .prepare(
                "INSERT INTO Playlists (
                name,
                description,
                tracks
            ) VALUES (
                ?1, ?2, ?3
            )",
            )
            .unwrap();

        stmt.bind((1, playlist.name.as_str())).unwrap();
        stmt.bind((2, playlist.description.as_str())).unwrap();
        stmt.bind((3, serde_json::to_string(&playlist.tracks).unwrap().as_str()))
            .unwrap();

        let _ = stmt.next();
    }
}

fn sanitize_names(name: &str) -> String {
    // TODO: any other characters for macOS or Windows?
    // Define a set of unsafe characters for filenames (adjust based on your OS requirements)
    let unsafe_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

    // Replace each unsafe character with an underscore or another safe alternative
    name.chars()
        .map(|c| if unsafe_chars.contains(&c) { '_' } else { c })
        .collect::<String>()
}

fn _contains_cover_file(directory: &Path) -> bool {
    // Read all entries in the directory
    let mut contains_cover = false;
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.path().file_name() {
                // Check against both patterns
                if file_name
                    .to_string_lossy()
                    .to_lowercase()
                    .starts_with("cover.")
                {
                    contains_cover = true;
                    break;
                }
            }
        }
    }

    return contains_cover;
}

fn _get_timestamp() -> String {
    let start = SystemTime::now();
    // Convert to duration since Unix epoch
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX EPOCH!");

    // Format as a string
    return format!("{}", since_the_epoch.as_secs());
}

fn _ensure_and_copy_cover(dir: &str, artist: &str, album: &str, cover: &Image) -> String {
    /*
    TODO:
    Creating does involve storing the image in the album path
    as Cover.jpg or Cover.png depending on media_type.
    We do not want to overwrite any other picture that might be there,
    therefore we should test if a file Cover.* exists, if so, add a timestamp to the name.
     */

    println!("ensure_and_copy_cover image type: {}", cover.media_type);
    let suffix = if cover.media_type.to_lowercase().contains("jpeg") {
        "jpg".to_string()
    } else if cover.media_type.to_lowercase().contains("jpg") {
        "jpg".to_string()
    } else if cover.media_type.to_lowercase().contains("png") {
        "png".to_string()
    } else {
        cover
            .media_type
            .clone()
            .to_lowercase()
            .replace("image/", "")
    };

    // Create the full paths for directories and file
    let mut artist_dir = PathBuf::from(dir);
    artist_dir.push(sanitize_names(artist));

    let mut album_dir = artist_dir.clone();
    album_dir.push(sanitize_names(album));

    // Ensure the artist directory exists
    if !artist_dir.exists() {
        fs::create_dir(&artist_dir).unwrap();
    }

    // Ensure the album directory exists within the artist directory
    if !album_dir.exists() {
        fs::create_dir(&album_dir).unwrap();
    }

    // Create the destination path for copying the file
    let dest_path = {
        let mut p = album_dir.clone();
        // Go through album_dir and test if a file Cover.* exists.
        if _contains_cover_file(&album_dir) {
            //p.push(path_str.split('/').last().unwrap_or("unknown"));
            p.push(format!("Cover{}.{}", _get_timestamp(), suffix));
        } else {
            //p.push(path_str.split('/').last().unwrap_or("unknown"));
            p.push(format!("Cover.{}", suffix));
        }

        p
    };

    // Write the Image to the album subdirectory
    fs::write(dest_path.clone(), cover.data.clone()).unwrap();

    // Return the full path of the copied file as a String
    return dest_path.to_string_lossy().to_string();
}

fn ensure_and_copy_file(dir: &str, artist: &str, album: &str, path_str: &str) -> String {
    // Create the full paths for directories and file
    let mut artist_dir = PathBuf::from(dir);
    artist_dir.push(sanitize_names(artist));

    let mut album_dir = artist_dir.clone();
    album_dir.push(sanitize_names(album));

    // Ensure the artist directory exists
    if !artist_dir.exists() {
        fs::create_dir(&artist_dir).unwrap();
    }

    // Ensure the album directory exists within the artist directory
    if !album_dir.exists() {
        fs::create_dir(&album_dir).unwrap();
    }

    // Create the destination path for copying the file
    let dest_path = {
        let mut p = PathBuf::from(album_dir);
        #[cfg(not(target_os = "windows"))]
        p.push(path_str.split('/').last().unwrap_or("unknown"));
        #[cfg(target_os = "windows")]
        p.push(path_str.split('\\').last().unwrap_or("unknown"));
        p
    };

    // Copy the file to the album subdirectory
    fs::copy(path_str, &dest_path).unwrap();

    // Return the full path of the copied file as a String
    dest_path.to_string_lossy().to_string()
}

fn get_runtime_and_space(conn: &Connection, vec_ids: Option<Vec<i64>>) -> SpaceTime {
    let mut spacetime = SpaceTime {
        space: None,
        time: None,
    };

    if let Some(ids) = vec_ids {
        let mut query = r#"
                                    SELECT SUM(size) as space, SUM(total_time) as runtime
                                    FROM Tracks
                                    WHERE track_id IN (
                                "#
        .to_string();
        for _ in &ids {
            query.push_str("?,");
        }
        query.pop(); // Remove the trailing comma
        query.push(')');

        let mut stmt = conn.prepare(query).unwrap();
        for (index, track_id) in ids.iter().enumerate() {
            stmt.bind((index + 1, track_id.to_string().as_str()))
                .unwrap();
        }

        if let Ok(State::Row) = stmt.next() {
            spacetime.space = Some(stmt.read::<i64, _>("space").unwrap_or_default());
            spacetime.time = Some(stmt.read::<i64, _>("runtime").unwrap_or_default());
        }
    } else {
        let mut stmt = conn
            .prepare("SELECT SUM(size) as space, SUM(total_time) as runtime FROM Tracks")
            .unwrap();

        if let Ok(State::Row) = stmt.next() {
            spacetime.space = Some(stmt.read::<i64, _>("space").unwrap_or_default());
            spacetime.time = Some(stmt.read::<i64, _>("runtime").unwrap_or_default());
        }
    }

    return spacetime;
}

fn get_init_data(conn: &Connection) -> Data {
    let data = Data {
        queue: None,
        albums: Some(get_albums(conn)),
        tracks: Some(get_tracks(conn)),
        artists: Some(get_artists(conn)),
        artist_albums: Some(get_artist_albums(conn)),
        artist_tracks: Some(get_artist_tracks(conn)),
        composers: Some(get_composers(conn)),
        composer_tracks: Some(get_composer_tracks(conn)),
        covers: None, // excluded, takes too long... Some(get_covers(conn)),
        playlists: Some(get_playlists(conn)),
        genres: Some(get_genres(conn)),
        genre_tracks: Some(get_genre_tracks(conn)),
        spacetime: Some(get_runtime_and_space(&conn, None)),
        search: None,
        albums_order: Some((
            vec![Order::ByAddedDateInverse],
            get_albums_order(conn, vec![Order::ByAddedDateInverse]),
        )),
        artists_order: Some((
            vec![Order::ByName],
            get_artists_order(conn, vec![Order::ByName]),
        )),
        composers_order: Some((
            vec![Order::ByName],
            get_composers_order(conn, vec![Order::ByName]),
        )),
        genres_order: Some((
            vec![Order::ByName],
            get_genres_order(conn, vec![Order::ByName]),
        )),
        playlists_order: Some((
            vec![Order::ByName],
            get_playlists_order(conn, vec![Order::ByName]),
        )),
        tracks_order: Some((
            vec![Order::ByName],
            get_tracks_order(conn, vec![Order::ByName]),
        )),
        error: None,
        loading: None,
        is_init: Some(true),
    };

    /*
    TODO:
    Implement retrieval of queue from previous session.

    Implement caching of init data.
    The init data is the same it was after the last time data was added to the database.
    The number of playbacks of a track are not sent to the frontend.
    If we would send that, the data would frequently change.
     */

    return data;
}

fn get_albums(conn: &Connection) -> Vec<Album> {
    let query = r#"
    SELECT *
    FROM Albums
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut albums: Vec<Album> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let mut album = Album {
            id: stmt.read::<i64, _>("album_id").unwrap_or_default(),
            artist_id: stmt.read::<i64, _>("artist_id").unwrap_or_default(),
            name: stmt.read::<String, _>("name").unwrap_or_default(),
            sort_album: stmt.read::<String, _>("sort_album").unwrap_or_default(),
            genre_id: stmt.read::<i64, _>("genre_id").unwrap_or_default(),
            year: stmt.read::<i64, _>("year").unwrap_or_default(),
            release_date: stmt.read::<String, _>("release_date").unwrap_or_default(),
            date_modified: stmt.read::<String, _>("date_modified").unwrap_or_default(),
            date_added: stmt.read::<String, _>("date_added").unwrap_or_default(),
            tracks: vec![],
            cover_id: stmt.read::<i64, _>("cover_id").unwrap_or_default(),
        };

        let tracks_query = r#"
        SELECT track_id
        FROM Tracks
        WHERE album_id = ?1
        ORDER BY disc_number ASC, track_number ASC
        "#;
        let mut tracks_stmt = conn.prepare(tracks_query).unwrap();
        tracks_stmt.bind((1, album.id)).unwrap();

        while let Ok(State::Row) = tracks_stmt.next() {
            let track_id: i64 = tracks_stmt.read("track_id").unwrap_or_default();
            album.tracks.push(track_id);
        }

        albums.push(album);
    }

    return albums;
}

fn get_artists(conn: &Connection) -> Vec<Artist> {
    let query = r#"
    SELECT *
    FROM Artists
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut artists: Vec<Artist> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let artist = Artist {
            id: stmt.read::<i64, _>("artist_id").unwrap_or_default(),
            name: stmt.read::<String, _>("name").unwrap_or_default(),
            sort_artist: stmt.read::<String, _>("sort_artist").unwrap_or_default(),
        };

        artists.push(artist);
    }

    return artists;
}

fn get_composers(conn: &Connection) -> Vec<Composer> {
    let query = r#"
    SELECT *
    FROM Composers
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut composers: Vec<Composer> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let composer = Composer {
            id: stmt.read::<i64, _>("composer_id").unwrap_or_default(),
            name: stmt.read::<String, _>("name").unwrap_or_default(),
        };

        composers.push(composer);
    }

    return composers;
}

fn _get_covers(conn: &Connection) -> Vec<Cover> {
    let query = r#"
    SELECT *
    FROM Covers
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut covers: Vec<Cover> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let cover = Cover {
            id: stmt.read::<i64, _>("cover_id").unwrap_or_default(),
            album_id: stmt.read::<i64, _>("album_id").unwrap_or_default(),
            data: stmt.read::<String, _>("base64").unwrap_or_default(),
        };

        covers.push(cover);
    }

    return covers;
}

fn get_covers_by_id(conn: &Connection, vec_id: &Vec<i64>) -> Option<Vec<Cover>> {
    let mut query = r#"
    SELECT *
    FROM Covers
    WHERE cover_id IN (
    "#
    .to_string();
    for _ in vec_id {
        query.push_str("?,");
    }
    query.pop(); // Remove the trailing comma
    query.push(')');

    let mut stmt = conn.prepare(query).unwrap();
    for (index, cover_id) in vec_id.iter().enumerate() {
        stmt.bind((index + 1, cover_id.clone())).unwrap();
    }

    let mut covers: Vec<Cover> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let cover = Cover {
            id: stmt.read::<i64, _>("cover_id").unwrap_or_default(),
            album_id: stmt.read::<i64, _>("album_id").unwrap_or_default(),
            data: stmt.read::<String, _>("base64").unwrap_or_default(),
        };

        covers.push(cover);
    }

    if covers.len() != 0 {
        return Some(covers);
    }

    return None;
}

fn get_genres(conn: &Connection) -> Vec<Genre> {
    let query = r#"
    SELECT *
    FROM Genres
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut genres: Vec<Genre> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let genre = Genre {
            id: stmt.read::<i64, _>("genre_id").unwrap_or_default(),
            name: stmt.read::<String, _>("name").unwrap_or_default(),
        };

        genres.push(genre);
    }

    return genres;
}

fn get_tracks(conn: &Connection) -> Vec<Track> {
    let query = r#"
    SELECT *
    FROM Tracks
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut tracks: Vec<Track> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let track = Track {
            id: stmt.read::<i64, _>("track_id").unwrap_or_default(),
            name: stmt.read::<String, _>("name").unwrap_or_default(),
            artist_id: stmt.read::<i64, _>("artist_id").unwrap_or_default(),
            album_artist_id: stmt.read::<i64, _>("album_artist_id").unwrap_or_default(),
            album_id: stmt.read::<i64, _>("album_id").unwrap_or_default(),
            genre_id: stmt.read::<i64, _>("genre_id").unwrap_or_default(),
            total_time: stmt.read::<i64, _>("total_time").unwrap_or_default() as i32,
            disc_number: stmt.read::<i64, _>("disc_number").unwrap_or_default() as i32,
            track_number: stmt.read::<i64, _>("track_number").unwrap_or_default() as i32,
        };

        tracks.push(track);
    }

    return tracks;
}

fn copy_unmanaged_tracks(conn: &Connection, media_path: String) {
    let query = r#"
    SELECT track_id, album_id, album_artist_id, location
    FROM Tracks
    WHERE location NOT LIKE ?
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut location = media_path.clone();
    location.push('%');

    stmt.bind((1, location.as_str())).unwrap();

    let mut location_map: HashMap<i64, String> = HashMap::new();
    let mut album_map: HashMap<i64, String> = HashMap::new();
    let mut album_artist_map: HashMap<i64, String> = HashMap::new();

    while let Ok(State::Row) = stmt.next() {
        let track_id = stmt.read::<i64, _>("track_id").unwrap();
        let album_id = stmt.read::<i64, _>("album_id").unwrap();
        let album_artist_id = stmt.read::<i64, _>("album_artist_id").unwrap();
        let location = stmt.read::<String, _>("location").unwrap();

        let album = if let Some(album) = album_map.get(&album_id) {
            album.to_owned()
        } else {
            let album_query = r#"
            SELECT name
            FROM Albums
            WHERE album_id = ?
            "#
            .to_string();

            let mut album_stmt = conn.prepare(album_query).unwrap();
            album_stmt.bind((1, album_id)).unwrap();

            let mut album = "Unknown".to_string();

            if let Ok(State::Row) = album_stmt.next() {
                album = album_stmt.read::<String, _>("name").unwrap();
                album_map.insert(album_id, album.clone());
            }

            album
        };

        let album_artist = if let Some(album_artist) = album_artist_map.get(&album_artist_id) {
            album_artist.to_owned()
        } else {
            let album_artist_query = r#"
            SELECT name
            FROM Artists
            WHERE artist_id = ?
            "#
            .to_string();

            let mut album_artist_stmt = conn.prepare(album_artist_query).unwrap();
            album_artist_stmt.bind((1, album_artist_id)).unwrap();

            let mut album_artist = "Unknown".to_string();

            if let Ok(State::Row) = album_artist_stmt.next() {
                album_artist = album_artist_stmt.read::<String, _>("name").unwrap();
                album_artist_map.insert(album_artist_id, album_artist.clone());
            }

            album_artist
        };

        let new_location = ensure_and_copy_file(&media_path, &album_artist, &album, &location);

        location_map.insert(track_id, new_location);

        // SQLite 3.32.0 can handle up to 32766 variables
        // Updated location uses 3, thus max 10922 tracks can be batch updated
        // Leaving some headroom
        if location_map.keys().len() > 10000 {
            update_track_location_batch(conn, location_map);
            location_map = HashMap::new();
        }
    }
    // Update remaining
    update_track_location_batch(conn, location_map);
}

fn get_playlists(conn: &Connection) -> Vec<Playlist> {
    let query = r#"
    SELECT playlist_id, name, description, tracks
    FROM Playlists
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut playlists: Vec<Playlist> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let tracks_string = stmt.read::<String, _>("tracks").unwrap_or_default();

        let tracks: Vec<i64> = serde_json::from_str(&tracks_string).unwrap();

        let playlist = Playlist {
            id: stmt.read::<i64, _>("playlist_id").unwrap_or_default(),
            name: stmt.read::<String, _>("name").unwrap_or_default(),
            description: stmt.read::<String, _>("description").unwrap_or_default(),
            tracks: tracks,
        };

        playlists.push(playlist);
    }

    return playlists;
}

fn get_artist_albums(conn: &Connection) -> Vec<ArtistAlbums> {
    let query = r#"
    SELECT artist_id
    FROM Artists
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut artist_albums: Vec<ArtistAlbums> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let mut artist_album = ArtistAlbums {
            id: stmt.read::<i64, _>("artist_id").unwrap_or_default(),
            albums: vec![],
        };

        let albums_query = r#"
        SELECT album_id
        FROM Albums
        WHERE artist_id = ?1
        ORDER BY year
        "#;
        let mut albums_stmt = conn.prepare(albums_query).unwrap();
        albums_stmt.bind((1, artist_album.id)).unwrap();

        while let Ok(State::Row) = albums_stmt.next() {
            artist_album
                .albums
                .push(albums_stmt.read("album_id").unwrap_or_default());
        }

        if artist_album.albums.len() != 0 {
            artist_albums.push(artist_album);
        }
    }

    return artist_albums;
}

fn get_artist_tracks(conn: &Connection) -> Vec<ArtistTracks> {
    let query = r#"
    SELECT artist_id
    FROM Artists
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut artist_tracks: Vec<ArtistTracks> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let mut artist_track = ArtistTracks {
            id: stmt.read::<i64, _>("artist_id").unwrap_or_default(),
            tracks: vec![],
        };

        let tracks_query = r#"
        SELECT track_id
        FROM Tracks
        WHERE artist_id = ?1
        ORDER BY year, album_id, disc_number, track_number ASC
        "#;
        let mut tracks_stmt = conn.prepare(tracks_query).unwrap();
        tracks_stmt.bind((1, artist_track.id)).unwrap();

        while let Ok(State::Row) = tracks_stmt.next() {
            artist_track
                .tracks
                .push(tracks_stmt.read("track_id").unwrap_or_default());
        }

        if artist_track.tracks.len() != 0 {
            artist_tracks.push(artist_track);
        }
    }

    return artist_tracks;
}

fn get_composer_tracks(conn: &Connection) -> Vec<ComposerTracks> {
    let query = r#"
    SELECT composer_id
    FROM Composers
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut composer_tracks: Vec<ComposerTracks> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let mut composer_track = ComposerTracks {
            id: stmt.read::<i64, _>("composer_id").unwrap_or_default(),
            tracks: vec![],
        };

        let tracks_query = r#"
        SELECT track_id
        FROM Tracks
        WHERE composer_id = ?1
        "#;
        let mut tracks_stmt = conn.prepare(tracks_query).unwrap();
        tracks_stmt.bind((1, composer_track.id)).unwrap();

        while let Ok(State::Row) = tracks_stmt.next() {
            composer_track
                .tracks
                .push(tracks_stmt.read("track_id").unwrap_or_default());
        }

        if composer_track.tracks.len() != 0 {
            composer_tracks.push(composer_track);
        }
    }

    return composer_tracks;
}

fn get_genre_tracks(conn: &Connection) -> Vec<GenreTracks> {
    let query = r#"
    SELECT genre_id
    FROM Genres
    "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();

    let mut genre_tracks: Vec<GenreTracks> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let mut genre_track = GenreTracks {
            id: stmt.read::<i64, _>("genre_id").unwrap_or_default(),
            tracks: vec![],
        };

        let tracks_query = r#"
        SELECT track_id
        FROM Tracks
        WHERE genre_id = ?1
        "#;
        let mut tracks_stmt = conn.prepare(tracks_query).unwrap();
        tracks_stmt.bind((1, genre_track.id)).unwrap();

        while let Ok(State::Row) = tracks_stmt.next() {
            genre_track
                .tracks
                .push(tracks_stmt.read("track_id").unwrap_or_default());
        }

        if genre_track.tracks.len() != 0 {
            genre_tracks.push(genre_track);
        }
    }

    return genre_tracks;
}

fn get_albums_order(conn: &Connection, vec_order: Vec<Order>) -> Vec<i64> {
    /*
    valid orderings are
    ByName,
    ByReleaseDate,
    ByAddedDate,
    ByAlbumArtist,
    ByAlbum, same as by added data
    ByGenre,
     */
    let mut query = r#"
    SELECT album_id
    FROM Albums
    LEFT JOIN Genres ON Albums.genre_id = Genres.genre_id
    LEFT JOIN Artists ON Albums.artist_id = Artists.artist_id
    ORDER BY
    "#
    .to_string();
    for order in vec_order {
        match order {
            Order::ByName => {
                query.push_str(" LOWER(Albums.name) ASC,");
            }
            Order::ByReleaseDate => {
                query.push_str(" release_date ASC,");
            }
            Order::ByAddedDate => {
                query.push_str(" date_added ASC,");
            }
            Order::ByAlbumArtist => {
                // requires subquery or join, since this would order by id, not artist name
                query.push_str(" LOWER(Artists.name) ASC,");
            }
            Order::ByGenre => {
                // requires subquery or join, since this would order by id, not genre name
                query.push_str(" LOWER(Genres.name) ASC,");
            }
            Order::ByNameInverse => {
                query.push_str(" LOWER(Albums.name) DESC,");
            }
            Order::ByReleaseDateInverse => {
                query.push_str(" release_date DESC,");
            }
            Order::ByAddedDateInverse => {
                query.push_str(" date_added DESC,");
            }
            Order::ByAlbumArtistInverse => {
                // requires subquery or join, since this would order by id, not artist name
                query.push_str(" LOWER(Artists.name) DESC,");
            }
            Order::ByGenreInverse => {
                // requires subquery or join, since this would order by id, not genre name
                query.push_str(" LOWER(Genres.name) DESC,");
            }
            _ => {}
        }
    }
    query.pop(); // Remove the trailing comma

    let mut stmt = conn.prepare(query).unwrap();

    let mut album_ids: Vec<i64> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        album_ids.push(stmt.read::<i64, _>("album_id").unwrap_or_default());
    }

    return album_ids;
}

fn get_artists_order(conn: &Connection, vec_order: Vec<Order>) -> Vec<i64> {
    // TODO: exclude if the artist id is not in albums artist_id column? Since then it would only be an artist for a track
    /*
    TODO:
    only valid ordering is by ByName & ByNameInverse
    We can get info about date added, date modifed with subqueries in albums or tracks table
     */
    let mut query = r#"
    SELECT artist_id
    FROM Artists
    ORDER BY
    "#
    .to_string();
    for order in vec_order {
        match order {
            Order::ByName => {
                query.push_str(" LOWER(name) ASC,");
            }
            Order::ByNameInverse => {
                query.push_str(" LOWER(name) DESC,");
            }
            _ => {}
        }
    }
    query.pop(); // Remove the trailing comma

    let mut stmt = conn.prepare(query).unwrap();

    let mut artist_ids: Vec<i64> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        artist_ids.push(stmt.read::<i64, _>("artist_id").unwrap_or_default());
    }

    return artist_ids;
}

fn get_composers_order(conn: &Connection, vec_order: Vec<Order>) -> Vec<i64> {
    /*
    TODO:
    only valid ordering is by ByName & ByNameInverse
    We can get info about date added, date modifed with subqueries in albums or tracks table
     */
    let mut query = r#"
    SELECT composer_id
    FROM Composers
    ORDER BY
    "#
    .to_string();
    for order in vec_order {
        match order {
            Order::ByName => {
                query.push_str(" LOWER(name) ASC,");
            }
            Order::ByNameInverse => {
                query.push_str(" LOWER(name) DESC,");
            }
            _ => {}
        }
    }
    query.pop(); // Remove the trailing comma

    let mut stmt = conn.prepare(query).unwrap();

    let mut composer_ids: Vec<i64> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        composer_ids.push(stmt.read::<i64, _>("composer_id").unwrap_or_default());
    }

    return composer_ids;
}

fn get_genres_order(conn: &Connection, vec_order: Vec<Order>) -> Vec<i64> {
    /*
    TODO:
    only valid ordering is ByName & ByNameInverse
    We can get info about date added, date modifed with subqueries in albums or tracks table
     */
    let mut query = r#"
    SELECT genre_id
    FROM Genres
    ORDER BY
    "#
    .to_string();
    for order in vec_order {
        match order {
            Order::ByName => {
                query.push_str(" LOWER(name) ASC,");
            }
            Order::ByNameInverse => {
                query.push_str(" LOWER(name) DESC,");
            }
            _ => {}
        }
    }
    query.pop(); // Remove the trailing comma

    let mut stmt = conn.prepare(query).unwrap();

    let mut genre_ids: Vec<i64> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        genre_ids.push(stmt.read::<i64, _>("genre_id").unwrap_or_default());
    }

    return genre_ids;
}

fn get_playlists_order(conn: &Connection, vec_order: Vec<Order>) -> Vec<i64> {
    /*
    TODO:
    valid orderings are
    ByName,
    ByAddedDate,
    ByModifiedDate,
    ByNameInverse,
    ByAddedDateInverse,
    ByModifiedDateInverse,
     */
    let mut query = r#"
    SELECT playlist_id
    FROM Playlists
    ORDER BY
    "#
    .to_string();
    for order in vec_order {
        match order {
            Order::ByName => {
                query.push_str(" LOWER(name) ASC,");
            }
            Order::ByAddedDate => {
                query.push_str(" date_added ASC,");
            }
            Order::ByModifiedDate => {
                query.push_str(" date_modified ASC,");
            }
            Order::ByNameInverse => {
                query.push_str(" LOWER(name) DESC,");
            }
            Order::ByAddedDateInverse => {
                query.push_str(" date_added DESC,");
            }
            Order::ByModifiedDateInverse => {
                query.push_str(" date_modified DESC,");
            }
            _ => {}
        }
    }
    query.pop(); // Remove the trailing comma

    let mut stmt = conn.prepare(query).unwrap();

    let mut playlist_ids: Vec<i64> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        playlist_ids.push(stmt.read::<i64, _>("playlist_id").unwrap_or_default());
    }

    return playlist_ids;
}

fn get_tracks_order(conn: &Connection, vec_order: Vec<Order>) -> Vec<i64> {
    /*
    TODO:
    valid orderings are
    ByName,
    ByReleaseDate,
    ByAddedDate,
    ByArtist,
    ByAlbumArtist,
    ByComposer,
    ByAlbum,
    ByGenre,
    BySize,
    ByTime,
    ByNameInverse,
    ByReleaseDateInverse,
    ByAddedDateInverse,
    ByArtistInverse,
    ByAlbumArtistInverse,
    ByComposerInverse,
    ByAlbumInverse,
    ByGenreInverse,
    BySizeInverse,
    ByTimeInverse,
     */
    let mut query = r#"
    SELECT track_id
    FROM Tracks
    LEFT JOIN Artists Artist ON Tracks.artist_id = Artist.artist_id
    LEFT JOIN Artists AlbumArtist ON Tracks.album_artist_id = AlbumArtist.artist_id
    LEFT JOIN Composers ON Tracks.composer_id = Composers.composer_id
    LEFT JOIN Albums ON Tracks.album_id = Albums.album_id
    LEFT JOIN Genres ON Tracks.genre_id = Genres.genre_id
    ORDER BY
    "#
    .to_string();
    for order in vec_order {
        match order {
            Order::ByName => {
                query.push_str(" LOWER(Tracks.name) ASC,");
            }
            Order::ByReleaseDate => {
                query.push_str(" Tracks.release_date ASC,");
            }
            Order::ByAddedDate => {
                query.push_str(" Tracks.date_added ASC,");
            }
            Order::ByArtist => {
                // requires subquery or join, since this would order by id, not artist name
                query.push_str(" LOWER(Artist.name) ASC,");
            }
            Order::ByAlbumArtist => {
                // requires subquery or join, since this would order by id, not album_artist name
                query.push_str(" LOWER(AlbumArtist.name) ASC,");
            }
            Order::ByComposer => {
                // requires subquery or join, since this would order by id, not composer name
                query.push_str(" LOWER(Composers.name) ASC,");
            }
            Order::ByAlbum => {
                // requires subquery or join, since this would order by id, not album name
                query.push_str(" LOWER(Albums.name) ASC,");
            }
            Order::ByGenre => {
                // requires subquery or join, since this would order by id, not genre name
                query.push_str(" LOWER(Genres.name) ASC,");
            }
            Order::BySize => {
                query.push_str(" size ASC,");
            }
            Order::ByTime => {
                query.push_str(" total_time ASC,");
            }
            Order::ByNameInverse => {
                query.push_str(" LOWER(Tracks.name) DESC,");
            }
            Order::ByReleaseDateInverse => {
                query.push_str(" Tracks.release_date DESC,");
            }
            Order::ByAddedDateInverse => {
                query.push_str(" Tracks.date_added DESC,");
            }
            Order::ByArtistInverse => {
                // requires subquery or join, since this would order by id, not artist name
                query.push_str(" LOWER(Artist.name) DESC,");
            }
            Order::ByAlbumArtistInverse => {
                // requires subquery or join, since this would order by id, not album_artist name
                query.push_str(" LOWER(AlbumArtist.name) DESC,");
            }
            Order::ByComposerInverse => {
                // requires subquery or join, since this would order by id, not composer name
                query.push_str(" LOWER(Composers.name) DESC,");
            }
            Order::ByAlbumInverse => {
                // requires subquery or join, since this would order by id, not album name
                query.push_str(" LOWER(Albums.name) DESC,");
            }
            Order::ByGenreInverse => {
                // requires subquery or join, since this would order by id, not genre name
                query.push_str(" LOWER(Genres.name) DESC,");
            }
            Order::BySizeInverse => {
                query.push_str(" size DESC,");
            }
            Order::ByTimeInverse => {
                query.push_str(" total_time DESC,");
            }
            _ => {}
        }
    }
    query.pop(); // Remove the trailing comma

    let mut stmt = conn.prepare(query).unwrap();

    let mut track_ids: Vec<i64> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        track_ids.push(stmt.read::<i64, _>("track_id").unwrap_or_default());
    }

    return track_ids;
}

fn get_track_by_id(conn: &Connection, track_id: i64) -> Option<Track> {
    let query = r#"
        SELECT *
        FROM Tracks
        WHERE track_id = ?1
        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, track_id)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        let track = Track {
            id: stmt.read::<i64, _>("track_id").unwrap_or_default(),
            name: stmt.read::<String, _>("name").unwrap_or_default(),
            artist_id: stmt.read::<i64, _>("artist_id").unwrap_or_default(),
            album_artist_id: stmt.read::<i64, _>("album_artist_id").unwrap_or_default(),
            album_id: stmt.read::<i64, _>("album_id").unwrap_or_default(),
            genre_id: stmt.read::<i64, _>("genre_id").unwrap_or_default(),
            total_time: stmt.read::<i64, _>("total_time").unwrap_or_default() as i32,
            disc_number: stmt.read::<i64, _>("disc_number").unwrap_or_default() as i32,
            track_number: stmt.read::<i64, _>("track_number").unwrap_or_default() as i32,
        };

        return Some(track);
    }

    return None;
}

fn get_artist_by_id(conn: &Connection, artist_id: i64) -> Option<Artist> {
    let query = r#"
        SELECT *
        FROM Artists
        WHERE artist_id = ?1
        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, artist_id)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        let artist = Artist {
            id: stmt.read::<i64, _>("artist_id").unwrap_or_default(),
            name: stmt.read::<String, _>("name").unwrap_or_default(),
            sort_artist: stmt.read::<String, _>("sort_artist").unwrap_or_default(),
        };

        return Some(artist);
    }

    return None;
}

fn get_album_by_id(conn: &Connection, album_id: i64) -> Option<Album> {
    let query = r#"
        SELECT *
        FROM Albums
        WHERE album_id = ?1
        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, album_id)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        let mut album = Album {
            id: stmt.read::<i64, _>("album_id").unwrap_or_default(),
            artist_id: stmt.read::<i64, _>("artist_id").unwrap_or_default(),
            name: stmt.read::<String, _>("name").unwrap_or_default(),
            sort_album: stmt.read::<String, _>("sort_album").unwrap_or_default(),
            genre_id: stmt.read::<i64, _>("genre_id").unwrap_or_default(),
            year: stmt.read::<i64, _>("year").unwrap_or_default(),
            release_date: stmt.read::<String, _>("release_date").unwrap_or_default(),
            date_modified: stmt.read::<String, _>("date_modified").unwrap_or_default(),
            date_added: stmt.read::<String, _>("date_added").unwrap_or_default(),
            tracks: vec![],
            cover_id: stmt.read::<i64, _>("cover_id").unwrap_or_default(),
        };

        let tracks_query = r#"
            SELECT track_id
            FROM Tracks
            WHERE album_id = ?1
            ORDER BY disc_number ASC, track_number ASC
            "#;
        let mut tracks_stmt = conn.prepare(tracks_query).unwrap();
        tracks_stmt.bind((1, album.id)).unwrap();

        while let Ok(State::Row) = tracks_stmt.next() {
            let track_id: i64 = tracks_stmt.read("track_id").unwrap_or_default();
            album.tracks.push(track_id);
        }

        return Some(album);
    }

    return None;
}

fn get_genre_by_id(conn: &Connection, genre_id: i64) -> Option<Genre> {
    let query = r#"
        SELECT *
        FROM Genres
        WHERE genre_id = ?1
        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, genre_id)).unwrap();

    if let Ok(State::Row) = stmt.next() {
        let genre = Genre {
            id: stmt.read::<i64, _>("genre_id").unwrap_or_default(),
            name: stmt.read::<String, _>("name").unwrap_or_default(),
        };

        return Some(genre);
    }

    return None;
}

fn cover_as_base64(cover: &Image) -> String {
    println!("load_cover_as_base64: {:?}", cover.media_type);

    let base64_string = general_purpose::STANDARD.encode(cover.data.clone());
    return format!("data:{};base64,{}", cover.media_type, base64_string);
}

fn extract_cover_by_album(conn: &Connection, album_ids: Vec<i64>) {
    for album_id in album_ids {
        let mut stmt = conn
            .prepare(
                "SELECT *
                FROM Tracks
                WHERE album_id = ?1",
            )
            .unwrap();
        stmt.bind((1, album_id)).unwrap();

        let mut locations: Vec<String> = vec![];

        while let Ok(State::Row) = stmt.next() {
            locations.push(stmt.read::<String, _>("location").unwrap());
        }

        for location in locations {
            eprintln!("extract location: {}", location.clone());
            let path = Path::new(&location);

            if let Some(meta) = extract_metadata(path) {
                if let Some(ref cover) = meta.cover {
                    let cover_id = get_or_create_cover_id(&conn, album_id, cover);

                    let mut stmt = conn
                        .prepare(
                            "UPDATE Albums
                            SET cover_id = ?1
                            WHERE album_id = ?2",
                        )
                        .unwrap();
                    stmt.bind((1, cover_id)).unwrap();
                    stmt.bind((2, album_id)).unwrap();
                    let _ = stmt.next();

                    break;
                }
            }
        }
    }
}

fn search_tracks(conn: &Connection, search: String, limit: i64) -> Option<Vec<i64>> {
    let query = r#"
                        SELECT track_id
                        FROM Tracks
                        WHERE name LIKE ?1
                        LIMIT ?2
                        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, format!("%{}%", search).as_str())).unwrap();
    stmt.bind((2, limit)).unwrap();

    let mut track_ids: Vec<i64> = vec![];

    while let Ok(State::Row) = stmt.next() {
        track_ids.push(stmt.read::<i64, _>("track_id").unwrap_or_default());
        if track_ids.len() == limit as usize {
            break;
        }
    }

    if track_ids.len() != 0 {
        return Some(track_ids);
    }

    return None;
}

fn search_albums(conn: &Connection, search: String, limit: i64) -> Option<Vec<i64>> {
    let query = r#"
                        SELECT album_id
                        FROM Albums
                        WHERE name LIKE ?1
                        LIMIT ?2
                        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, format!("%{}%", search).as_str())).unwrap();
    stmt.bind((2, limit)).unwrap();

    let mut album_ids: Vec<i64> = vec![];

    while let Ok(State::Row) = stmt.next() {
        album_ids.push(stmt.read::<i64, _>("album_id").unwrap_or_default());
        if album_ids.len() == limit as usize {
            break;
        }
    }

    if album_ids.len() != 0 {
        return Some(album_ids);
    }

    return None;
}

fn search_artists(conn: &Connection, search: String, limit: i64) -> Option<Vec<i64>> {
    let query = r#"
                        SELECT artist_id
                        FROM Artists
                        WHERE name LIKE ?1
                        LIMIT ?2
                        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, format!("%{}%", search).as_str())).unwrap();
    stmt.bind((2, limit)).unwrap();

    let mut artist_ids: Vec<i64> = vec![];

    while let Ok(State::Row) = stmt.next() {
        artist_ids.push(stmt.read::<i64, _>("artist_id").unwrap_or_default());
        if artist_ids.len() == limit as usize {
            break;
        }
    }

    if artist_ids.len() != 0 {
        return Some(artist_ids);
    }

    return None;
}

fn search_composers(conn: &Connection, search: String, limit: i64) -> Option<Vec<i64>> {
    let query = r#"
                        SELECT composer_id
                        FROM Composers
                        WHERE name LIKE ?1
                        LIMIT ?2
                        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, format!("%{}%", search).as_str())).unwrap();
    stmt.bind((2, limit)).unwrap();

    let mut composer_ids: Vec<i64> = vec![];

    while let Ok(State::Row) = stmt.next() {
        composer_ids.push(stmt.read::<i64, _>("composer_id").unwrap_or_default());
        if composer_ids.len() == limit as usize {
            break;
        }
    }

    if composer_ids.len() != 0 {
        return Some(composer_ids);
    }

    return None;
}

fn search_genres(conn: &Connection, search: String, limit: i64) -> Option<Vec<i64>> {
    let query = r#"
                        SELECT genre_id
                        FROM Genres
                        WHERE name LIKE ?1
                        LIMIT ?2
                        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, format!("%{}%", search).as_str())).unwrap();
    stmt.bind((2, limit)).unwrap();

    let mut genre_ids: Vec<i64> = vec![];

    while let Ok(State::Row) = stmt.next() {
        genre_ids.push(stmt.read::<i64, _>("genre_id").unwrap_or_default());
        if genre_ids.len() == limit as usize {
            break;
        }
    }

    if genre_ids.len() != 0 {
        return Some(genre_ids);
    }

    return None;
}

fn search_playlists(conn: &Connection, search: String, limit: i64) -> Option<Vec<i64>> {
    let query = r#"
                        SELECT playlist_id
                        FROM Playlists
                        WHERE name LIKE ?1
                        LIMIT ?2
                        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, format!("%{}%", search).as_str())).unwrap();
    stmt.bind((2, limit)).unwrap();

    let mut playlist_ids: Vec<i64> = vec![];

    while let Ok(State::Row) = stmt.next() {
        playlist_ids.push(stmt.read::<i64, _>("playlist_id").unwrap_or_default());
        if playlist_ids.len() == limit as usize {
            break;
        }
    }

    if playlist_ids.len() != 0 {
        return Some(playlist_ids);
    }

    return None;
}

fn get_audiotracks_by_id(conn: &Connection, vec_id: &Vec<i64>) -> Option<Vec<AudioTrack>> {
    /*
    TODO:
    What happens if a track_id in vec_id does not exist? Should never happen.
     */
    let mut query = r#"
        SELECT track_id, location
        FROM Tracks
        WHERE track_id IN (
        "#
    .to_string();
    for _ in vec_id {
        query.push_str("?,");
    }
    query.pop(); // Remove the trailing comma
    query.push(')');

    let mut stmt = conn.prepare(query).unwrap();
    for (index, track_id) in vec_id.iter().enumerate() {
        stmt.bind((index + 1, track_id.clone())).unwrap();
    }

    let mut audiotracks: Vec<AudioTrack> = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        let audiotrack = AudioTrack {
            id: stmt.read::<i64, _>("track_id").unwrap_or_default(),
            location: stmt.read::<String, _>("location").unwrap_or_default(),
        };

        audiotracks.push(audiotrack);
    }

    if audiotracks.len() == 0 {
        return None;
    } else {
        // Create a lookup map from ID to the desired position
        let index_map: std::collections::HashMap<_, _> = vec_id
            .iter()
            .enumerate()
            .map(|(index, &id)| (id, index))
            .collect();

        // Sort audiotracks based on their position in vec_id
        audiotracks.sort_by_key(|track| index_map[&track.id]);
        // TODO: test if they are actually ordered correctly.
        // TODO: is sorting even required?
        return Some(audiotracks);
    }
}

fn get_audiotracks_from_album_by_id(
    conn: &Connection,
    vec_id: &Vec<i64>,
) -> Option<Vec<AudioTrack>> {
    let query = r#"
    SELECT location, track_id
    FROM Tracks
    WHERE album_id = ?1
    ORDER BY disc_number, track_number
    "#
    .to_string();
    let mut audiotracks: Vec<AudioTrack> = Vec::new();

    for album_id in vec_id {
        let mut stmt = conn.prepare(&query).unwrap();
        stmt.bind((1, album_id.clone())).unwrap();

        while let Ok(State::Row) = stmt.next() {
            let audiotrack = AudioTrack {
                id: stmt.read::<i64, _>("track_id").unwrap(),
                location: stmt.read::<String, _>("location").unwrap(),
            };
            audiotracks.push(audiotrack);
        }
    }

    if audiotracks.len() == 0 {
        return None;
    } else {
        return Some(audiotracks);
    }
}

fn get_audiotracks_from_playlist_by_id(
    conn: &Connection,
    vec_id: &Vec<i64>,
) -> Option<Vec<AudioTrack>> {
    let query = r#"
    SELECT tracks
    FROM Playlists
    WHERE playlist_id = ?1
    "#
    .to_string();

    let mut audiotracks: Vec<AudioTrack> = Vec::new();

    for playlist_id in vec_id {
        let mut stmt = conn.prepare(&query).unwrap();
        stmt.bind((1, playlist_id.clone())).unwrap();

        while let Ok(State::Row) = stmt.next() {
            let tracks_string = stmt.read::<String, _>("tracks").unwrap_or_default();

            let track_ids: Vec<i64> = serde_json::from_str(&tracks_string).unwrap();

            if let Some(audiotracks_from_ids) = get_audiotracks_by_id(conn, &track_ids) {
                audiotracks.append(&mut audiotracks_from_ids.clone());
            };
        }
    }

    if audiotracks.len() == 0 {
        return None;
    } else {
        return Some(audiotracks);
    }
}

fn get_audiotracks_from_artist_by_id(
    conn: &Connection,
    vec_id: &Vec<i64>,
) -> Option<Vec<AudioTrack>> {
    // TODO: sort by album name? year? release_date? date_added?
    // Default sort by year, album, disk number, track number
    let query = r#"
    SELECT track_id, location
    FROM Tracks
    WHERE artist_id = ?1 OR album_artist_id = ?1
    ORDER BY year, album_id, disc_number, track_number
    "#
    .to_string();
    let mut audiotracks: Vec<AudioTrack> = Vec::new();

    for artist_id in vec_id {
        let mut stmt = conn.prepare(&query).unwrap();
        stmt.bind((1, artist_id.clone())).unwrap();

        while let Ok(State::Row) = stmt.next() {
            let audiotrack = AudioTrack {
                id: stmt.read::<i64, _>("track_id").unwrap(),
                location: stmt.read::<String, _>("location").unwrap(),
            };
            audiotracks.push(audiotrack);
        }
    }

    if audiotracks.len() == 0 {
        return None;
    } else {
        return Some(audiotracks);
    }
}

fn get_audiotracks_from_composer_by_id_and_order(
    conn: &Connection,
    vec_id: &Vec<i64>,
    vec_order: Vec<Order>,
) -> Option<Vec<AudioTrack>> {
    /*
    TODO:
    valid orderings are
    ByName,
    ByReleaseDate,
    ByAddedDate,
    ByArtist,
    ByAlbumArtist,
    ByAlbum,
    ByGenre,
    BySize,
    ByTime,
    ByNameInverse,
    ByReleaseDateInverse,
    ByAddedDateInverse,
    ByArtistInverse,
    ByAlbumArtistInverse,
    ByAlbumInverse,
    ByGenreInverse,
    BySizeInverse,
    ByTimeInverse,
     */
    let mut query = r#"
    SELECT track_id, location
    FROM Tracks
    LEFT JOIN Artists Artist ON Tracks.artist_id = Artist.artist_id
    LEFT JOIN Artists AlbumArtist ON Tracks.album_artist_id = AlbumArtist.artist_id
    LEFT JOIN Composers ON Tracks.composer_id = Composers.composer_id
    LEFT JOIN Albums ON Tracks.album_id = Albums.album_id
    LEFT JOIN Genres ON Tracks.genre_id = Genres.genre_id
    WHERE Tracks.composer_id = ?1
    ORDER BY
    "#
    .to_string();
    for order in vec_order {
        match order {
            Order::ByName => {
                query.push_str(" LOWER(Tracks.name) ASC,");
            }
            Order::ByReleaseDate => {
                query.push_str(" Tracks.release_date ASC,");
            }
            Order::ByAddedDate => {
                query.push_str(" Tracks.date_added ASC,");
            }
            Order::ByArtist => {
                // requires subquery or join, since this would order by id, not artist name
                query.push_str(" LOWER(Artist.name) ASC,");
            }
            Order::ByAlbumArtist => {
                // requires subquery or join, since this would order by id, not album_artist name
                query.push_str(" LOWER(AlbumArtist.name) ASC,");
            }
            Order::ByAlbum => {
                // requires subquery or join, since this would order by id, not album name
                query.push_str(" LOWER(Albums.name) ASC,");
            }
            Order::ByGenre => {
                // requires subquery or join, since this would order by id, not genre name
                query.push_str(" LOWER(Genres.name) ASC,");
            }
            Order::BySize => {
                query.push_str(" size ASC,");
            }
            Order::ByTime => {
                query.push_str(" total_time ASC,");
            }
            Order::ByNameInverse => {
                query.push_str(" LOWER(Tracks.name) DESC,");
            }
            Order::ByReleaseDateInverse => {
                query.push_str(" Tracks.release_date DESC,");
            }
            Order::ByAddedDateInverse => {
                query.push_str(" Tracks.date_added DESC,");
            }
            Order::ByArtistInverse => {
                // requires subquery or join, since this would order by id, not artist name
                query.push_str(" LOWER(Artist.name) DESC,");
            }
            Order::ByAlbumArtistInverse => {
                // requires subquery or join, since this would order by id, not album_artist name
                query.push_str(" LOWER(AlbumArtist.name) DESC,");
            }
            Order::ByAlbumInverse => {
                // requires subquery or join, since this would order by id, not album name
                query.push_str(" LOWER(Albums.name) DESC,");
            }
            Order::ByGenreInverse => {
                // requires subquery or join, since this would order by id, not genre name
                query.push_str(" LOWER(Genres.name) DESC,");
            }
            Order::BySizeInverse => {
                query.push_str(" size DESC,");
            }
            Order::ByTimeInverse => {
                query.push_str(" total_time DESC,");
            }
            _ => {}
        }
    }
    query.pop(); // Remove the trailing comma

    let mut audiotracks: Vec<AudioTrack> = Vec::new();

    for composer_id in vec_id {
        let mut stmt = conn.prepare(&query).unwrap();
        stmt.bind((1, composer_id.clone())).unwrap();

        while let Ok(State::Row) = stmt.next() {
            let audiotrack = AudioTrack {
                id: stmt.read::<i64, _>("track_id").unwrap(),
                location: stmt.read::<String, _>("location").unwrap(),
            };
            audiotracks.push(audiotrack);
        }
    }

    if audiotracks.len() == 0 {
        return None;
    } else {
        return Some(audiotracks);
    }
}

fn get_audiotracks_from_genre_by_id_and_order(
    conn: &Connection,
    vec_id: &Vec<i64>,
    vec_order: Vec<Order>,
) -> Option<Vec<AudioTrack>> {
    /*
    TODO:
    valid orderings are
    ByName,
    ByReleaseDate,
    ByAddedDate,
    ByArtist,
    ByAlbumArtist,
    ByAlbum,
    BySize,
    ByTime,
    ByNameInverse,
    ByReleaseDateInverse,
    ByAddedDateInverse,
    ByArtistInverse,
    ByAlbumArtistInverse,
    ByComposerInverse,
    ByAlbumInverse,
    BySizeInverse,
    ByTimeInverse,
     */
    let mut query = r#"
    SELECT track_id, location
    FROM Tracks
    LEFT JOIN Artists Artist ON Tracks.artist_id = Artist.artist_id
    LEFT JOIN Artists AlbumArtist ON Tracks.album_artist_id = AlbumArtist.artist_id
    LEFT JOIN Albums ON Tracks.album_id = Albums.album_id
    WHERE Tracks.genre_id = ?1
    ORDER BY
    "#
    .to_string();
    for order in vec_order {
        match order {
            Order::ByName => {
                query.push_str(" LOWER(Tracks.name) ASC,");
            }
            Order::ByReleaseDate => {
                query.push_str(" Tracks.release_date ASC,");
            }
            Order::ByAddedDate => {
                query.push_str(" Tracks.date_added ASC,");
            }
            Order::ByArtist => {
                // requires subquery or join, since this would order by id, not artist name
                query.push_str(" LOWER(Artist.name) ASC,");
            }
            Order::ByAlbumArtist => {
                // requires subquery or join, since this would order by id, not album_artist name
                query.push_str(" LOWER(AlbumArtist.name) ASC,");
            }
            Order::ByComposer => {
                // requires subquery or join, since this would order by id, not composer name
                query.push_str(" LOWER(Composers.name) ASC,");
            }
            Order::ByAlbum => {
                // requires subquery or join, since this would order by id, not album name
                query.push_str(" LOWER(Albums.name) ASC,");
            }
            Order::BySize => {
                query.push_str(" size ASC,");
            }
            Order::ByTime => {
                query.push_str(" total_time ASC,");
            }
            Order::ByNameInverse => {
                query.push_str(" LOWER(Tracks.name) DESC,");
            }
            Order::ByReleaseDateInverse => {
                query.push_str(" Tracks.release_date DESC,");
            }
            Order::ByAddedDateInverse => {
                query.push_str(" Tracks.date_added DESC,");
            }
            Order::ByArtistInverse => {
                // requires subquery or join, since this would order by id, not artist name
                query.push_str(" LOWER(Artist.name) DESC,");
            }
            Order::ByAlbumArtistInverse => {
                // requires subquery or join, since this would order by id, not album_artist name
                query.push_str(" LOWER(AlbumArtist.name) DESC,");
            }
            Order::ByComposerInverse => {
                // requires subquery or join, since this would order by id, not composer name
                query.push_str(" LOWER(Composers.name) DESC,");
            }
            Order::ByAlbumInverse => {
                // requires subquery or join, since this would order by id, not album name
                query.push_str(" LOWER(Albums.name) DESC,");
            }
            Order::BySizeInverse => {
                query.push_str(" size DESC,");
            }
            Order::ByTimeInverse => {
                query.push_str(" total_time DESC,");
            }
            _ => {}
        }
    }
    query.pop(); // Remove the trailing comma

    let mut audiotracks: Vec<AudioTrack> = Vec::new();

    for genre_id in vec_id {
        let mut stmt = conn.prepare(&query).unwrap();
        stmt.bind((1, genre_id.clone())).unwrap();

        while let Ok(State::Row) = stmt.next() {
            let audiotrack = AudioTrack {
                id: stmt.read::<i64, _>("track_id").unwrap(),
                location: stmt.read::<String, _>("location").unwrap(),
            };
            audiotracks.push(audiotrack);
        }
    }

    if audiotracks.len() == 0 {
        return None;
    } else {
        return Some(audiotracks);
    }
}

fn update_media_path(conn: &Connection, old_path: String, new_path: String) {
    // only updates tracks starting with the old media path, leaves others alone
    let query = r#"
        SELECT track_id, location
        FROM Tracks
        "#
    .to_string();
    // TODO: use LIKE to only select relevant tracks

    debug!("old_path: {}\nnew_path: {}", old_path, new_path);

    let mut location_map: HashMap<i64, String> = HashMap::new();

    let mut stmt = conn.prepare(&query).unwrap();

    while let Ok(State::Row) = stmt.next() {
        location_map.insert(
            stmt.read::<i64, _>("track_id").unwrap(),
            stmt.read::<String, _>("location")
                .unwrap()
                .replace(&old_path, &new_path),
        );

        // SQLite 3.32.0 can handle up to 32766 variables
        // Updated location uses 3, thus max 10922 tracks can be batch updated
        // Leaving some headroom
        if location_map.keys().len() > 10000 {
            update_track_location_batch(conn, location_map);
            location_map = HashMap::new();
        }
    }
    // Update remaining
    update_track_location_batch(conn, location_map);
}

fn update_track_location_batch(conn: &Connection, location_map: HashMap<i64, String>) {
    if location_map.len() != 0 {
        let mut update_query = r#"
        UPDATE Tracks
        SET location = CASE track_id
        "#
        .to_string();
        let mut end_query = r#"END
        WHERE track_id IN ("#
            .to_string();

        /*
        TODO:
        The following commented code does compile and seemingly run, but the locations are not updated correctly...
        Most are left unchanged, others are set to NULL.
        for _ in location_map.keys() {
            update_query.push_str("WHEN ? THEN (?)\n");
            end_query.push_str(" ?,");
        }
        The below code works, but is not using prepared statements.
        */
        for (track_id, location) in location_map.iter() {
            update_query.push_str(format!("WHEN {} THEN \"{}\"\n", track_id, location).as_str());
            end_query.push_str(format!(" {},", track_id).as_str());
        }
        end_query.pop();
        end_query.push(')');

        update_query.push_str(&end_query);

        let mut update_stmt = conn.prepare(&update_query).unwrap();

        /*for (index, (track_id, location)) in location_map.iter().enumerate() {
            let offset = index * 3;
            update_stmt.bind((offset + 1, track_id.to_owned())).unwrap();
            update_stmt.bind((offset + 2, location.as_str())).unwrap();
            update_stmt.bind((offset + 3, track_id.to_owned())).unwrap();
        }*/

        update_stmt.next().unwrap();
        println!(
            "updated track locations batch ({})",
            location_map.keys().len()
        );
    }
}

fn longest_common_path(paths: Vec<String>) -> Option<String> {
    if paths.is_empty() {
        return None;
    }

    // Convert the first path to a PathBuf
    let mut common_path = PathBuf::from(&paths[0]);

    for path in &paths[1..] {
        // Convert the current path to a PathBuf
        let current_path = PathBuf::from(path);

        // Iterate through components of both paths and keep only the common ones
        let mut iter1 = common_path.components().peekable();
        let mut iter2 = current_path.components().peekable();

        let mut new_common_path = PathBuf::new();

        while let (Some(component1), Some(component2)) = (iter1.peek(), iter2.peek()) {
            if component1 != component2 {
                break;
            }
            new_common_path.push(iter1.next().unwrap());
            iter2.next();
        }

        common_path = new_common_path;
    }

    println!("common path: {:?}", common_path.clone());

    // Convert PathBuf back to String
    Some(common_path.to_string_lossy().into_owned())
}

fn is_db_or_init(conn: &Connection) {
    // Check if a table exists in the database
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='Version'")
        .unwrap();

    // If the table exists stmt.next().unwrap() results in a row, otherwise in done.
    match stmt.next().unwrap() {
        State::Row => {
            // Check DB version
            let mut version_stmt = conn
                .prepare("SELECT major, minor, patch FROM Version WHERE ROWID IN ( SELECT max( ROWID ) FROM Version )")
                .unwrap();
            match version_stmt.next().unwrap() {
                State::Row => {
                    let major = version_stmt.read::<i64, _>("major").unwrap_or_default();
                    let minor = version_stmt.read::<i64, _>("minor").unwrap_or_default();
                    let patch = version_stmt.read::<i64, _>("patch").unwrap_or_default();

                    if DB_MAJOR != major || DB_MINOR != minor || DB_PATCH != patch {
                        update_db_version(conn, major, minor, patch);
                    }
                }
                State::Done => {
                    // TODO: we should never be here! Probably send warning to frontend
                    println!("Version table exists, but no enties!");
                }
            }
        }
        State::Done => {
            // Values table does not exist or has no entries, need to init db.
            init_db(&conn);
        }
    }
}

fn init_db(conn: &Connection) {
    let sql_statements = "
        CREATE TABLE Version (
            major INTEGER NOT NULL,
            minor INTEGER NOT NULL,
            patch INTEGER NOT NULL,
            applied TEXT DEFAULT CURRENT_TIMESTAMP,
            comment TEXT
        );
        CREATE TABLE Tracks (
            track_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
            orig_track_id INTEGER NOT NULL,
            name TEXT,
            artist_id INTEGER REFERENCES Artists(artist_id),
            album_artist_id INTEGER REFERENCES Artists(artist_id),
            composer_id INTEGER REFERENCES Composers(composer_id),
            album_id INTEGER REFERENCES Albums(album_id),
            genre_id INTEGER REFERENCES Genres(genre_id),
            kind TEXT,
            size INTEGER,
            total_time INTEGER,
            disc_number INTEGER,
            disc_count INTEGER,
            track_number INTEGER,
            track_count INTEGER,
            year INTEGER,
            date_modified TEXT DEFAULT CURRENT_TIMESTAMP,
            date_added TEXT DEFAULT CURRENT_TIMESTAMP,
            bit_rate INTEGER,
            sample_rate INTEGER,
            release_date TEXT,
            normalization INTEGER,
            artwork_count INTEGER,
            sort_name TEXT,
            persistent_id TEXT,
            track_type TEXT,
            purchased INTEGER,
            has_video INTEGER,
            music_video INTEGER,
            location TEXT,
            file_folder_count INTEGER,
            library_folder_count INTEGER,
            plays INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE Artists (
            artist_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
            name TEXT UNIQUE,
            sort_artist TEXT
        );
        CREATE TABLE Composers (
            composer_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
            name TEXT UNIQUE
        );
        CREATE TABLE Genres (
            genre_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
            name TEXT UNIQUE
        );
        CREATE TABLE Albums (
            album_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
            artist_id INTEGER REFERENCES Artists(artist_id),
            name TEXT,
            sort_album TEXT,
            genre_id INTEGER REFERENCES Genres(genre_id),
            year INTEGER,
            release_date TEXT,
            date_modified TEXT DEFAULT CURRENT_TIMESTAMP,
            date_added TEXT DEFAULT CURRENT_TIMESTAMP,
            cover_id INTEGER
        );
        CREATE TABLE Covers (
            cover_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
            album_id INTEGER REFERENCES Albums(album_id),
            base64 TEXT
        );
        CREATE TABLE Playlists (
            playlist_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
            orig_playlist_id INTEGER,
            name TEXT,
            description TEXT,
            master INTEGER,
            playlist_persistent_id TEXT,
            parent_persistent_id TEXT,
            distinguished_kind INTEGER,
            visible INTEGER,
            all_items INTEGER,
            folder INTEGER,
            smart_info TEXT,
            smart_criteria TEXT,
            date_modified TEXT DEFAULT CURRENT_TIMESTAMP,
            date_added TEXT DEFAULT CURRENT_TIMESTAMP,
            tracks TEXT
        );
        CREATE INDEX idx_tracks_track_id ON Tracks(track_id);
        CREATE INDEX idx_tracks_artist_id ON Tracks(artist_id);
        CREATE INDEX idx_tracks_album_artist_id ON Tracks(album_artist_id);
        CREATE INDEX idx_tracks_composer_id ON Tracks(composer_id);
        CREATE INDEX idx_tracks_album_id ON Tracks(album_id);
        CREATE INDEX idx_tracks_genre_id ON Tracks(genre_id);
        CREATE INDEX idx_tracks_date_added ON Tracks(date_added);
        CREATE INDEX idx_tracks_release_date ON Tracks(release_date);
        CREATE INDEX idx_albums_album_id ON Albums(album_id);
        CREATE INDEX idx_albums_artist_id ON Albums(artist_id);
        CREATE INDEX idx_albums_name ON Albums(name);
        CREATE INDEX idx_albums_genre_id ON Albums(genre_id);
        CREATE INDEX idx_albums_year ON Albums(year);
        CREATE INDEX idx_albums_release_date ON Albums(release_date);
        CREATE INDEX idx_albums_date_added ON Albums(date_added);
        CREATE INDEX idx_artists_artist_id ON Artists(artist_id);
        CREATE INDEX idx_artists_name ON Artists(name);
        CREATE INDEX idx_composers_composer_id ON Composers(composer_id);
        CREATE INDEX idx_composers_name ON Composers(name);
        CREATE INDEX idx_genres_genre_id ON Genres(genre_id);
        CREATE INDEX idx_genres_name ON Genres(name);
        CREATE INDEX idx_playlists_playlist_id ON Playlists(playlist_id);
        CREATE INDEX idx_playlists_name ON Playlists(name);
    ";

    conn.execute(sql_statements).unwrap();

    insert_db_version(conn, None);
}

fn update_db_version(conn: &Connection, major: i64, minor: i64, patch: i64) {
    // In the future, provide code to update DB from version 1.0.0
    // First create a backup of the current db file.
    insert_db_version(
        conn,
        Some(format!(
            "Updated from version {}.{}.{}",
            major, minor, patch
        )),
    );
}

fn insert_db_version(conn: &Connection, opt_comment: Option<String>) {
    let mut stmt = conn
        .prepare(
            "INSERT INTO Version (
                major, minor, patch, comment
            ) VALUES (
                ?1, ?2, ?3, ?4
            )",
        )
        .unwrap();

    stmt.bind((1, DB_MAJOR)).unwrap();
    stmt.bind((2, DB_MINOR)).unwrap();
    stmt.bind((3, DB_PATCH)).unwrap();

    if let Some(comment) = opt_comment {
        stmt.bind((4, comment.as_str())).unwrap();
    } else {
        stmt.bind((4, "")).unwrap();
    }

    let _ = stmt.next();
}

fn year_from_str(input: &str) -> Option<i64> {
    let year_re = Regex::new(r"(\d{4})").unwrap();

    if let Some(caps) = year_re.captures(input) {
        if let Some(mat) = caps.get(1) {
            return mat.as_str().parse::<i64>().ok();
        }
    }

    None
}

fn move_tracks(
    conn: &Connection,
    track_ids: Vec<i64>,
    artist_names: Vec<String>,
    album_names: Vec<String>,
    media_dir: String,
) {
    debug!("{:?}, {:?}, {:?}", track_ids, artist_names, album_names);
    let mut location_map: HashMap<i64, String> = HashMap::new();

    for (index, track_id) in track_ids.iter().enumerate() {
        let query = r#"
            SELECT location
            FROM Tracks
            WHERE track_id = ?1
            "#
        .to_string();

        let mut stmt = conn.prepare(query).unwrap();
        stmt.bind((1, track_id.clone())).unwrap();

        if let Ok(State::Row) = stmt.next() {
            let location = stmt.read::<String, _>("location").unwrap();
            if artist_names.len() > index && album_names.len() > index {
                let new_location = ensure_and_copy_file(
                    &media_dir.clone(),
                    &artist_names[index],
                    &album_names[index],
                    &location,
                );

                location_map.insert(*track_id, new_location);

                // Delte old file
                match fs::remove_file(&location) {
                    Ok(()) => (),
                    Err(error) => error!("{}", error),
                };

                // Delete parent folders if they are empty
                delete_location_parent(location, media_dir.clone());
            }
        }
    }

    update_track_location_batch(conn, location_map);
}

fn delete_tracks_exhaustive(
    conn: &Connection,
    vec_id: &Vec<i64>,
    delete_files: bool,
    media_dir: String,
) {
    /*
    If delete_files is true, delete the track files.
    If a folder is empty afterwards, delete it as well.
    Delete track entries from tracks table.

    Delete dangling artists, composers, albums, genres.

    TODO: Go through all the playlists and remove the track_ids if they exist. Or let users remove deleted tracks from playlists?

    TODO:
    make sure to do it in batches if we get above sqlite limit.
    SQLite 3.32.0 can handle up to 32766 variables
     */

    if vec_id.len() != 0 {
        if delete_files {
            let mut query = r#"
            SELECT location
            FROM Tracks
            WHERE track_id IN (
            "#
            .to_string();
            for _ in vec_id {
                query.push_str("?,");
            }
            query.pop(); // Remove the trailing comma
            query.push(')');

            let mut stmt = conn.prepare(query).unwrap();
            for (index, track_id) in vec_id.iter().enumerate() {
                stmt.bind((index + 1, track_id.clone())).unwrap();
            }

            while let Ok(State::Row) = stmt.next() {
                let location = stmt.read::<String, _>("location").unwrap();
                fs::remove_file(location.clone()).unwrap();

                delete_location_parent(location, media_dir.clone());
            }
        }

        // Delete track entries
        let mut delete_query = r#"
        DELETE FROM Tracks
        WHERE track_id IN (
        "#
        .to_string();
        for _ in vec_id {
            delete_query.push_str("?,");
        }
        delete_query.pop(); // Remove the trailing comma
        delete_query.push(')');

        let mut delete_stmt = conn.prepare(delete_query).unwrap();
        for (index, track_id) in vec_id.iter().enumerate() {
            delete_stmt.bind((index + 1, track_id.clone())).unwrap();
        }

        let _ = delete_stmt.next();
    }

    delete_unused_entries(&conn);
}

fn delete_location_parent(location: String, media_dir: String) {
    let mut location_parent = Path::new(&location).parent().unwrap();
    let media_path = Path::new(&media_dir);
    let mut deleted = true;

    // Deletes parent folders of location only if they are empty, while preserving media path even if empty
    while deleted && !location_parent.ends_with(media_path) {
        deleted = match fs::remove_dir(location_parent) {
            Ok(()) => true,
            Err(_) => false,
        };

        location_parent = location_parent.parent().unwrap();
    }
}

fn delete_unused_entries(conn: &Connection) {
    // Delete unused artists
    let artists_query = r#"
        SELECT artist_id
        FROM Artists
        WHERE artist_id NOT IN (
            SELECT DISTINCT artist_id
            FROM Tracks
        )
        AND artist_id NOT IN (
            SELECT DISTINCT album_artist_id
            FROM Tracks
        );"#
    .to_string();

    let mut artists_stmt = conn.prepare(artists_query).unwrap();

    let mut artist_id_set: HashSet<i64> = HashSet::new();

    while let Ok(State::Row) = artists_stmt.next() {
        artist_id_set.insert(artists_stmt.read::<i64, _>("artist_id").unwrap());
    }

    if artist_id_set.len() != 0 {
        let mut delete_query = r#"
            DELETE FROM Artists
            WHERE artist_id IN (
            "#
        .to_string();
        for _ in artist_id_set.iter() {
            delete_query.push_str("?,");
        }
        delete_query.pop(); // Remove the trailing comma
        delete_query.push(')');

        let mut delete_stmt = conn.prepare(delete_query).unwrap();
        for (index, artist_id) in artist_id_set.iter().enumerate() {
            delete_stmt.bind((index + 1, artist_id.clone())).unwrap();
        }

        let _ = delete_stmt.next();
    }

    // Delete unused composers
    let delete_query = r#"
        DELETE FROM Composers
        WHERE composer_id NOT IN (
            SELECT DISTINCT composer_id
            FROM Tracks
        );"#
    .to_string();

    let mut delete_stmt = conn.prepare(delete_query).unwrap();
    let _ = delete_stmt.next();

    // Delete unused albums
    let delete_query = r#"
        DELETE FROM Albums
        WHERE album_id NOT IN (
            SELECT DISTINCT album_id
            FROM Tracks
        );"#
    .to_string();

    let mut delete_stmt = conn.prepare(delete_query).unwrap();
    let _ = delete_stmt.next();

    // Delete unused genres
    let delete_query = r#"
        DELETE FROM Genres
        WHERE genre_id NOT IN (
            SELECT DISTINCT genre_id
            FROM Tracks
        );"#
    .to_string();

    let mut delete_stmt = conn.prepare(delete_query).unwrap();
    let _ = delete_stmt.next();
}

fn delete_playlists(conn: &Connection, vec_playlist_id: &Vec<i64>) {
    let mut query = r#"
        DELETE FROM Playlists
        WHERE playlist_id IN (
        "#
    .to_string();
    for _ in vec_playlist_id {
        query.push_str("?,");
    }
    query.pop(); // Remove the trailing comma
    query.push(')');

    let mut stmt = conn.prepare(query).unwrap();
    for (index, playlist_id) in vec_playlist_id.iter().enumerate() {
        stmt.bind((index + 1, playlist_id.clone())).unwrap();
    }

    stmt.next().unwrap();
}

fn collect_files_in_directory(dir_path: &Path, paths_vec: &mut Vec<String>, max_level: u8) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    paths_vec.push(entry_path.to_string_lossy().into_owned());
                } else if entry_path.is_dir() {
                    if max_level != 0 {
                        collect_files_in_directory(&entry_path, paths_vec, max_level - 1);
                    }
                }
            }
        }
    }
}

fn increment_plays_for_track(conn: &Connection, id: i64) {
    let query = r#"
        UPDATE Tracks SET plays = plays + 1 WHERE track_id = ?
        "#
    .to_string();

    let mut stmt = conn.prepare(query).unwrap();
    stmt.bind((1, id)).unwrap();

    stmt.next().unwrap();
}

pub fn create_sqlite_instance(
    app: AppHandle,
    db_receiver: Receiver<DBRequest>,
    data_sender: Sender<DBData>,
    mut config_state: ConfigState,
) {
    thread::spawn(move || {
        match app.path().app_local_data_dir() {
            Ok(mut data_path) => {
                if tauri::is_dev() {
                    data_path.push("debug.sqlite"); // test db
                } else {
                    data_path.push("db.sqlite"); // release db
                }

                let conn = sqlite::open(data_path.as_path()).unwrap();

                is_db_or_init(&conn);

                let mut db_state = get_db_state(&conn);

                // Input handling loop
                loop {
                    let mut data = Data {
                        queue: None,
                        tracks: None,
                        albums: None,
                        artists: None,
                        artist_albums: None,
                        artist_tracks: None,
                        composers: None,
                        composer_tracks: None,
                        covers: None,
                        genres: None,
                        genre_tracks: None,
                        playlists: None,
                        spacetime: None,
                        search: None,
                        albums_order: None,
                        artists_order: None,
                        composers_order: None,
                        genres_order: None,
                        playlists_order: None,
                        tracks_order: None,
                        error: None,
                        loading: None,
                        is_init: None,
                    };

                    // Wait for user input
                    let received_request = db_receiver.recv();

                    match received_request {
                        Ok(result) => match result {
                            DBRequest::Play(datatype, vec_id, opt_vec_order) => match datatype {
                                DataType::Album => {
                                    println!("DBRequest::Play::Album {:?}", vec_id[0]);
                                    if let Some(audiotracks) =
                                        get_audiotracks_from_album_by_id(&conn, &vec_id)
                                    {
                                        let _ = data_sender.send(DBData::Play(audiotracks));
                                    }
                                }
                                DataType::Playlist => {
                                    println!("DBRequest::Play::Playlist {:?}", vec_id[0]);
                                    if let Some(audiotracks) =
                                        get_audiotracks_from_playlist_by_id(&conn, &vec_id)
                                    {
                                        let _ = data_sender.send(DBData::Play(audiotracks));
                                    }
                                }
                                DataType::Track => {
                                    println!("DBRequest::Play::Track {:?}", vec_id[0]);
                                    if let Some(audiotracks) = get_audiotracks_by_id(&conn, &vec_id)
                                    {
                                        let _ = data_sender.send(DBData::Play(audiotracks));
                                    }
                                }
                                DataType::Artist => {
                                    println!("DBRequest::Play::Artist {:?}", vec_id[0]);
                                    if let Some(audiotracks) =
                                        get_audiotracks_from_artist_by_id(&conn, &vec_id)
                                    {
                                        let _ = data_sender.send(DBData::Play(audiotracks));
                                    }
                                }
                                DataType::Composer => {
                                    println!("DBRequest::Play::Composer {:?}", vec_id[0]);
                                    if let Some(vec_order) = opt_vec_order {
                                        if let Some(audiotracks) =
                                            get_audiotracks_from_composer_by_id_and_order(
                                                &conn, &vec_id, vec_order,
                                            )
                                        {
                                            let _ = data_sender.send(DBData::Play(audiotracks));
                                        }
                                    } else {
                                        // default sort by name
                                        if let Some(audiotracks) =
                                            get_audiotracks_from_composer_by_id_and_order(
                                                &conn,
                                                &vec_id,
                                                vec![Order::ByName],
                                            )
                                        {
                                            let _ = data_sender.send(DBData::Play(audiotracks));
                                        }
                                    }
                                }
                                DataType::Genre => {
                                    println!("DBRequest::Play::Genre {:?}", vec_id[0]);
                                    if let Some(vec_order) = opt_vec_order {
                                        if let Some(audiotracks) =
                                            get_audiotracks_from_genre_by_id_and_order(
                                                &conn, &vec_id, vec_order,
                                            )
                                        {
                                            let _ = data_sender.send(DBData::Play(audiotracks));
                                        }
                                    } else {
                                        // default sort by name
                                        if let Some(audiotracks) =
                                            get_audiotracks_from_genre_by_id_and_order(
                                                &conn,
                                                &vec_id,
                                                vec![Order::ByName],
                                            )
                                        {
                                            let _ = data_sender.send(DBData::Play(audiotracks));
                                        }
                                    }
                                }
                                _ => {}
                            },
                            DBRequest::QueueInsert(datatype, vec_id, opt_index, opt_vec_order) => {
                                match datatype {
                                    DataType::Album => {
                                        println!(
                                            "DBRequest::QueueInsert::Album: {:?}, index: {:?}",
                                            vec_id[0], opt_index
                                        );

                                        if let Some(audiotracks) =
                                            get_audiotracks_from_album_by_id(&conn, &vec_id)
                                        {
                                            let _ = data_sender
                                                .send(DBData::QueueInsert(audiotracks, opt_index));
                                        }
                                    }
                                    DataType::Playlist => {
                                        println!(
                                            "DBRequest::QueueInsert::Playlist: {:?}, index: {:?}",
                                            vec_id[0], opt_index
                                        );

                                        if let Some(audiotracks) =
                                            get_audiotracks_from_playlist_by_id(&conn, &vec_id)
                                        {
                                            let _ = data_sender
                                                .send(DBData::QueueInsert(audiotracks, opt_index));
                                        }
                                    }
                                    DataType::Track => {
                                        println!(
                                            "DBRequest::QueueInsert::Track: {:?}, index: {:?}",
                                            vec_id[0], opt_index
                                        );

                                        if let Some(audiotracks) =
                                            get_audiotracks_by_id(&conn, &vec_id)
                                        {
                                            let _ = data_sender
                                                .send(DBData::QueueInsert(audiotracks, opt_index));
                                        }
                                    }
                                    DataType::Artist => {
                                        println!(
                                            "DBRequest::QueueInsert::Artist: {:?}, index: {:?}",
                                            vec_id[0], opt_index
                                        );
                                        if let Some(audiotracks) =
                                            get_audiotracks_from_artist_by_id(&conn, &vec_id)
                                        {
                                            let _ = data_sender
                                                .send(DBData::QueueInsert(audiotracks, opt_index));
                                        }
                                    }
                                    DataType::Composer => {
                                        println!(
                                            "DBRequest::QueueInsert::Composer: {:?}, index: {:?}",
                                            vec_id[0], opt_index
                                        );
                                        if let Some(vec_order) = opt_vec_order {
                                            if let Some(audiotracks) =
                                                get_audiotracks_from_composer_by_id_and_order(
                                                    &conn, &vec_id, vec_order,
                                                )
                                            {
                                                let _ = data_sender.send(DBData::QueueInsert(
                                                    audiotracks,
                                                    opt_index,
                                                ));
                                            }
                                        } else {
                                            // default sort by name
                                            if let Some(audiotracks) =
                                                get_audiotracks_from_composer_by_id_and_order(
                                                    &conn,
                                                    &vec_id,
                                                    vec![Order::ByName],
                                                )
                                            {
                                                let _ = data_sender.send(DBData::QueueInsert(
                                                    audiotracks,
                                                    opt_index,
                                                ));
                                            }
                                        }
                                    }
                                    DataType::Genre => {
                                        println!(
                                            "DBRequest::QueueInsert::Genre: {:?}, index: {:?}",
                                            vec_id[0], opt_index
                                        );
                                        if let Some(vec_order) = opt_vec_order {
                                            if let Some(audiotracks) =
                                                get_audiotracks_from_genre_by_id_and_order(
                                                    &conn, &vec_id, vec_order,
                                                )
                                            {
                                                let _ = data_sender.send(DBData::QueueInsert(
                                                    audiotracks,
                                                    opt_index,
                                                ));
                                            }
                                        } else {
                                            // default sort by name
                                            if let Some(audiotracks) =
                                                get_audiotracks_from_genre_by_id_and_order(
                                                    &conn,
                                                    &vec_id,
                                                    vec![Order::ByName],
                                                )
                                            {
                                                let _ = data_sender.send(DBData::QueueInsert(
                                                    audiotracks,
                                                    opt_index,
                                                ));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            DBRequest::AudioBackendRecover(history_ids, queue_ids) => {
                                let _ = data_sender.send(DBData::AudioBackendRecover(
                                    get_audiotracks_by_id(&conn, &history_ids),
                                    get_audiotracks_by_id(&conn, &queue_ids),
                                ));
                            }
                            DBRequest::NewPlaylist(playlist) => {
                                let db_playlist = DBPlaylist {
                                    name: playlist.name,
                                    tracks: playlist.tracks,
                                    description: playlist.description,
                                    ..Default::default()
                                };
                                insert_playlist(&conn, db_playlist, false);

                                // TODO: verify playlist update, then send updated playlist, not all playlists...
                                data.playlists = Some(get_playlists(&conn));
                                data.playlists_order = Some((
                                    vec![Order::ByName],
                                    get_playlists_order(&conn, vec![Order::ByName]),
                                ));
                                db_state = get_db_state(&conn);
                            }
                            DBRequest::AddToLibrary(paths) => {
                                let mut paths_to_check: Vec<String> = paths;
                                let mut import_paths: Vec<String> = vec![];
                                let mut artists: HashMap<String, i64> = HashMap::new();
                                let mut albums: HashMap<String, i64> = HashMap::new();
                                let mut composers: HashMap<String, i64> = HashMap::new();
                                let mut genres: HashMap<String, i64> = HashMap::new();

                                while let Some(path_str) = paths_to_check.pop() {
                                    let path = Path::new(&path_str);

                                    if path.exists() {
                                        if path.is_file() {
                                            import_paths.push(path_str);
                                        } else if path.is_dir() {
                                            // Add all files in the directory to the paths vector, max search level 3
                                            collect_files_in_directory(
                                                path,
                                                &mut paths_to_check,
                                                3,
                                            );
                                        }
                                    } else {
                                        error!("{} does not exist", path_str);
                                    }
                                }

                                let mut count = 0;
                                // Process the import_paths vector
                                for path_str in import_paths {
                                    let path = Path::new(&path_str);
                                    /*
                                    For importing we need to get metadata from the files.
                                    Which files can we actually get metadata from?
                                    symphonia probe seems to handle this by itself.
                                    Listed formats (from https://github.com/pdeljanov/Symphonia):
                                    mp4 mp3 id3v2 aac id3v1 wav flac ogg vorbis pcm mkv alac apple-lossless m4a mp2 adpcm
                                     */

                                    /*
                                    TODO:
                                    if the file is a jpg or png, we want to consider using it as album cover. Maybe?
                                    Problem is that we cannot easily associate a file with audio files in the same folder.
                                    Need to restructure import to be able to achieve this.
                                     */
                                    if let Some(meta) = extract_metadata(path) {
                                        let mut track = DBTrack {
                                            orig_track_id: 0,
                                            name: meta.name.unwrap_or_default(),
                                            artist_id: 0,
                                            album_artist_id: 0,
                                            composer_id: 0,
                                            album_id: 0,
                                            genre_id: 0,
                                            kind: meta.kind.unwrap_or_default(),
                                            size: meta.size.unwrap_or_default(),
                                            total_time: meta.total_time.unwrap_or_default(),
                                            disc_number: meta.disc_number.unwrap_or_default(),
                                            disc_count: meta.disc_count.unwrap_or_default(),
                                            track_number: meta.track_number.unwrap_or_default(),
                                            track_count: meta.track_count.unwrap_or_default(),
                                            year: meta.year.unwrap_or_default(),
                                            date_modified: "".to_string(),
                                            date_added: "".to_string(),
                                            bit_rate: meta.bit_rate.unwrap_or_default(),
                                            sample_rate: meta.sample_rate.unwrap_or_default(),
                                            release_date: meta.release_date.unwrap_or_default(),
                                            normalization: 0,
                                            artwork_count: 0,
                                            sort_name: "".to_string(),
                                            persistent_id: "".to_string(),
                                            track_type: "".to_string(),
                                            purchased: 0,
                                            has_video: 0,
                                            hd: 0,
                                            video_width: 0,
                                            video_height: 0,
                                            music_video: 0,
                                            location: "".to_string(),
                                            file_folder_count: 0,
                                            library_folder_count: 0,
                                        };
                                        /*
                                        TODO:
                                        Go through the meta entries and populate an insert query with it.
                                        Check if an album, artist, album_artist, composer, genre exists or create otherwise. Insert id.
                                        Implement support for importing music videos.
                                         */
                                        //println!("{:?}", meta);

                                        track.artist_id = if let Some(ref artist) = meta.artist {
                                            if let Some(id) = artists.get(artist) {
                                                id.to_owned()
                                            } else {
                                                let id = get_or_create_artist_id(&conn, artist, "");
                                                artists.insert(artist.to_string(), id);
                                                id
                                            }
                                        } else {
                                            // See if we have an album_artist to use instead
                                            0
                                        };

                                        track.album_artist_id =
                                            if let Some(ref album_artist) = meta.album_artist {
                                                if let Some(id) = artists.get(album_artist) {
                                                    id.to_owned()
                                                } else {
                                                    let id = get_or_create_artist_id(
                                                        &conn,
                                                        album_artist,
                                                        "",
                                                    );
                                                    artists.insert(album_artist.to_string(), id);
                                                    id
                                                }
                                            } else {
                                                if track.artist_id == 0 {
                                                    track.artist_id =
                                                        get_or_create_artist_id(&conn, "", "");
                                                    artists.insert("".to_string(), track.artist_id);
                                                }
                                                // Instead use track.artist_id as track.album_artist_id
                                                track.artist_id
                                            };

                                        if track.artist_id == 0 {
                                            track.artist_id = track.album_artist_id;
                                        }

                                        track.composer_id = if let Some(ref composer) =
                                            meta.composer
                                        {
                                            if let Some(id) = composers.get(composer) {
                                                id.to_owned()
                                            } else {
                                                let id = get_or_create_composer_id(&conn, composer);
                                                composers.insert(composer.to_string(), id);
                                                id
                                            }
                                        } else {
                                            let id = get_or_create_composer_id(&conn, "");
                                            composers.insert("".to_string(), id);
                                            id
                                        };

                                        track.genre_id = if let Some(ref genre) = meta.genre {
                                            if let Some(id) = genres.get(genre) {
                                                println!("found id 1: {}", id);
                                                id.to_owned()
                                            } else {
                                                let id = get_or_create_genre_id(&conn, genre);
                                                genres.insert(genre.to_string(), id);
                                                println!("found id 2: {}", id);
                                                id
                                            }
                                        } else {
                                            let id = get_or_create_genre_id(&conn, "");
                                            genres.insert("".to_string(), id);
                                            println!("found id 3: {}", id);
                                            id
                                        };

                                        // TODO: seemingly not working sometimes?
                                        if track.year == 0 && track.release_date != "" {
                                            if let Some(year) = year_from_str(&track.release_date) {
                                                track.year = year;
                                            }
                                        }

                                        track.album_id = if let Some(ref album) = meta.album {
                                            // Album cannot be identified just by name, requires album_artist_id as well
                                            let mut album_key = album.clone();
                                            album_key.push_str(&track.album_artist_id.to_string());

                                            if let Some(id) = albums.get(&album_key) {
                                                id.to_owned()
                                            } else {
                                                let id = get_or_create_album_id(
                                                    &conn,
                                                    album,
                                                    "",
                                                    track.album_artist_id,
                                                    track.genre_id,
                                                    track.year,
                                                    &track.release_date,
                                                    None,
                                                );
                                                albums.insert(album_key, id);
                                                id
                                            }
                                        } else {
                                            let id = get_or_create_album_id(
                                                &conn,
                                                "",
                                                "",
                                                track.album_artist_id,
                                                track.genre_id,
                                                track.year,
                                                &track.release_date,
                                                None,
                                            );
                                            albums
                                                .insert(format!(" {}", track.album_artist_id), id);
                                            id
                                        };

                                        let cover_id = if let Some(ref cover) = meta.cover {
                                            let id = get_or_create_cover_id(
                                                &conn,
                                                track.album_id,
                                                cover,
                                            );
                                            id
                                        } else {
                                            0
                                        };

                                        println!(
                                            "Current state: {} {} {} {} {} {}",
                                            track.artist_id,
                                            track.album_artist_id,
                                            track.composer_id,
                                            track.genre_id,
                                            track.album_id,
                                            cover_id
                                        );

                                        track.location = if config_state.manage_folders {
                                            ensure_and_copy_file(
                                                &config_state.media_path.clone(),
                                                meta.album_artist
                                                    .clone()
                                                    .unwrap_or_else(|| "Unknown".to_owned())
                                                    .as_str(),
                                                meta.album
                                                    .clone()
                                                    .unwrap_or_else(|| "Unknown".to_owned())
                                                    .as_str(),
                                                path_str.as_str(),
                                            )
                                        } else {
                                            path_str.to_string()
                                        };

                                        let metadata =
                                            fs::metadata(track.location.clone()).unwrap();

                                        track.size = metadata.len() as i64;

                                        // TODO: Instead collect tracks and batch insert
                                        println!("AddToLibrary, implement track batch import!!!");
                                        insert_track(&conn, track, false);

                                        count += 1;
                                        let _ = app.emit(
                                            "backend_message",
                                            BackendMessage {
                                                notification: None,
                                                error: None,
                                                warning: None,
                                                progress: Some(Progress {
                                                    info: ProgressInfo::FileImport,
                                                    value: Some(count),
                                                    done: false,
                                                }),
                                            },
                                        );
                                    }
                                }

                                let _ = app.emit("db_state", get_db_state(&conn));
                                data = get_init_data(&conn);
                                db_state = get_db_state(&conn);
                                let _ = app.emit(
                                    "backend_message",
                                    BackendMessage {
                                        notification: None,
                                        error: None,
                                        warning: None,
                                        progress: Some(Progress {
                                            info: ProgressInfo::FileImport,
                                            value: None,
                                            done: true,
                                        }),
                                    },
                                );
                            }
                            DBRequest::ImportLibrary(path_string) => {
                                let path = Path::new(&path_string);

                                if path.exists() {
                                    if path.is_file() && path.extension().unwrap() == "xml" {
                                        let prev_db_state = get_db_state(&conn);

                                        let _ = app.emit(
                                            "backend_message",
                                            BackendMessage {
                                                notification: None,
                                                error: None,
                                                warning: None,
                                                progress: Some(Progress {
                                                    info: ProgressInfo::LibraryImport,
                                                    value: None,
                                                    done: false,
                                                }),
                                            },
                                        );
                                        extract_tracks_and_playlists(path_string, &conn);

                                        /*
                                        TODO:
                                        update media_path based on paths in db after extraction
                                        only if the db was empty before

                                        Otherwise:
                                        Implement some way to import lib in existing db and update paths via "find missing tracks"?
                                         */

                                        // If db was empty, automatically update media path to be same as in library.xml
                                        /*
                                        TODO:
                                        There actually is a "Music Folder" key in the file. But for now we are not using that.
                                        This key also differs from our definition of media_path in that it requires to use the "Music" subfolder.
                                        This "Music" folder actually containers the artist and album folders.
                                        */
                                        if prev_db_state.tracks_max.unwrap() == 0 {
                                            // Get all locations
                                            let query = r#"
                                                SELECT location
                                                FROM Tracks
                                                "#
                                            .to_string();

                                            let mut location_vec: Vec<String> = Vec::new();

                                            let mut stmt = conn.prepare(&query).unwrap();

                                            while let Ok(State::Row) = stmt.next() {
                                                location_vec.push(
                                                    stmt.read::<String, _>("location").unwrap(),
                                                );
                                            }

                                            match longest_common_path(location_vec) {
                                                Some(common_path) => {
                                                    let mut new_config_state = config_state.clone();
                                                    new_config_state.media_path = common_path;

                                                    // TODO: this is ugly and might conflict with simultanious frontend config_request..
                                                    match load_or_setup_config_path(app.clone()) {
                                                        Ok(config_path) => {
                                                            match set_config(
                                                                config_path,
                                                                new_config_state,
                                                            ) {
                                                                Ok(new_config) => {
                                                                    config_state = new_config;
                                                                    match app.emit(
                                                                        "config_state",
                                                                        config_state.clone(),
                                                                    ) {
                                                                        Ok(()) => {}
                                                                        Err(error) => {
                                                                            error!("{}", error);
                                                                            // TODO: send error to frontend?
                                                                        }
                                                                    };
                                                                }
                                                                Err(error) => {
                                                                    error!("{}", error);
                                                                    // TODO: emit error event
                                                                    //app.emit(event, payload);
                                                                }
                                                            }
                                                        }
                                                        Err(error) => {
                                                            error!("{}", error);
                                                            // TODO: emit error event
                                                            //app.emit(event, payload);
                                                        }
                                                    }
                                                }
                                                None => {}
                                            }
                                        }

                                        let _ = app.emit("db_state", get_db_state(&conn));
                                        data = get_init_data(&conn);

                                        db_state = get_db_state(&conn);
                                        let _ = app.emit(
                                            "backend_message",
                                            BackendMessage {
                                                notification: Some(Notification::LibraryImport),
                                                error: None,
                                                warning: None,
                                                progress: Some(Progress {
                                                    info: ProgressInfo::LibraryImport,
                                                    value: None,
                                                    done: true,
                                                }),
                                            },
                                        );
                                    } else {
                                        error!("{} is not an xml file", path.to_string_lossy());
                                    }
                                } else {
                                    error!("{} does not exist", path.to_string_lossy());
                                }
                            }
                            DBRequest::UpdateTrackLocations(old_path, new_path) => {
                                // TODO: used by find missing tracks, updates location only for those tracks whose location starts with old_path
                                if let Some(tracks_max) = db_state.tracks_max {
                                    if tracks_max != 0 {
                                        update_media_path(&conn, old_path, new_path);
                                    }
                                }
                                // TODO: update find missing tracks, if some tracks are found
                            }
                            DBRequest::DeleteById(datatype, vec_id, delete_files) => {
                                let _ = app.emit(
                                    "backend_message",
                                    BackendMessage {
                                        notification: None,
                                        error: None,
                                        warning: None,
                                        progress: Some(Progress {
                                            info: ProgressInfo::Delete,
                                            value: None,
                                            done: false,
                                        }),
                                    },
                                );

                                if datatype == DataType::Playlist {
                                    // Delete the playlists but no tracks
                                    delete_playlists(&conn, &vec_id);

                                    data.playlists = Some(get_playlists(&conn));
                                    data.playlists_order = Some((
                                        vec![Order::ByName],
                                        get_playlists_order(&conn, vec![Order::ByName]),
                                    ));
                                } else if config_state.allow_delete_from_db {
                                    // Gather track_ids from db if delete allowed
                                    let vec_track_id: Vec<i64> = match datatype {
                                        DataType::Album => {
                                            if let Some(audiotracks) =
                                                get_audiotracks_from_album_by_id(&conn, &vec_id)
                                            {
                                                let mut track_ids = Vec::new();

                                                for audiotrack in audiotracks {
                                                    track_ids.push(audiotrack.id);
                                                }

                                                track_ids
                                            } else {
                                                Vec::new()
                                            }
                                        }
                                        DataType::Artist => {
                                            if let Some(audiotracks) =
                                                get_audiotracks_from_artist_by_id(&conn, &vec_id)
                                            {
                                                let mut track_ids = Vec::new();

                                                for audiotrack in audiotracks {
                                                    track_ids.push(audiotrack.id);
                                                }

                                                track_ids
                                            } else {
                                                Vec::new()
                                            }
                                        }
                                        DataType::Composer => {
                                            if let Some(audiotracks) =
                                                get_audiotracks_from_composer_by_id_and_order(
                                                    &conn,
                                                    &vec_id,
                                                    vec![Order::ByName],
                                                )
                                            {
                                                let mut track_ids = Vec::new();

                                                for audiotrack in audiotracks {
                                                    track_ids.push(audiotrack.id);
                                                }

                                                track_ids
                                            } else {
                                                Vec::new()
                                            }
                                        }
                                        DataType::Genre => {
                                            if let Some(audiotracks) =
                                                get_audiotracks_from_genre_by_id_and_order(
                                                    &conn,
                                                    &vec_id,
                                                    vec![Order::ByName],
                                                )
                                            {
                                                let mut track_ids = Vec::new();

                                                for audiotrack in audiotracks {
                                                    track_ids.push(audiotrack.id);
                                                }

                                                track_ids
                                            } else {
                                                Vec::new()
                                            }
                                        }
                                        DataType::Track => {
                                            // already contains track_ids
                                            vec_id
                                        }
                                        _ => Vec::new(), // Covers and playlists are irrelevant for track delete, Video datatype not implemented atm
                                    };

                                    // Delete tracks from db, and if allowed delete files
                                    delete_tracks_exhaustive(
                                        &conn,
                                        &vec_track_id,
                                        config_state.allow_delete_files && delete_files,
                                        config_state.media_path.clone(),
                                    );

                                    // Too many db entries might change, easier to resend all data
                                    data = get_init_data(&conn);
                                    db_state = get_db_state(&conn);
                                    let _ = app.emit("db_state", db_state.clone());
                                }

                                let _ = app.emit(
                                    "backend_message",
                                    BackendMessage {
                                        notification: None,
                                        error: None,
                                        warning: None,
                                        progress: Some(Progress {
                                            info: ProgressInfo::Delete,
                                            value: None,
                                            done: true,
                                        }),
                                    },
                                );
                            }
                            DBRequest::GetDataOrder(datatype, opt_order) => {
                                match datatype {
                                    DataType::Artist => {
                                        let mut vec_order = vec![Order::ByName];
                                        if let Some(vec_ordering) = opt_order {
                                            vec_order = vec_ordering;
                                        }

                                        let new_order = get_artists_order(&conn, vec_order.clone());

                                        data.artists_order = Some((vec_order, new_order));
                                    }
                                    DataType::Composer => {
                                        let mut vec_order = vec![Order::ByName];
                                        if let Some(vec_ordering) = opt_order {
                                            vec_order = vec_ordering;
                                        }

                                        let new_order =
                                            get_composers_order(&conn, vec_order.clone());

                                        data.composers_order = Some((vec_order, new_order));
                                    }
                                    DataType::Album => {
                                        let mut vec_order = vec![Order::ByName];
                                        if let Some(vec_ordering) = opt_order {
                                            vec_order = vec_ordering;
                                        }

                                        let new_order = get_albums_order(&conn, vec_order.clone());

                                        data.albums_order = Some((vec_order, new_order));
                                    }
                                    DataType::Cover => {
                                        /*
                                        There is no reason to retrieve covers by order
                                         */
                                    }
                                    DataType::Genre => {
                                        let mut vec_order = vec![Order::ByName];
                                        if let Some(vec_ordering) = opt_order {
                                            vec_order = vec_ordering;
                                        }

                                        let new_order = get_genres_order(&conn, vec_order.clone());

                                        data.genres_order = Some((vec_order, new_order));
                                    }
                                    DataType::Playlist => {
                                        let mut vec_order = vec![Order::ByName];
                                        if let Some(vec_ordering) = opt_order {
                                            vec_order = vec_ordering;
                                        }

                                        let new_order =
                                            get_playlists_order(&conn, vec_order.clone());

                                        data.playlists_order = Some((vec_order, new_order));
                                    }
                                    DataType::Track => {
                                        let mut vec_order = vec![Order::ByName];
                                        if let Some(vec_ordering) = opt_order {
                                            vec_order = vec_ordering;
                                        }

                                        let new_order = get_tracks_order(&conn, vec_order.clone());

                                        data.tracks_order = Some((vec_order, new_order));
                                    }
                                    DataType::Video => {
                                        /*
                                        TODO:
                                        ByName,
                                        ByReleaseDate,
                                        ByAddedDate,
                                        ByModifiedDate,
                                        ByArtist,
                                        ByAlbumArtist,
                                        ByComposer,
                                        ByAlbum,
                                        ByGenre,
                                        BySize,
                                        ByTime,
                                        ByNameInverse,
                                        ByReleaseDateInverse,
                                        ByAddedDateInverse,
                                        ByModifiedDateInverse,
                                        ByArtistInverse,
                                        ByAlbumArtistInverse,
                                        ByComposerInverse,
                                        ByAlbumInverse,
                                        ByGenreInverse,
                                        BySizeInverse,
                                        ByTimeInverse,
                                         */
                                    }
                                }
                            }
                            DBRequest::GetCoversById(vec_id) => {
                                data.covers = get_covers_by_id(&conn, &vec_id);
                            }
                            DBRequest::GetTrackPaths(_vec_id) => {
                                // TODO: return the track paths for a given track_ids
                            }
                            DBRequest::Search(search, opt_datatypes, opt_limit) => {
                                let mut search_result = Search {
                                    tracks: None,
                                    albums: None,
                                    genres: None,
                                    artists: None,
                                    composers: None,
                                    playlists: None,
                                };

                                let mut limit = 100;
                                if let Some(ui_limit) = opt_limit {
                                    limit = ui_limit;
                                }

                                if let Some(datatypes) = opt_datatypes {
                                    for datatype in datatypes {
                                        match datatype {
                                            DataType::Album => {
                                                search_result.albums =
                                                    search_albums(&conn, search.clone(), limit);
                                            }
                                            DataType::Artist => {
                                                search_result.artists =
                                                    search_artists(&conn, search.clone(), limit);
                                            }
                                            DataType::Composer => {
                                                search_result.composers =
                                                    search_composers(&conn, search.clone(), limit);
                                            }
                                            DataType::Cover => {
                                                // irrelevant for search
                                            }
                                            DataType::Genre => {
                                                search_result.genres =
                                                    search_genres(&conn, search.clone(), limit);
                                            }
                                            DataType::Track => {
                                                search_result.tracks =
                                                    search_tracks(&conn, search.clone(), limit);
                                            }
                                            DataType::Video => {
                                                // Included in tracks
                                            }
                                            DataType::Playlist => {
                                                search_result.playlists =
                                                    search_playlists(&conn, search.clone(), limit);
                                            }
                                        }
                                    }
                                } else {
                                    search_result.albums =
                                        search_albums(&conn, search.clone(), limit);
                                    search_result.artists =
                                        search_artists(&conn, search.clone(), limit);
                                    search_result.composers =
                                        search_composers(&conn, search.clone(), limit);
                                    search_result.genres =
                                        search_genres(&conn, search.clone(), limit);
                                    search_result.tracks =
                                        search_tracks(&conn, search.clone(), limit);
                                    search_result.playlists =
                                        search_playlists(&conn, search.clone(), limit);
                                }

                                data.search = Some(search_result);
                            }
                            DBRequest::UpdateTracks(
                                tracks,
                                artist_names,
                                album_names,
                                genre_names,
                            ) => {
                                let _ = app.emit(
                                    "backend_message",
                                    BackendMessage {
                                        notification: None,
                                        error: None,
                                        warning: None,
                                        progress: Some(Progress {
                                            info: ProgressInfo::UpdateTracks,
                                            value: Some(0),
                                            done: false,
                                        }),
                                    },
                                );

                                let mut count = 0;
                                for (index, track) in tracks.iter().enumerate() {
                                    // Get track from db, then compare the data and update what is required
                                    if let Some(mut db_track) = get_track_by_id(&conn, track.id) {
                                        db_track.name = track.name.clone();
                                        db_track.disc_number = track.disc_number;
                                        db_track.track_number = track.track_number;

                                        let mut move_track = false;

                                        // Don't need to move track if artist changes, only relevant for album_artist
                                        if artist_names.len() > index {
                                            let artist_name = &artist_names[index];
                                            if let Some(db_artist) =
                                                get_artist_by_id(&conn, db_track.artist_id)
                                            {
                                                if db_artist.name != *artist_name {
                                                    db_track.artist_id = get_or_create_artist_id(
                                                        &conn,
                                                        &artist_name,
                                                        "",
                                                    );
                                                }
                                            } else {
                                                db_track.artist_id = get_or_create_artist_id(
                                                    &conn,
                                                    &artist_name,
                                                    "",
                                                );
                                            }
                                        }

                                        if genre_names.len() > index {
                                            let genre_name = &genre_names[index];
                                            println!("genre_name {genre_name}");
                                            if let Some(db_genre) =
                                                get_genre_by_id(&conn, db_track.genre_id)
                                            {
                                                println!("got genre");
                                                if db_genre.name != *genre_name {
                                                    db_track.genre_id =
                                                        get_or_create_genre_id(&conn, &genre_name);
                                                    println!(
                                                        "applied new id: {}",
                                                        db_track.genre_id
                                                    );
                                                }
                                            } else {
                                                db_track.genre_id =
                                                    get_or_create_genre_id(&conn, &genre_name);
                                                println!("2applied new id: {}", db_track.genre_id);
                                            }
                                        }
                                        debug!("new genre_id {}", db_track.genre_id);

                                        let mut active_album_name = "".to_string();
                                        if album_names.len() > index {
                                            let album_name = &album_names[index];
                                            if let Some(db_album) =
                                                get_album_by_id(&conn, db_track.album_id)
                                            {
                                                active_album_name = db_album.name.to_string();
                                                if db_album.name != *album_name {
                                                    active_album_name = album_name.to_string();
                                                    move_track = true;
                                                    db_track.album_id = get_or_create_album_id(
                                                        &conn,
                                                        &album_name,
                                                        "",
                                                        db_track.artist_id,
                                                        db_track.genre_id,
                                                        db_album.year,
                                                        &db_album.release_date,
                                                        Some((
                                                            &db_album.date_modified,
                                                            &db_album.date_added,
                                                        )),
                                                    );
                                                }
                                            } else {
                                                active_album_name = album_name.to_string();
                                                move_track = true;
                                                db_track.album_id = get_or_create_album_id(
                                                    &conn,
                                                    &album_name,
                                                    "",
                                                    db_track.artist_id,
                                                    db_track.genre_id,
                                                    0,
                                                    "",
                                                    None,
                                                );
                                            }
                                        }

                                        let query = r#"
                                            UPDATE Tracks
                                            SET name = ?1, artist_id = ?2, album_id = ?3, genre_id = ?4, disc_number = ?5, track_number = ?6, date_modified = CURRENT_TIMESTAMP
                                            WHERE track_id = ?7
                                            "#
                                        .to_string();

                                        let mut stmt = conn.prepare(query).unwrap();
                                        stmt.bind((1, db_track.name.as_str())).unwrap();
                                        stmt.bind((2, db_track.artist_id)).unwrap();
                                        stmt.bind((3, db_track.album_id)).unwrap();
                                        stmt.bind((4, db_track.genre_id)).unwrap();
                                        stmt.bind((5, db_track.disc_number as i64)).unwrap();
                                        stmt.bind((6, db_track.track_number as i64)).unwrap();
                                        stmt.bind((7, db_track.id)).unwrap();

                                        stmt.next().unwrap();

                                        if move_track && config_state.manage_folders {
                                            if let Some(album_artist) =
                                                get_artist_by_id(&conn, db_track.album_artist_id)
                                            {
                                                // Move the track to the new correct location of artist and album
                                                move_tracks(
                                                    &conn,
                                                    vec![db_track.id],
                                                    vec![album_artist.name],
                                                    vec![active_album_name],
                                                    config_state.media_path.clone(),
                                                );
                                            }
                                        }

                                        count += 1;
                                        let _ = app.emit(
                                            "backend_message",
                                            BackendMessage {
                                                notification: None,
                                                error: None,
                                                warning: None,
                                                progress: Some(Progress {
                                                    info: ProgressInfo::UpdateTracks,
                                                    value: Some(count),
                                                    done: false,
                                                }),
                                            },
                                        );
                                    }
                                }

                                delete_unused_entries(&conn);

                                let _ = app.emit("db_state", get_db_state(&conn));
                                db_state = get_db_state(&conn);
                                // Too much can change, easier to resend whole init data.
                                data = get_init_data(&conn);
                                let _ = app.emit(
                                    "backend_message",
                                    BackendMessage {
                                        notification: None,
                                        error: None,
                                        warning: None,
                                        progress: Some(Progress {
                                            info: ProgressInfo::UpdateTracks,
                                            value: None,
                                            done: true,
                                        }),
                                    },
                                );
                            }
                            DBRequest::UpdateAlbum(album, artist_name, genre_name) => {
                                if let Some(mut db_album) = get_album_by_id(&conn, album.id) {
                                    let _ = app.emit(
                                        "backend_message",
                                        BackendMessage {
                                            notification: None,
                                            error: None,
                                            warning: None,
                                            progress: Some(Progress {
                                                info: ProgressInfo::UpdateAlbum,
                                                value: None,
                                                done: false,
                                            }),
                                        },
                                    );
                                    db_album.sort_album = album.sort_album;
                                    db_album.year = album.year;
                                    db_album.release_date = album.release_date;

                                    let mut move_track_multi = false;
                                    let mut new_artist_genre = false;

                                    if db_album.name != album.name {
                                        move_track_multi = true;
                                        db_album.name = album.name;
                                    }

                                    let mut active_artist_name;
                                    if let Some(db_artist) =
                                        get_artist_by_id(&conn, db_album.artist_id)
                                    {
                                        active_artist_name = db_artist.name.to_string();
                                        if db_artist.name != *artist_name {
                                            active_artist_name = artist_name.to_string();
                                            move_track_multi = true;
                                            new_artist_genre = true;
                                            db_album.artist_id =
                                                get_or_create_artist_id(&conn, &artist_name, "");
                                        }
                                    } else {
                                        active_artist_name = artist_name.to_string();
                                        move_track_multi = true;
                                        new_artist_genre = true;
                                        db_album.artist_id =
                                            get_or_create_artist_id(&conn, &artist_name, "");
                                    }

                                    let old_genre_id = db_album.genre_id;
                                    if let Some(db_genre) =
                                        get_genre_by_id(&conn, db_album.genre_id)
                                    {
                                        if db_genre.name != *genre_name {
                                            new_artist_genre = true;
                                            db_album.genre_id =
                                                get_or_create_genre_id(&conn, &genre_name);
                                        }
                                    } else {
                                        new_artist_genre = true;
                                        db_album.genre_id =
                                            get_or_create_genre_id(&conn, &genre_name);
                                    }

                                    let query = r#"
                                        UPDATE Albums
                                        SET name = ?1, artist_id = ?2, sort_album = ?3, genre_id = ?4, year = ?5, release_date = ?6, date_modified = CURRENT_TIMESTAMP
                                        WHERE album_id = ?7
                                        "#
                                    .to_string();

                                    let mut stmt = conn.prepare(query).unwrap();
                                    stmt.bind((1, db_album.name.as_str())).unwrap();
                                    stmt.bind((2, db_album.artist_id)).unwrap();
                                    stmt.bind((3, db_album.sort_album.as_str())).unwrap();
                                    stmt.bind((4, db_album.genre_id)).unwrap();
                                    stmt.bind((5, db_album.year)).unwrap();
                                    stmt.bind((6, db_album.release_date.as_str())).unwrap();
                                    stmt.bind((7, db_album.id)).unwrap();

                                    debug!("db_album {:?}", db_album);

                                    stmt.next().unwrap();

                                    if new_artist_genre {
                                        // Update tracks with new artist_id or genre_id
                                        let query = r#"
                                            UPDATE Tracks
                                            SET album_artist_id = ?1, date_modified = CURRENT_TIMESTAMP, genre_id = CASE
                                                    WHEN genre_id = ?2 THEN ?3
                                                    ELSE genre_id
                                                END
                                            WHERE album_id = ?4
                                            "#
                                        .to_string();

                                        let mut stmt = conn.prepare(query).unwrap();
                                        stmt.bind((1, db_album.artist_id)).unwrap();
                                        stmt.bind((2, old_genre_id)).unwrap();
                                        stmt.bind((3, db_album.genre_id)).unwrap();
                                        stmt.bind((4, db_album.id)).unwrap();

                                        stmt.next().unwrap();
                                    }

                                    if move_track_multi && config_state.manage_folders {
                                        // Move the tracks to the new correct location of artist and album
                                        let num_tracks = db_album.tracks.len();
                                        move_tracks(
                                            &conn,
                                            db_album.tracks,
                                            vec![active_artist_name; num_tracks],
                                            vec![db_album.name; num_tracks],
                                            config_state.media_path.clone(),
                                        );
                                    }

                                    delete_unused_entries(&conn);
                                    let _ = app.emit(
                                        "backend_message",
                                        BackendMessage {
                                            notification: None,
                                            error: None,
                                            warning: None,
                                            progress: Some(Progress {
                                                info: ProgressInfo::UpdateAlbum,
                                                value: None,
                                                done: true,
                                            }),
                                        },
                                    );
                                }

                                let _ = app.emit("db_state", get_db_state(&conn));
                                db_state = get_db_state(&conn);
                                // Too much can change, easier to resend whole init data.
                                data = get_init_data(&conn);
                            }
                            DBRequest::UpdateArtist(artist) => {
                                // Combining artists with the same name is not handled automatically.
                                // The user will in the future have to use the "Standardise Names" tool, once it is implemented.

                                let query = r#"
                                    UPDATE Artists SET name = ?1, sort_artist = ?2 WHERE artist_id = ?3
                                    "#
                                .to_string();

                                let mut stmt = conn.prepare(query).unwrap();
                                stmt.bind((1, artist.name.as_str())).unwrap();
                                stmt.bind((2, artist.sort_artist.as_str())).unwrap();
                                stmt.bind((3, artist.id)).unwrap();

                                stmt.next().unwrap();

                                let _ = app.emit("db_state", get_db_state(&conn));
                                db_state = get_db_state(&conn);
                                data.artists = Some(vec![artist]);
                                data.artists_order = Some((
                                    vec![Order::ByName],
                                    get_artists_order(&conn, vec![Order::ByName]),
                                ));
                            }
                            DBRequest::UpdateComposer(composer) => {
                                // Combining composers with the same name is not handled automatically.
                                // The user will in the future have to use the "Standardise Names" tool, once it is implemented.

                                let query = r#"
                                    UPDATE Composers SET name = ?1 WHERE composer_id = ?2
                                    "#
                                .to_string();

                                let mut stmt = conn.prepare(query).unwrap();
                                stmt.bind((1, composer.name.as_str())).unwrap();
                                stmt.bind((2, composer.id)).unwrap();

                                stmt.next().unwrap();

                                let _ = app.emit("db_state", get_db_state(&conn));
                                db_state = get_db_state(&conn);
                                data.composers = Some(vec![composer]);
                                data.composers_order = Some((
                                    vec![Order::ByName],
                                    get_composers_order(&conn, vec![Order::ByName]),
                                ));
                            }
                            DBRequest::UpdateGenre(genre) => {
                                // Combining genres with the same name is not handled automatically.
                                // The user will in the future have to use the "Standardise Names" tool, once it is implemented.

                                let query = r#"
                                    UPDATE Genres SET name = ?1 WHERE genre_id = ?2
                                    "#
                                .to_string();

                                let mut stmt = conn.prepare(query).unwrap();
                                stmt.bind((1, genre.name.as_str())).unwrap();
                                stmt.bind((2, genre.id)).unwrap();

                                stmt.next().unwrap();

                                let _ = app.emit("db_state", get_db_state(&conn));
                                db_state = get_db_state(&conn);
                                data.genres = Some(vec![genre]);
                                data.genres_order = Some((
                                    vec![Order::ByName],
                                    get_genres_order(&conn, vec![Order::ByName]),
                                ));
                            }
                            DBRequest::UpdatePlaylist(playlist) => {
                                // TODO: test if playlist with given ID exists.
                                // If it does, replace with the new playlist
                                // TODO: Update
                                /*
                                pub struct Playlist {
                                    pub id: i64,
                                    pub name: String,
                                    pub date_modified: String,
                                    pub date_added: String,
                                    pub tracks: Vec<i64>,
                                }
                                 */
                                let query = r#"
                                SELECT playlist_id
                                FROM Playlists
                                WHERE playlist_id = ?1
                                "#
                                .to_string();

                                let mut stmt = conn.prepare(query).unwrap();
                                stmt.bind((1, playlist.id)).unwrap();

                                if let Ok(State::Row) = stmt.next() {
                                    let found_id =
                                        stmt.read::<i64, _>("playlist_id").unwrap_or_default();

                                    if found_id == playlist.id {
                                        let update_query = r#"
                                        UPDATE Playlists
                                        SET name = ?1, description = ?2, tracks = ?3, date_modified = CURRENT_TIMESTAMP
                                        WHERE playlist_id = ?4
                                        "#
                                        .to_string();

                                        let mut update_stmt = conn.prepare(update_query).unwrap();
                                        update_stmt.bind((1, playlist.name.as_str())).unwrap();
                                        update_stmt
                                            .bind((2, playlist.description.as_str()))
                                            .unwrap();
                                        update_stmt
                                            .bind((
                                                3,
                                                serde_json::to_string(&playlist.tracks)
                                                    .unwrap()
                                                    .as_str(),
                                            ))
                                            .unwrap();
                                        update_stmt.bind((4, playlist.id)).unwrap();

                                        let _ = update_stmt.next().unwrap();

                                        // TODO: verify playlist update, then send updated playlist
                                        data.playlists = Some(vec![playlist]);
                                        data.playlists_order = Some((
                                            vec![Order::ByName],
                                            get_playlists_order(&conn, vec![Order::ByName]),
                                        ));
                                    }
                                }
                            }
                            DBRequest::ExtractCovers => {
                                let _ = app.emit(
                                    "backend_message",
                                    BackendMessage {
                                        notification: None,
                                        error: None,
                                        warning: None,
                                        progress: Some(Progress {
                                            info: ProgressInfo::CoverExtract,
                                            value: None,
                                            done: false,
                                        }),
                                    },
                                );
                                let mut stmt = conn
                                    .prepare(
                                        "SELECT album_id
                                        FROM Albums
                                        WHERE cover_id IS 0 OR cover_id IS NULL",
                                    )
                                    .unwrap();

                                let mut album_ids: Vec<i64> = vec![];

                                while let Ok(State::Row) = stmt.next() {
                                    album_ids.push(stmt.read::<i64, _>("album_id").unwrap());
                                }

                                extract_cover_by_album(&conn, album_ids);
                                data.albums = Some(get_albums(&conn));

                                let _ = app.emit(
                                    "backend_message",
                                    BackendMessage {
                                        notification: None,
                                        error: None,
                                        warning: None,
                                        progress: Some(Progress {
                                            info: ProgressInfo::CoverExtract,
                                            value: None,
                                            done: true,
                                        }),
                                    },
                                );
                            }
                            DBRequest::ExtractCover(album_id) => {
                                let _ = app.emit(
                                    "backend_message",
                                    BackendMessage {
                                        notification: None,
                                        error: None,
                                        warning: None,
                                        progress: Some(Progress {
                                            info: ProgressInfo::CoverExtract,
                                            value: None,
                                            done: false,
                                        }),
                                    },
                                );
                                extract_cover_by_album(&conn, vec![album_id]);
                                // TODO: only update album for given id, frontend requests cover when required
                                data.albums = Some(get_albums(&conn));
                                //data.covers = Some(get_covers(&conn));
                                let _ = app.emit(
                                    "backend_message",
                                    BackendMessage {
                                        notification: None,
                                        error: None,
                                        warning: None,
                                        progress: Some(Progress {
                                            info: ProgressInfo::CoverExtract,
                                            value: None,
                                            done: true,
                                        }),
                                    },
                                );
                            }
                            DBRequest::Init => {
                                let _ = app.emit("db_state", get_db_state(&conn));
                                data = get_init_data(&conn);

                                //let _ = app.emit("data", get_init_data(&conn));
                                //data.covers = Some(get_covers(&conn));

                                //pending_covers = get_covers(&conn).chunks(100).map(|s| s.into()).collect();
                                /*
                                TODO:
                                instead of sending all the data at once, load small portion for faster time to first interaction.
                                Then gather the remaining data in chunks and send to frontend.
                                 */
                            }
                            DBRequest::UpdateConfig(new_config) => {
                                // Only relevant if changes to media path, allow delete from db, allow delete files, manage folders

                                // If anmutunes is setup to manage, change all the tracks locations in the managed path.
                                // Otherwise anmutunes is setup to check for changes in the media path and import based on filesystem timestamps
                                if new_config.manage_folders
                                    && config_state.media_path != new_config.media_path
                                {
                                    if let Some(tracks_max) = db_state.tracks_max {
                                        if tracks_max != 0 {
                                            update_media_path(
                                                &conn,
                                                config_state.media_path.clone(),
                                                new_config.media_path.clone(),
                                            );
                                        }
                                    }
                                }

                                config_state = new_config;
                            }
                            DBRequest::CopyNotCopied => {
                                if config_state.manage_folders {
                                    copy_unmanaged_tracks(&conn, config_state.media_path.clone());
                                }
                            }
                            DBRequest::OpenContainingDir(datatype, id) => {
                                match datatype {
                                    DataType::Artist => {
                                        // TODO: not every artist has a dir, only album_artists
                                        // This is also only true if managed by anmutunes...
                                        // For now only allow OpenContainingDir for DataType::Track
                                    }
                                    DataType::Album => {
                                        // TODO: get a track from the album and extract the album path from it
                                        // Potentially should test all tracks? If not managed by anmutunes they might be all over the place...
                                        // For now only allow OpenContainingDir for DataType::Track
                                    }
                                    DataType::Track => {
                                        // TODO: can use the track location
                                        let track = get_audiotracks_by_id(&conn, &vec![id]);
                                        match track {
                                            Some(track) => {
                                                if track.len() > 0 {
                                                    match tauri_plugin_opener::reveal_item_in_dir(
                                                        track[0].location.clone(),
                                                    ) {
                                                        Ok(()) => {}
                                                        Err(error) => {
                                                            error!("{}", error);
                                                        }
                                                    }
                                                }
                                            }
                                            None => {}
                                        }
                                    }
                                    DataType::Video => {
                                        // TODO: Video currently not used, videos are just tracks at the moment.
                                        todo!();
                                    }
                                    _ => {
                                        // Composer, Cover, Genre, Playlist do not have associated dir
                                    }
                                }
                            }
                            DBRequest::ListenedToTrack(id) => {
                                increment_plays_for_track(&conn, id);
                            }
                        },
                        Err(error) => {
                            error!("Error: {}", error);
                            // TODO: send info to frontend?
                        }
                    }

                    match app.emit("data", data) {
                        Ok(()) => {}
                        Err(error) => {
                            error!("{}", error);
                            // TODO: send info to frontend?
                        }
                    };
                }
            }
            Err(error) => {
                error!("{}", error);
            }
        }
    });
}
