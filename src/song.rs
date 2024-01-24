use crate::{track::Track, utils};
use midly::{Smf, Timing};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongOptions {
    pub auto_boot_velocity: bool,
    pub velocity_min: u8,
    pub velocity_max: u8,
}

impl Default for SongOptions {
    fn default() -> Self {
        SongOptions {
            auto_boot_velocity: true,
            velocity_min: 0,
            velocity_max: 15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub ppq: u16,
    pub bpm: u16,
    pub tracks: Vec<Track>,
}

impl Song {
    pub fn from_path<P>(path: P, options: SongOptions) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => return Err(err),
        };

        Self::from_bytes(bytes, options)
    }

    pub fn from_bytes(bytes: Vec<u8>, options: SongOptions) -> Result<Self, Error> {
        let smf = match Smf::parse(&bytes) {
            Ok(smf) => smf,
            Err(err) => return Err(Error::new(ErrorKind::Other, err)),
        };

        let ppq = match Self::get_ppq_from_smf(&smf) {
            Some(ppq) => ppq,
            None => 480,
        };

        let mut bpm = 120u16;

        // Tracks
        let mut tracks: Vec<Track> = Vec::new();

        for smf_track in smf.tracks.iter() {
            let track = Track::new(
                smf_track,
                ppq,
                &mut bpm,
                options.velocity_min,
                options.velocity_max,
            );

            if track.notes.len() > 0 {
                tracks.push(track);
            }
        }

        // Merge track
        // if options.merge_track.len() > 0 {
        //     merge_track(&mut tracks, options.merge_track);
        // }

        // Split track
        // if options.is_split_track {
        //     split_track(&mut tracks);
        // }

        if options.auto_boot_velocity {
            modify_note_velocity(&mut tracks);
        }

        Ok(Self { ppq, bpm, tracks })
    }

    pub fn get_ppq_from_smf(smf: &Smf) -> Option<u16> {
        match smf.header.timing {
            Timing::Metrical(ppq) => Some(ppq.as_int()),
            _ => None,
        }
    }
}

fn modify_note_velocity(tracks: &mut Vec<Track>) {
    let mut max = 0u8;

    for track in tracks.iter() {
        let current_max = utils::get_highest_velocity(&track.notes);
        if current_max > max {
            max = current_max;
        }
    }

    let diff = 15 - max;

    for track in tracks.iter_mut() {
        track.modify_velocity(diff);
    }
}

// fn split_track(tracks: &mut Vec<Track>) {
//     if tracks.len() == 1 {
//         let (a, b) = tracks.first().unwrap().split();
//         *tracks = vec![a, b];
//     } else {
//         let mut longest_track_index = 0_usize;
//         let mut longest_note_length = 0_usize;
//
//         for (index, track) in tracks.iter().enumerate() {
//             let current_note_length = track.notes.len();
//             if current_note_length > longest_note_length {
//                 longest_track_index = index;
//                 longest_note_length = current_note_length;
//             }
//         }
//
//         let longest_track = tracks.get(longest_track_index).unwrap().to_owned();
//         let (a, b) = longest_track.split();
//         tracks.splice(longest_track_index..longest_track_index+1, [a, b]);
//     }
// }
//
// fn merge_track(tracks: &mut Vec<Track>, indexes: Vec<(usize, usize)>) {
//     let mut to_remove: Vec<usize> = Vec::new();
//
//     for index in indexes.iter() {
//         let mut track_b = tracks.get(index.1).unwrap().to_owned();
//         let track_a = tracks.get_mut(index.0).unwrap();
//
//         track_a.merge(&mut track_b);
//         to_remove.push(index.1);
//     }
//
//     for index in to_remove {
//         tracks.remove(index);
//     }
// }
