// Code based on: https://github.com/pdeljanov/Symphonia/blob/master/symphonia-play/src/main.rs
use crate::defs::{Image, Meta};
use std::{fs::File, path::Path};
//use symphonia::core::formats::{Cue, FormatOptions, Track};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
//use symphonia::core::meta::{ColorMode, MetadataOptions, Tag, Value, Visual};
use symphonia::core::meta::{ColorMode, StandardVisualKey, Visual};
use symphonia::core::meta::{MetadataOptions, Tag};
use symphonia::core::probe::{Hint, ProbeResult};
//use symphonia::core::units::TimeBase;

/*
fn print_tracks(tracks: &[Track]) {
    if !tracks.is_empty() {
        println!("|");
        println!("| // Tracks //");

        for (idx, track) in tracks.iter().enumerate() {
            let params = &track.codec_params;

            print!("|     [{:0>2}] Codec:           ", idx + 1);

            if let Some(codec) = symphonia::default::get_codecs().get_codec(params.codec) {
                println!("{} ({})", codec.long_name, codec.short_name);
            } else {
                println!("Unknown (#{})", params.codec);
            }

            if let Some(sample_rate) = params.sample_rate {
                println!("|          Sample Rate:     {}", sample_rate);
            }
            if params.start_ts > 0 {
                if let Some(tb) = params.time_base {
                    println!(
                        "|          Start Time:      {} ({})",
                        fmt_time(params.start_ts, tb),
                        params.start_ts
                    );
                } else {
                    println!("|          Start Time:      {}", params.start_ts);
                }
            }
            if let Some(n_frames) = params.n_frames {
                if let Some(tb) = params.time_base {
                    println!(
                        "|          Duration:        {} ({})",
                        fmt_time(n_frames, tb),
                        n_frames
                    );
                } else {
                    println!("|          Frames:          {}", n_frames);
                }
            }
            if let Some(tb) = params.time_base {
                println!("|          Time Base:       {}", tb);
            }
            if let Some(padding) = params.delay {
                println!("|          Encoder Delay:   {}", padding);
            }
            if let Some(padding) = params.padding {
                println!("|          Encoder Padding: {}", padding);
            }
            if let Some(sample_format) = params.sample_format {
                println!("|          Sample Format:   {:?}", sample_format);
            }
            if let Some(bits_per_sample) = params.bits_per_sample {
                println!("|          Bits per Sample: {}", bits_per_sample);
            }
            if let Some(channels) = params.channels {
                println!("|          Channel(s):      {}", channels.count());
                println!("|          Channel Map:     {}", channels);
            }
            if let Some(channel_layout) = params.channel_layout {
                println!("|          Channel Layout:  {:?}", channel_layout);
            }
            if let Some(language) = &track.language {
                println!("|          Language:        {}", language);
            }
        }
    }
}
*/

/*
fn print_tag_item(idx: usize, key: &str, value: &Value, indent: usize) -> String {
    let key_str = match key.len() {
        0..=28 => format!("| {:w$}[{:0>2}] {:<28} : ", "", idx, key, w = indent),
        _ => format!(
            "| {:w$}[{:0>2}] {:.<28} : ",
            "",
            idx,
            key.split_at(26).0,
            w = indent
        ),
    };

    let line_prefix = format!("\n| {:w$} : ", "", w = indent + 4 + 28 + 1);
    let line_wrap_prefix = format!("\n| {:w$}   ", "", w = indent + 4 + 28 + 1);

    let mut out = String::new();

    out.push_str(&key_str);

    for (wrapped, line) in value.to_string().lines().enumerate() {
        if wrapped > 0 {
            out.push_str(&line_prefix);
        }

        let mut chars = line.chars();
        let split = (0..)
            .map(|_| chars.by_ref().take(72).collect::<String>())
            .take_while(|s| !s.is_empty())
            .collect::<Vec<_>>();

        out.push_str(&split.join(&line_wrap_prefix));
    }

    out
}
*/

/*
fn print_cues(cues: &[Cue]) {
    if !cues.is_empty() {
        println!("|");
        println!("| // Cues //");

        for (idx, cue) in cues.iter().enumerate() {
            println!("|     [{:0>2}] Track:      {}", idx + 1, cue.index);
            println!("|          Timestamp:  {}", cue.start_ts);

            // Print tags associated with the Cue.
            if !cue.tags.is_empty() {
                println!("|          Tags:");

                for (tidx, tag) in cue.tags.iter().enumerate() {
                    if let Some(std_key) = tag.std_key {
                        println!(
                            "{}",
                            print_tag_item(tidx + 1, &format!("{:?}", std_key), &tag.value, 21)
                        );
                    } else {
                        println!("{}", print_tag_item(tidx + 1, &tag.key, &tag.value, 21));
                    }
                }
            }

            // Print any sub-cues.
            if !cue.points.is_empty() {
                println!("|          Sub-Cues:");

                for (ptidx, pt) in cue.points.iter().enumerate() {
                    println!(
                        "|                      [{:0>2}] Offset:    {:?}",
                        ptidx + 1,
                        pt.start_offset_ts
                    );

                    // Start the number of sub-cue tags, but don't print them.
                    if !pt.tags.is_empty() {
                        println!(
                            "|                           Sub-Tags:  {} (not listed)",
                            pt.tags.len()
                        );
                    }
                }
            }
        }
    }
}
*/

/*
fn print_tags(tags: &[Tag]) {
    if !tags.is_empty() {
        println!("|");
        println!("| // Tags //");

        let mut idx = 1;

        // Print tags with a standard tag key first, these are the most common tags.
        for tag in tags.iter().filter(|tag| tag.is_known()) {
            if let Some(std_key) = tag.std_key {
                println!(
                    "{}",
                    print_tag_item(idx, &format!("{:?}", std_key), &tag.value, 4)
                );
            }
            idx += 1;
        }

        // Print the remaining tags with keys truncated to 26 characters.
        for tag in tags.iter().filter(|tag| !tag.is_known()) {
            println!("{}", print_tag_item(idx, &tag.key, &tag.value, 4));
            idx += 1;
        }
    }
}
*/

fn get_meta_cover(visuals: &[Visual]) -> Option<Image> {
    if !visuals.is_empty() {
        /*println!("|");
        println!("| // Visuals //");*/
        /*
        TODO:
        if usage exists only export if it is of StandardVisualKey "FrontCover"
        We do need the visual.media_type and visual.data to create the file and base64 string.
        What about other images stored?
        For quite a few file no cover is extracted, but in the file browser they clearly have a cover.
         */

        let mut cover_index = None;

        for (idx, visual) in visuals.iter().enumerate() {
            if let Some(usage) = visual.usage {
                if usage == StandardVisualKey::FrontCover {
                    cover_index = Some(idx);
                }
            }
        }

        if let Some(index) = cover_index {
            let visual = &visuals[index];
            let cover: Image = Image {
                media_type: visual.media_type.clone(),
                data: visual.data.clone().into_vec(),
            };

            return Some(cover);

            /*
            if let Some(dimensions) = visual.dimensions {
                println!(
                    "|          Dimensions: {} px x {} px",
                    dimensions.width, dimensions.height
                );
            }
            if let Some(bpp) = visual.bits_per_pixel {
                println!("|          Bits/Pixel: {}", bpp);
            }
            if let Some(ColorMode::Indexed(colors)) = visual.color_mode {
                println!("|          Palette:    {} colors", colors);
            }
            println!("|          Size:       {} bytes", visual.data.len());

            // Print out tags similar to how regular tags are printed.
            if !visual.tags.is_empty() {
                println!("|          Tags:");
            }

            for (tidx, tag) in visual.tags.iter().enumerate() {
                if let Some(std_key) = tag.std_key {
                    //println!(
                    //    "{}",
                    //    print_tag_item(tidx + 1, &format!("{:?}", std_key), &tag.value, 21)
                    //);
                } else {
                    //println!("{}", print_tag_item(tidx + 1, &tag.key, &tag.value, 21));
                }
            }*/
        }
    }
    return None;
}

fn fill_meta_tags(mut meta: Meta, tags: &[Tag]) -> Meta {
    if !tags.is_empty() {
        // Print tags with a standard tag key first, these are the most common tags.
        for tag in tags.iter().filter(|tag| tag.is_known()) {
            if let Some(std_key) = tag.std_key {
                match format!("{:?}", std_key).as_str() {
                    "TrackTitle" => {
                        meta.name = Some(tag.value.to_string());
                    }
                    "Artist" => {
                        meta.artist = Some(tag.value.to_string());
                    }
                    "AlbumArtist" => {
                        meta.album_artist = Some(tag.value.to_string());
                    }
                    "Composer" => {
                        meta.composer = Some(tag.value.to_string());
                    }
                    "Album" => {
                        meta.album = Some(tag.value.to_string());
                    }
                    "Genre" => {
                        meta.genre = Some(tag.value.to_string());
                    }
                    "DiscNumber" => {
                        match tag.value.to_string().parse::<i64>() {
                            Ok(value) => {
                                meta.disc_number = Some(value);
                            }
                            Err(_error) => {}
                        };
                    }
                    "DiscTotal" => {
                        match tag.value.to_string().parse::<i64>() {
                            Ok(value) => {
                                meta.disc_count = Some(value);
                            }
                            Err(_error) => {}
                        };
                    }
                    "TrackNumber" => {
                        match tag.value.to_string().parse::<i64>() {
                            Ok(value) => {
                                meta.track_number = Some(value);
                            }
                            Err(_error) => {}
                        };
                    }
                    "TrackTotal" => {
                        match tag.value.to_string().parse::<i64>() {
                            Ok(value) => {
                                meta.track_count = Some(value);
                            }
                            Err(_error) => {}
                        };
                    }
                    "Date" => {
                        // TODO: get release date and year for meta
                        /*
                        year: None,  // tags[11]
                        release_date: None, // tags[11]
                        [11] Date                         : 2025-08-22
                        */
                        meta.release_date = Some(tag.value.to_string());
                    }
                    _ => {}
                }
            }
        }

        // TODO: could there be tags of interest in here?
        // Print the remaining tags with keys truncated to 26 characters.
        /*for tag in tags.iter().filter(|tag| !tag.is_known()) {
            println!("{}", print_tag_item(idx, &tag.key, &tag.value, 4));
            idx += 1;
        }*/
    }

    return meta;
}

fn get_meta(probed: &mut ProbeResult) -> Meta {
    let mut meta: Meta = Meta {
        name: None,         // tags[02]
        artist: None,       // check Artists table, create new if not existing from tags[05]
        album_artist: None, // check Artists table, create new if not existing from tags[04]
        composer: None,     // check Composers table, create new if not existing from tags[12]
        album: None,        // check Albums table, create new if not existing from tags[03]
        genre: None, // check Genres table, create new if not existing from inexistent? tags[]
        kind: None,  // Tracks [01] Codec
        size: None,  // not available? calculate from copy file or os call.
        total_time: None, // Tracks [01] Duration
        disc_number: None, // tags[09]
        disc_count: None, // tags[10]
        track_number: None, // tags[07]
        track_count: None, // tags[08]
        year: None,  // tags[11]
        bit_rate: None, // Tracks [01] sample_rate * bits_per_sample / 1024 / 8      (96000*24/1024/8 = 281,25)
        sample_rate: None, // Tracks [01]
        release_date: None, // tags[11]
        // normalization: not really available... tags[15-18] have values, but not sure if useful, also so far no use for normalization value.,
        // artwork_count: -1, although theoretically I could use Visuals [01] FrontCover,
        // sort_name: not available, also not really used,
        // persistent_id: not available, can calculate hash with salt of current time to prevent collision or pass some uuid,
        // track_type: "File", aka useless,
        // purchased: NULL,
        location: None, // path, update if copied,
        // file_folder_count: always 5 as it seems...,
        // library_folder_count: always 1 as it seems...,
        cover: None,
    };
    /*
    + /run/user/1000/gvfs/smb-share:server=silentchest,share=daten/Einsortieren/Deftones/private music(Explicit)/01. Deftones - my mind is a mountain(Explicit).flac
    |
    | // Tracks //
    |     [01] Codec:           Free Lossless Audio Codec (flac)
    |          Sample Rate:     96000
    |          Duration:        0:02:50.942 (16410411)
    |          Time Base:       1/96000
    |          Bits per Sample: 24
    |          Channel(s):      2
    |          Channel Map:     0b000000000000000000000000000011
    |
    */
    /*
    | // Tags //
    |     [01] Encoder                      : Lavf60.16.100
    |     [02] TrackTitle                   : my mind is a mountain
    |     [03] Album                        : private music
    |     [04] AlbumArtist                  : Deftones
    |     [05] Artist                       : Deftones
    |     [06] Copyright                    : ℗ 2025 Reprise Records
    |     [07] TrackNumber                  : 1
    |     [08] TrackTotal                   : 11
    |     [09] DiscNumber                   : 1
    |     [10] DiscTotal                    : 1
    |     [11] Date                         : 2025-08-22
    |     [12] Composer                     :
    |     [13] IdentIsrc                    : USRE12500150
    |     [14] Lyrics                       :
    |     [15] ReplayGainAlbumGain          : -12.37
    |     [16] ReplayGainAlbumPeak          : 1.0
    |     [17] ReplayGainTrackGain          : -11.83
    |     [18] ReplayGainTrackPeak          : 0.999885
    |     [19] major_brand                  : iso8
    |     [20] minor_version                : 0
    |     [21] compatible_brands            : mp41dash
    |     [22] URL                          : https://tidal.com/browse/track/455128515
    |*/
    /*
    | // Visuals //
    |     [01] Usage:      FrontCover
    |          Media Type: image/jpeg
    |          Size:       34169 bytes
    |          Tags:
    |                      [01] Description                  :
    :
     */
    let tracks = probed.format.tracks();
    if !tracks.is_empty() {
        for track in tracks.iter() {
            let params = &track.codec_params;

            if let Some(codec) = symphonia::default::get_codecs().get_codec(params.codec) {
                meta.kind = Some(codec.long_name.to_string());
            }

            if let Some(sample_rate) = params.sample_rate {
                meta.sample_rate = Some(sample_rate as i64);
            }

            if let Some(n_frames) = params.n_frames {
                if let Some(tb) = params.time_base {
                    let time = tb.calc_time(n_frames);
                    let ms = f64::from((time.seconds) as u32) + time.frac;
                    meta.total_time = Some((ms * 1000.0).round() as i64);
                }
            }

            if let Some(bits_per_sample) = params.bits_per_sample {
                if let Some(sample_rate) = params.sample_rate {
                    meta.bit_rate = Some((sample_rate * bits_per_sample / 1024) as i64);
                }
            }

            // We do not allow multiple tracks in one file
            break;
        }
    }

    if let Some(metadata_rev) = probed.format.metadata().current() {
        meta = fill_meta_tags(meta, metadata_rev.tags());
        meta.cover = get_meta_cover(metadata_rev.visuals());

        // Warn that certain tags are preferred.
        if probed.metadata.get().as_ref().is_some() {
            println!("tags that are part of the container format are preferentially printed.");
            println!("not printing additional tags that were found while probing.");
        }
    } else if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
        meta = fill_meta_tags(meta, metadata_rev.tags());
        meta.cover = get_meta_cover(metadata_rev.visuals());
    }

    return meta;
}

/*
fn print_format(path: &Path, probed: &mut ProbeResult) {
    println!("+ {}", path.display());
    print_tracks(probed.format.tracks());

    // Prefer metadata that's provided in the container format, over other tags found during the
    // probe operation.
    if let Some(metadata_rev) = probed.format.metadata().current() {
        print_tags(metadata_rev.tags());
        print_visuals(metadata_rev.visuals());

        // Warn that certain tags are preferred.
        if probed.metadata.get().as_ref().is_some() {
            println!("tags that are part of the container format are preferentially printed.");
            println!("not printing additional tags that were found while probing.");
        }
    } else if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
        print_tags(metadata_rev.tags());
        print_visuals(metadata_rev.visuals());
    }

    print_cues(probed.format.cues());
    println!(":");
    println!();
}
*/

pub fn extract_metadata(path: &Path) -> Option<Meta> {
    /*
    TODO:
    print some more infos. Some files fail to extract, especially the cover is important.
     */
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_e) => return None,
    };

    // Create the media source stream.
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // Use the default options for format readers other than for gapless playback.
    let format_opts = FormatOptions {
        ..Default::default()
    };

    // Use the default options for metadata readers.
    let metadata_opts: MetadataOptions = Default::default();

    // Create a hint to help the format registry guess what format reader is appropriate.
    let mut hint = Hint::new();

    // Provide the file extension as a hint.
    if let Some(extension) = path.extension() {
        if let Some(extension_str) = extension.to_str() {
            println!("Extension: {}", extension_str);
            hint.with_extension(extension_str);
        }
    }

    // Probe the media source stream for metadata and get the format reader.
    match symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts) {
        Ok(mut probed) => {
            //print_format(path, &mut probed);
            return Some(get_meta(&mut probed));
        }
        Err(_err) => {
            // The input was not supported by any format reader.
            println!("the input is not supported");
            return None;
        }
    }
}

/*
fn fmt_time(ts: u64, tb: TimeBase) -> String {
    let time = tb.calc_time(ts);

    let hours = time.seconds / (60 * 60);
    let mins = (time.seconds % (60 * 60)) / 60;
    let secs = f64::from((time.seconds % 60) as u32) + time.frac;

    format!("{}:{:0>2}:{:0>6.3}", hours, mins, secs)
}
*/

/*
+ /run/media/andreas/ExDaten/iTunes/iTunes Media/Music/Chevelle/NIRATIAS/1-01 Verruckt.mp3
|
| // Tracks //
|     [01] Codec:           MPEG Audio Layer 3 (mp3)
|          Sample Rate:     44100
|          Duration:        0:03:30.808 (9296640)
|          Time Base:       1/44100
|          Encoder Delay:   1105
|          Encoder Padding: 431
|          Channel(s):      2
|          Channel Map:     0b000000000000000000000000000011
|
| // Tags //
|     [01] AlbumArtist                  : Chevelle
|     [02] TrackTitle                   : Verruckt
|     [03] Artist                       : Chevelle
|     [04] TrackNumber                  : 1
|     [05] Album                        : NIRATIAS
|     [06] DiscNumber                   : 1
|     [07] Genre                        : GetMetal.club
|     [08] Bpm                          : 0
|     [09] Date                         : 0305
|     [10] Label                        : Epic
|     [11] Date                         : 2021
|     [12] Comment                      : www.NewAlbumReleases.net
|     [13] TLEN                         : 210000
|     [14] TXXX:COMMENT                 : GetMetal.club
|     [15] IPLS                         : author
|     [16] IPLS                         : Pete Loeffler
|
| // Visuals //
|     [01] Usage:      FrontCover
|          Media Type: image/jpeg
|          Size:       166124 bytes
|          Tags:
|                      [01] Description                  : cover
:
 */

/*
+ /run/media/andreas/ExDaten/iTunes/iTunes Media/Music/Noga Erez/KIDS (Against the Machine)/01 KTD (Against the Machine).m4a
|
| // Tracks //
|     [01] Codec:           Apple Lossless Audio Codec (alac)
|          Sample Rate:     44100
|          Duration:        0:00:13.293 (586221)
|          Time Base:       1/44100
|
| // Tags //
|     [01] TrackTitle                   : KTD (Against the Machine)
|     [02] Artist                       : Noga Erez
|     [03] AlbumArtist                  : Noga Erez
|     [04] Album                        : KIDS (Against the Machine)
|     [05] Composer                     :
|     [06] TrackNumber                  : 1
|     [07] TrackTotal                   : 14
|     [08] Date                         : 2021-03-26
|     [09] Lyrics                       :
|     [10] com.apple.iTunes:iTunes_CD.. : B1095E0E+180060+14+150+1146+19650+31050+46759+59898+73599+89387+103618+1
|                                         14228+128060+141812+156005+170141
|     [11] com.apple.iTunes:iTunes_CD.. : 1
|     [12] com.apple.iTunes:URL         : https://tidal.com/browse/track/334921643
|     [13] com.apple.iTunes:COPYRIGHT   : City Slang
|
| // Visuals //
|     [01] Usage:      FrontCover
|          Media Type: image/jpeg
|          Size:       11306 bytes
:
 */
