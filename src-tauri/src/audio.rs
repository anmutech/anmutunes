extern crate vlc;
use crate::defs::{AudioBackendState, AudioRequest, AudioState, DBData, DBRequest, RepeatMode};
use log::{debug, error};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use tauri::{AppHandle, Emitter, Manager};
use vlc::{Event, EventType, Instance, Media, MediaPlayer, MediaPlayerAudioEx, Meta};

fn format_time(milis: i64) -> String {
    let secs = milis / 1000; // convert miliseconds to seconds
    let min = secs / 60; // get the minutes value
    let sec = secs % 60; // get the remaining seconds

    return format!("{}:{:02}", min, sec);
}

fn get_metadata(media: Media) -> String {
    let metadata = format!(
        "Artist: {}\nTitle: {}\nAlbum: {}\nGenre: {}\nTrackNumber: {}\nTrackTotal: {}\nTrackID: {}\nArtworkURL: {}\nURL: {}\nDescription: {}\nPublisher: {}\nDate: {}",
        media
            .get_meta(Meta::Artist)
            .unwrap_or_else(|| "".to_string()),
        media
            .get_meta(Meta::Title)
            .unwrap_or_else(|| "".to_string()),
        media
            .get_meta(Meta::Album)
            .unwrap_or_else(|| "".to_string()),
        media
            .get_meta(Meta::Genre)
            .unwrap_or_else(|| "".to_string()),
        media
            .get_meta(Meta::TrackNumber)
            .unwrap_or_else(|| "".to_string()),
        media
            .get_meta(Meta::TrackTotal)
            .unwrap_or_else(|| "".to_string()),
        media
            .get_meta(Meta::TrackID)
            .unwrap_or_else(|| "".to_string()),
        media
            .get_meta(Meta::ArtworkURL)
            .unwrap_or_else(|| "".to_string()),
        media.get_meta(Meta::URL).unwrap_or_else(|| "".to_string()),
        media
            .get_meta(Meta::Description)
            .unwrap_or_else(|| "".to_string()),
        media
            .get_meta(Meta::Publisher)
            .unwrap_or_else(|| "".to_string()),
        media.get_meta(Meta::Date).unwrap_or_else(|| "".to_string())
    );

    return metadata;
}

pub fn get_audio_backend_state(
    audio_backend_state_path: &PathBuf,
) -> Result<AudioBackendState, String> {
    match File::open(&audio_backend_state_path) {
        Ok(mut audio_backend_state_file) => {
            let mut audio_backend_state_json = "".to_string();

            match audio_backend_state_file.read_to_string(&mut audio_backend_state_json) {
                Ok(_) => match serde_json::from_str::<AudioBackendState>(&audio_backend_state_json)
                {
                    Ok(audio_backend_state) => {
                        return Ok(audio_backend_state);
                    }
                    Err(error) => {
                        error!("{}", error);
                        return Err(format!(
                            "{:?} is not a valid audio_backend_state.json for anmutunes",
                            audio_backend_state_path
                        ));
                    }
                },
                Err(error) => {
                    error!("{}", error);
                    return Err(format!(
                        "Could not read file: {:?}",
                        audio_backend_state_path
                    ));
                }
            }
        }
        Err(error) => {
            error!("{}", error);
            return Err(format!(
                "Could not open file: {:?}",
                audio_backend_state_path
            ));
        }
    }
}

pub fn set_audio_backend_state(
    audio_backend_state_name: &str,
    audio_backend_state_path: &PathBuf,
    audio_backend_state: AudioBackendState,
) {
    if audio_backend_state_path.ends_with(audio_backend_state_name) {
        match serde_json::to_string(&audio_backend_state) {
            Ok(audio_backend_state_json) => match File::options()
                .write(true)
                .truncate(true)
                .open(&audio_backend_state_path)
            {
                Ok(mut audio_backend_state_file) => {
                    match audio_backend_state_file.write_all(audio_backend_state_json.as_bytes()) {
                        Ok(()) => {}
                        Err(error) => {
                            error!("{}", error);
                        }
                    }
                }
                Err(error) => {
                    error!("{}", error);
                    match File::create(&audio_backend_state_path) {
                        Ok(mut audio_backend_state_file) => {
                            match audio_backend_state_file
                                .write_all(audio_backend_state_json.as_bytes())
                            {
                                Ok(()) => {}
                                Err(error) => {
                                    error!("{}", error);
                                }
                            }
                        }
                        Err(error) => {
                            error!("{}", error);
                        }
                    }
                }
            },
            Err(error) => {
                error!("{}", error);
            }
        }
    }
}

pub fn create_player_instance(
    app: AppHandle,
    audio_receiver: Receiver<AudioRequest>,
    db_data_receiver: Receiver<DBData>,
    db_sender_audio: Sender<DBRequest>,
) {
    /*
    TODO:
    Detect when anmutunes gets terminated.
    In a graceful shutdown, store play position of track.
     */
    thread::spawn(move || loop {
        // VLC thread
        /*
         * Init vlc and loop to receive input and event
         */

        #[cfg(target_os = "macos")]
        {
            match std::env::current_exe() {
                Ok(mut dir) => {
                    // Remove executable and MacOS dir
                    dir.pop();
                    dir.pop();

                    // On macOS set_var seems to be unsafe
                    unsafe {
                        std::env::set_var(
                            "VLC_PLUGIN_PATH",
                            dir.join("Resources")
                                .join("vlc")
                                .join("macos")
                                .join("plugins")
                                .display()
                                .to_string(),
                        )
                    };
                    /*set_var(
                        "VLC_PLUGIN_PATH",
                        "/Applications/anmutunes.app/Contents/Resources/vlc/plugins",
                    );*/
                }
                Err(_error) => {
                    // TODO: send to frontend
                }
            }
        }

        // Create an instance of VLC
        let instance = Instance::new().unwrap();

        // Vector of track IDs
        let mut queue_ids: Vec<i64> = vec![];
        let mut history_ids: Vec<i64> = vec![];

        // Vector containing the paths or URLs to your media files
        let mut queue: Vec<String> = vec![];
        let mut history: Vec<String> = vec![];

        let mut current_id: i64 = 0;
        let mut current_location: String = "".to_string();

        let mut shuffle: bool = false;
        let mut rng: ThreadRng = rand::rng();

        let mut repeat: RepeatMode = RepeatMode::RepeatNone;

        let mut volume: i32 = 100;

        // Create a media player
        let mediaplayer = MediaPlayer::new(&instance).unwrap();

        let _ = mediaplayer.set_volume(volume);

        // Set up event listener for end of media
        let events = mediaplayer.event_manager();

        let (event_sender, event_receiver) = mpsc::channel::<Event>();
        let sender_copy = event_sender.clone();
        let _ = events.attach(EventType::MediaPlayerEndReached, move |e, _| {
            let _ = sender_copy.send(e);
        });

        let sender_copy2 = event_sender.clone();
        let _ = events.attach(EventType::MediaPlayerMediaChanged, move |e, _| {
            let _ = sender_copy2.send(e);
        });

        let audio_backend_state_name = if tauri::is_dev() {
            "debug.audio_backend_state.json"
        } else {
            "audio_backend_state.json"
        };

        let audio_backend_state_path = match app.path().app_local_data_dir() {
            Ok(mut data_path) => {
                data_path.push(audio_backend_state_name);
                data_path
            }
            Err(error) => {
                error!("{}", error);
                PathBuf::new()
            }
        };

        let mut old_position = 0;

        if audio_backend_state_path.ends_with(audio_backend_state_name) {
            match get_audio_backend_state(&audio_backend_state_path) {
                Ok(audio_backend_state) => {
                    shuffle = audio_backend_state.shuffle_mode;
                    repeat = audio_backend_state.repeat_mode;
                    current_id = audio_backend_state.current_id;
                    // TODO: this data is not trustworthy and should not be used. Instead get location from index.
                    current_location = audio_backend_state.current_location;
                    if current_location != "" {
                        mediaplayer.set_media(
                            &Media::new_path(&instance, current_location.clone()).unwrap(),
                        );
                    }
                    volume = audio_backend_state.volume;
                    let _ = mediaplayer.set_volume(volume);
                    old_position = audio_backend_state.position;
                    let _ = db_sender_audio
                        .send(DBRequest::AudioBackendRecover(
                            audio_backend_state.history,
                            audio_backend_state.queue,
                        ))
                        .unwrap();
                }
                Err(error) => {
                    error!("{}", error);
                }
            }
        }

        // Event and input handling loop
        loop {
            // TODO: package all requests into single receiver to get rid of delays?
            // Wait for mediaplayer event or user input
            let audio_request = audio_receiver.recv_timeout(std::time::Duration::from_millis(100));
            let vlc_event = event_receiver.recv_timeout(std::time::Duration::from_millis(100));
            let db_data = db_data_receiver.recv_timeout(std::time::Duration::from_millis(100));

            let mut audio_state = AudioState {
                is_playing: None,
                is_muted: None,
                volume: None,
                output: None,
                position: None,
                shuffle_mode: None,
                repeat_mode: None,
                current_track: None,
                queue: None,
                history: None,
            };

            let mut write_audio_backend_state = false;

            /*
            TODO: create function for Play and Next that covers all playback options, including shuffle.
             */

            match audio_request {
                Ok(result) => match result {
                    AudioRequest::PlayPause(is_playing) => {
                        if current_id != 0 {
                            println!("Playing: {}", current_id);
                            if is_playing {
                                println!("start playing");
                                match mediaplayer.play() {
                                    Ok(()) => {
                                        // Workaround to recover old playback position
                                        if old_position != 0 {
                                            mediaplayer.set_time(old_position);
                                            old_position = 0;
                                        }

                                        audio_state.is_playing = Some(true);
                                        audio_state.current_track = Some(current_id);
                                    }
                                    Err(_error) => {}
                                }
                            } else {
                                println!("stop playing");
                                mediaplayer.pause();
                                audio_state.is_playing = Some(false);
                            }

                            write_audio_backend_state = true;
                        } else {
                            println!("Nothing to play/pause");
                            audio_state.is_playing = Some(false);
                        }
                    }
                    AudioRequest::Next => {
                        let is_playing = mediaplayer.is_playing();
                        if queue.len() > 0 || repeat != RepeatMode::RepeatNone {
                            /*
                            TODO:
                            unload prev_media
                            move current_media into prev_media
                            move next_media into current_media
                            load new next_media
                             */

                            // TODO: needs proper testing, this has code smell...
                            if current_id != 0 {
                                history_ids.push(current_id);
                                history.push(current_location.clone());
                            }

                            match repeat {
                                RepeatMode::RepeatNone => {}
                                RepeatMode::RepeatQueue => {
                                    queue_ids.push(current_id);
                                    queue.push(current_location);
                                }
                                RepeatMode::RepeatTrack => {
                                    queue_ids.insert(0, current_id);
                                    queue.insert(0, current_location);
                                }
                            }

                            let mut shuffled_index: usize = 0;

                            if shuffle && repeat != RepeatMode::RepeatTrack {
                                shuffled_index = rng.random_range(..queue.len());
                            }

                            current_id = queue_ids.remove(shuffled_index);
                            current_location = queue.remove(shuffled_index);

                            mediaplayer.set_media(
                                &Media::new_path(&instance, current_location.clone()).unwrap(),
                            );

                            if is_playing {
                                mediaplayer.play().unwrap();
                            }

                            audio_state.queue = Some(queue_ids.clone());
                        } else {
                            if current_id != 0 {
                                history_ids.push(current_id);
                                history.push(current_location);
                            }

                            current_id = 0;
                            current_location = "".to_string();

                            mediaplayer.stop();
                        }

                        write_audio_backend_state = true;
                        audio_state.current_track = Some(current_id);
                        audio_state.history = Some(history_ids.clone());
                    }
                    AudioRequest::Prev => {
                        if history.len() > 0 {
                            /*
                            TODO:
                            unload next_media
                            move current_media into next_media
                            move prev_media into current_media
                            load new prev_media
                             */
                            let is_playing = mediaplayer.is_playing();

                            // TODO: needs proper testing, this has code smell...
                            queue_ids.insert(0, current_id);
                            queue.insert(0, current_location);

                            current_id = history_ids.pop().unwrap();
                            current_location = history.pop().unwrap();

                            mediaplayer.set_media(
                                &Media::new_path(&instance, current_location.clone()).unwrap(),
                            );

                            audio_state.current_track = Some(current_id);
                            audio_state.queue = Some(queue_ids.clone());
                            audio_state.history = Some(history_ids.clone());

                            if is_playing {
                                mediaplayer.play().unwrap();
                            }
                        } else {
                            mediaplayer.stop();
                        }

                        write_audio_backend_state = true;
                        // TODO: emit state for new active track? Requires ID
                        //app.emit("audio_state", AudioState current track and maybe position(aka time?)).unwrap();
                    }
                    AudioRequest::QueueJump(index) => {
                        // TODO: What is proper behaviour? move all previous tracks into history?
                        // Move current into history, discard all others before selected queue_id
                        // Move selected queue_id into current.
                        let id: usize = index.try_into().unwrap();
                        if queue.len() > id {
                            // TODO: needs proper testing, this has code smell...
                            if current_id != 0 {
                                history_ids.push(current_id);
                                history.push(current_location);
                            }

                            current_id = queue_ids[id];
                            current_location = queue[id].clone();

                            //  remove entries from queue up to and including id
                            queue_ids.drain(0..=id);
                            queue.drain(0..=id);

                            mediaplayer.set_media(
                                &Media::new_path(&instance, current_location.clone()).unwrap(),
                            );
                            mediaplayer.play().unwrap();

                            audio_state.current_track = Some(current_id);
                            audio_state.queue = Some(queue_ids.clone());
                            audio_state.history = Some(history_ids.clone());
                            audio_state.position = Some(mediaplayer.get_time().unwrap_or_default());
                        } else {
                            mediaplayer.stop();
                        }

                        write_audio_backend_state = true;
                        audio_state.is_playing = Some(mediaplayer.is_playing());
                    }
                    AudioRequest::QueueMove(vec_ids) => {
                        // vec_ids contains the track_ids in the new order
                        let mut new_queue = vec![];
                        let mut new_queue_ids = vec![];
                        for id in vec_ids {
                            let index = queue_ids.iter().position(|&i| i == id).unwrap();
                            new_queue.push(queue[index].clone());
                            new_queue_ids.push(queue_ids[index]);
                        }

                        queue = new_queue;
                        queue_ids = new_queue_ids;

                        write_audio_backend_state = true;
                        audio_state.queue = Some(queue_ids.clone());
                    }
                    AudioRequest::QueueRemove(indices) => {
                        // sort indices then remove in descending order
                        let mut mut_indices = indices.clone();
                        mut_indices.sort();
                        for index in mut_indices.iter().rev() {
                            let index: usize = index.to_owned().try_into().unwrap();
                            if index < queue_ids.len() {
                                queue_ids.remove(index);
                                queue.remove(index);
                            }

                            audio_state.queue = Some(queue_ids.clone());
                        }

                        write_audio_backend_state = true;
                    }
                    AudioRequest::HistoryJump(history_id) => {
                        // TODO: What is proper behaviour?
                        // Move current into history.
                        // Move selected history_id into current.
                        let id: usize = history_id.try_into().unwrap();
                        if history.len() > id {
                            // TODO: needs proper testing, this has code smell...
                            if current_id != 0 {
                                history_ids.push(current_id);
                                history.push(current_location);
                            }

                            current_id = history_ids[id];
                            current_location = history[id].clone();

                            mediaplayer.set_media(
                                &Media::new_path(&instance, current_location.clone()).unwrap(),
                            );
                            mediaplayer.play().unwrap();

                            audio_state.current_track = Some(current_id);
                            audio_state.queue = Some(queue_ids.clone());
                            audio_state.history = Some(history_ids.clone());
                            audio_state.position = Some(mediaplayer.get_time().unwrap_or_default());
                        } else {
                            mediaplayer.stop();
                        }

                        write_audio_backend_state = true;
                        audio_state.is_playing = Some(mediaplayer.is_playing());
                    }
                    AudioRequest::HistoryRemove => {
                        history_ids = vec![];

                        write_audio_backend_state = true;
                        audio_state.history = Some(history_ids.clone());
                    }
                    AudioRequest::Mute(new_mute) => {
                        mediaplayer.set_mute(new_mute);
                        audio_state.is_muted = Some(new_mute);
                    }
                    AudioRequest::Volume(new_volume) => {
                        match mediaplayer.set_volume(new_volume) {
                            Ok(()) => {
                                volume = new_volume;
                                audio_state.volume = Some(volume);
                            }
                            Err(_error) => {}
                        }
                        println!("{}", volume);
                        write_audio_backend_state = true;
                    }
                    AudioRequest::Output(_index) => {
                        // TODO: set output
                    }
                    AudioRequest::Seek(time) => {
                        mediaplayer.set_time(time);

                        // Workaround since set time on not yet played media fails
                        if old_position != 0 {
                            old_position = time;
                        }

                        if let Some(time) = mediaplayer.get_time() {
                            audio_state.position = Some(time);
                        }
                    }
                    AudioRequest::Shuffle(new_shuffle) => {
                        write_audio_backend_state = true;
                        shuffle = new_shuffle;
                    }
                    AudioRequest::Repeat(new_repeat) => {
                        write_audio_backend_state = true;
                        repeat = new_repeat;
                    }
                    AudioRequest::Init => {
                        // TODO: emit current audio state to frontend.
                        // Also we should send queue_ids so the frontend can retrieve track data
                        audio_state.is_playing = Some(mediaplayer.is_playing());
                        audio_state.is_muted = Some(mediaplayer.get_mute().unwrap_or_default());
                        audio_state.volume = Some(volume);
                        audio_state.position = Some(mediaplayer.get_time().unwrap_or_default());
                        audio_state.current_track = Some(current_id);
                        audio_state.queue = Some(queue_ids.clone());
                        audio_state.history = Some(history_ids.clone());
                    }
                },
                Err(_) => {
                    //eprintln!("Error: {}", e);
                }
            }

            /*
            AudioRequest::Play(datatype, vec_id, opt_index) => {
                        /* TODO:
                        This frontendevent should instead go to the db, which should then send an event to audio.

                            based on datatype query the db for all entries in vec_id.
                            For tracks, get the paths for all tracks in vec_id.
                            For albums, get all tracks per album and insert them into the queue
                            For playlists, get all tracks per playlist and insert them into the queue
                        */

                        /*current_index = 0;

                        queue = vec![];
                        // For each entry in the paths vector we need to do this:
                        for path in paths.iter() {
                            queue.append(&mut vec![Media::new_path(&instance, path).unwrap()]);
                        }

                        mediaplayer.set_media(&queue[current_index]);
                        mediaplayer.play();*/
                    }
             */

            /*AudioRequest::INFO => {
                if current_index == queue.len() {
                    println!(
                        "Currently playing:\nNone\nPlaylist index: {}\n{:?}",
                        current_index, queue_paths
                    );
                } else {
                    let current_time =
                        format_time(mediaplayer.get_time().unwrap_or_else(|| 0));
                    let mut duration = format_time(0);

                    if let Some(media) = mediaplayer.get_media() {
                        duration = format_time(media.duration().unwrap_or_else(|| 0));
                    }

                    let mut metadata = "".to_string();
                    if let Some(media) = mediaplayer.get_media() {
                        metadata = get_metadata(media);
                    }

                    println!(
                        "Currently playing:\n{}\n{} - {}\n{}\nPlaylist index: {}\n{:?}",
                        queue_paths[current_index],
                        current_time,
                        duration,
                        metadata,
                        current_index,
                        queue_paths
                    );
                }
            }*/

            match vlc_event {
                Ok(result) => match result {
                    Event::MediaPlayerEndReached => {
                        println!("NEXT");
                        // TOOD: Should we only increment if the user did not seek in the track?
                        let _ = db_sender_audio
                            .send(DBRequest::ListenedToTrack(current_id))
                            .unwrap();

                        if queue.len() > 0 || repeat != RepeatMode::RepeatNone {
                            // TODO: needs proper testing, this has code smell...
                            if current_id != 0 {
                                history_ids.push(current_id);
                                history.push(current_location.clone());
                            }

                            match repeat {
                                RepeatMode::RepeatNone => {}
                                RepeatMode::RepeatQueue => {
                                    queue_ids.push(current_id);
                                    queue.push(current_location);
                                }
                                RepeatMode::RepeatTrack => {
                                    queue_ids.insert(0, current_id);
                                    queue.insert(0, current_location);
                                }
                            }

                            let mut shuffled_index: usize = 0;

                            if shuffle && repeat != RepeatMode::RepeatTrack {
                                shuffled_index = rng.random_range(..queue.len());
                            }

                            current_id = queue_ids.remove(shuffled_index);
                            current_location = queue.remove(shuffled_index);

                            mediaplayer.set_media(
                                &Media::new_path(&instance, current_location.clone()).unwrap(),
                            );

                            mediaplayer.play().unwrap();

                            audio_state.queue = Some(queue_ids.clone());
                        } else {
                            if current_id != 0 {
                                history_ids.push(current_id);
                                history.push(current_location);
                            }

                            current_id = 0;
                            current_location = "".to_string();

                            mediaplayer.stop();
                        }

                        write_audio_backend_state = true;
                        audio_state.is_playing = Some(mediaplayer.is_playing());
                        audio_state.current_track = Some(current_id);
                        audio_state.history = Some(history_ids.clone());
                    }
                    _ => {}
                },
                Err(_) => {
                    //eprintln!("Error: {}", e);
                }
            }

            match db_data {
                Ok(result) => match result {
                    DBData::Play(audiotracks) => {
                        queue = vec![];
                        queue_ids = vec![];

                        for audiotrack in audiotracks.iter() {
                            queue.push(audiotrack.location.clone());
                            queue_ids.push(audiotrack.id);
                        }

                        if current_id != 0 {
                            history_ids.push(current_id);
                            history.push(current_location);
                        }

                        let mut shuffled_index: usize = 0;

                        // TODO: correct? repeat track would not play from new tracks...
                        if shuffle && repeat != RepeatMode::RepeatTrack {
                            shuffled_index = rng.random_range(..queue.len());
                        }

                        current_id = queue_ids.remove(shuffled_index);
                        current_location = queue.remove(shuffled_index);

                        mediaplayer.set_media(
                            &Media::new_path(&instance, current_location.clone()).unwrap(),
                        );
                        let _ = mediaplayer.play();

                        write_audio_backend_state = true;
                        audio_state.is_playing = Some(mediaplayer.is_playing());
                        audio_state.current_track = Some(current_id);
                        audio_state.queue = Some(queue_ids.clone());
                        audio_state.history = Some(history_ids.clone());
                    }
                    DBData::QueueInsert(audiotracks, opt_index) => {
                        /*
                        TODO:
                        Requires update of current_index to point to the same track as before.
                        The index only changes in the case where something is inserted before the current track,
                        which only is true for opt_index <= current_index.
                         */

                        if let Some(index) = opt_index {
                            for (audioindex, audiotrack) in audiotracks.iter().enumerate() {
                                queue.insert(index + audioindex, audiotrack.location.clone());
                                queue_ids.insert(index + audioindex, audiotrack.id);
                            }
                        } else {
                            // Without an index, just append the tracks
                            for audiotrack in audiotracks.iter() {
                                queue.push(audiotrack.location.clone());
                                queue_ids.push(audiotrack.id);
                            }
                        }

                        write_audio_backend_state = true;
                        audio_state.queue = Some(queue_ids.clone());
                    }
                    DBData::AudioBackendRecover(opt_history_audiotracks, opt_queue_audiotracks) => {
                        if let Some(history_audiotracks) = opt_history_audiotracks {
                            for audiotrack in history_audiotracks.iter() {
                                history.push(audiotrack.location.clone());
                                history_ids.push(audiotrack.id);
                            }
                            audio_state.history = Some(history_ids.clone());
                        }
                        if let Some(queue_audiotracks) = opt_queue_audiotracks {
                            for audiotrack in queue_audiotracks.iter() {
                                queue.push(audiotrack.location.clone());
                                queue_ids.push(audiotrack.id);
                            }
                            audio_state.queue = Some(queue_ids.clone());
                        }
                    }
                },
                Err(_) => {
                    //eprintln!("Error: {}", e);
                }
            }

            // Not required for every action, but for many
            if write_audio_backend_state {
                let audio_backend_state = AudioBackendState {
                    volume: volume,
                    position: mediaplayer.get_time().unwrap_or_default(),
                    shuffle_mode: shuffle,
                    repeat_mode: repeat.clone(),
                    current_id: current_id,
                    current_location: current_location.clone(),
                    queue: queue_ids.clone(),
                    history: history_ids.clone(),
                };
                set_audio_backend_state(
                    &audio_backend_state_name,
                    &audio_backend_state_path,
                    audio_backend_state,
                );
            }

            audio_state.shuffle_mode = Some(shuffle);
            audio_state.repeat_mode = Some(repeat.clone());
            audio_state.is_playing = Some(mediaplayer.is_playing());
            /*
            TODO:
            once events are unified this will not regularly update position in frontend.
            Frontend needs to run own timer and resync whenever new message comes in.
            On the other hand, unified events should be able to make audio backend more responsive.
             */
            audio_state.position = Some(mediaplayer.get_time().unwrap_or_default());

            // Workaround to make sure we recover old playback position
            if old_position != 0 {
                mediaplayer.set_time(old_position);
                audio_state.position = Some(old_position);
            }

            let _ = app.emit("audio_state", audio_state).unwrap();
        }
    });
}
