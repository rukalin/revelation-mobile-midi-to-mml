import 'package:midi_to_mml/messages/dart_to_rust.pb.dart';
import 'package:midi_to_mml/messages/types.pb.dart';

class SplitTrack {
	SplitTrack(int index) {
		SignalSplitTrackPayload(index: index).sendSignalToRust();
	}
}

class MergeTracks {
	MergeTracks(int indexA, int indexB) {
		SignalMergeTracksPayload(indexA: indexA, indexB: indexB).sendSignalToRust();
	}
}

class SaveSongSettings {
	SaveSongSettings(SignalMmlSongOptions songOptions) {
		SignalUpdateMmlSongOptionsPayload( songOptions: songOptions ).sendSignalToRust();
	}
}
