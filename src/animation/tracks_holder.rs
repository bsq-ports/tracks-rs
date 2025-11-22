use slotmap::{SlotMap, new_key_type};

use crate::animation::track::Track;

new_key_type! { pub struct TrackKey; }
// #[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// #[repr(C)]
// pub struct TrackKey(usize);

// impl Key for TrackKey {
//     fn from_usize(u: usize) -> Self {
//         TrackKey
//     }

//     fn to_usize(&self) -> usize {
//         0
//     }
// }

#[derive(Clone, Default)]
pub struct TracksHolder {
    // Using SlotMap as it provides stable keys and efficient storage
    // very fast lookups vs HashMap and avoids fragmentation issues of Vec
    tracks: SlotMap<TrackKey, Track>,
}

impl TracksHolder {
    pub fn new() -> Self {
        Self {
            tracks: SlotMap::with_key(),
        }
    }

    pub fn add_track(&mut self, track: Track) -> TrackKey {
        if self.tracks.iter().any(|t| t.1.name == track.name) {
            // If the track already exists, we can just return it
            // This avoids unnecessary duplication of tracks
            panic!("Track with name '{}' already exists.", track.name);
        }

        self.tracks.insert(track)
    }

    pub fn get_track(&self, index: TrackKey) -> Option<&Track> {
        self.tracks.get(index)
    }
    pub fn get_track_mut(&mut self, index: TrackKey) -> Option<&mut Track> {
        self.tracks.get_mut(index)
    }

    pub fn get_track_key(&mut self, name: &str) -> Option<TrackKey> {
        self.tracks
            .iter_mut()
            .find(|t| t.1.name == name)
            .map(|t| t.0)
    }

    pub fn get_track_by_name(&self, name: &str) -> Option<&Track> {
        self.tracks.iter().find(|t| t.1.name == name).map(|t| t.1)
    }
}
