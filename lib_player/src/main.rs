use std::path::PathBuf;
use lib_player::{MmlPlayer, MmlPlayerOptions};
use revelation_mobile_midi_to_mml::{Song, SongOptions};

fn main() {
    test_from_midi();
}

fn test_from_midi() {
    // let path = "../test_resources/midi/Hitchcock.mid"; // Grand piano
    // let path = "../test_resources/midi/Milonga.mid"; // Nylon guitar
    // let path = "../test_resources/midi/Lost-one no Gokoku.mid"; // Elictric guitar
    // let path = "../test_resources/midi/Sir_Duke_Bass_Guitar.mid"; // Bass
    let path = "../test_resources/midi/Kirameki_Piano_and_Violin_Duet.mid"; // Violin
    // let path = "../test_resources/midi/グッバイ宣言.mid"; // Drumset
    // let path = "../test_resources/midi/always_with_me_flute.mid"; // Flute
    let midi_path = PathBuf::from(path[1..].to_string());


    let song = Song::from_path(midi_path, SongOptions {
        auto_boot_velocity: false,
        velocity_min: 8,
        ..Default::default()
    }).unwrap();

    let player = MmlPlayer::from_song(&song, MmlPlayerOptions {
        soundfont_path: vec![
            // PathBuf::from("./test_resouces/soundfonts/gm.sf2"), // General MIDI, very light
            // PathBuf::from("/home/cuikho210/Documents/soundfonts/Monalisa GM v2_06_5.sf2"), // General MIDI, about 1.7GiB
            PathBuf::from("./test_resources/soundfonts/tx16w_GM_1.0.sf2"), // General MIDI, medium

            PathBuf::from("./test_resources/soundfonts/AMS_Grand_Piano_-_Remastered.sf2"), // Acoustic grand piano only
            PathBuf::from("./test_resources/soundfonts/megalovania_drums.sf2"), // Percussions
            PathBuf::from("./test_resources/soundfonts/Red_Pilled_Based_Gui.sf2"), // Guitars
            PathBuf::from("./test_resources/soundfonts/Valiant_Violin_V2.sf2"), // Strings
        ],
    });

    player.play();
}
