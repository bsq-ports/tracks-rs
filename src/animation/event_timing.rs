use crate::{easings::functions::Functions, time_types::SongTime};

/// When an event runs on the song clock and how its progress is eased.
///
/// Shared by [`super::coroutine_manager::CoroutineManager`], which steps events forward, and
/// [`super::timeline_coroutine_manager::TimelineCoroutineManager`], which evaluates them at any
/// time, so both agree on progress, easing and when an event has finished.
///
/// # Which method to use
/// | You are evaluating | Use |
/// | --- | --- |
/// | An `AnimateTrack` event for a single iteration, stepped by the live manager | [`Self::progress`] |
/// | An `AnimateTrack` event that may repeat, evaluated at any time | [`Self::repeated_progress`] |
/// | An `AssignPathAnimation` event | [`Self::path_blend`] (timeline) or [`Self::progress`] (live) |
/// | Deciding whether a coroutine is done and can be removed | [`Self::has_finished`] |
///
/// The examples below all use an event that starts at `10.0`s, lasts `2.0`s and eases linearly.
#[derive(Debug, Clone, Copy)]
pub(crate) struct EventTiming {
    pub start_song_time: SongTime,
    pub duration_song_time: SongTime,
    pub easing: Functions,
}

impl EventTiming {
    /// Seconds since the event started. Negative before it starts.
    ///
    /// ```text
    /// elapsed(9.0)  == -1.0   // before the start
    /// elapsed(11.0) ==  1.0
    /// ```
    pub fn elapsed(&self, song_time: SongTime) -> SongTime {
        song_time - self.start_song_time
    }

    /// Whether the duration has fully elapsed at `song_time`. Repeats are not considered.
    ///
    /// The live manager uses this to decide whether to keep a coroutine (`Yield`) or drop it (`Break`).
    ///
    /// ```text
    /// has_finished(11.9) == false
    /// has_finished(12.0) == true
    /// ```
    pub fn has_finished(&self, song_time: SongTime) -> bool {
        self.elapsed(song_time) >= self.duration_song_time
    }

    /// Eased progress through one iteration: `elapsed / duration` clamped to `[0, 1]`, then eased.
    /// A zero (or negative) duration is already complete.
    ///
    /// Use it for one iteration. The live manager restarts the start time itself for each repeat,
    /// so it can call this every time.
    ///
    /// ```text
    /// progress(9.0)  == 0.0   // before the start is clamped
    /// progress(11.0) == 0.5
    /// progress(15.0) == 1.0   // after the end is clamped
    /// ```
    pub fn progress(&self, song_time: SongTime) -> f32 {
        let duration = self.duration_song_time;
        let normalized = if duration <= SongTime::ZERO {
            1.0
        } else {
            (self.elapsed(song_time) / duration).clamp(0.0, 1.0) as f32
        };
        self.easing.interpolate(normalized)
    }

    /// Eased progress through the current iteration of an event that repeats `repeat` more times.
    /// Each repeat restarts at its iteration boundary, and the final value is held once every
    /// iteration has elapsed.
    ///
    /// Use it when you evaluate an arbitrary time without having stepped through the earlier
    /// iterations, as the timeline manager does. With `repeat = 1` the event runs twice, `10..12` and `12..14`:
    ///
    /// ```text
    /// repeated_progress(1, 11.0) == 0.5   // first iteration
    /// repeated_progress(1, 12.0) == 0.0   // second iteration restarts
    /// repeated_progress(1, 13.0) == 0.5   // second iteration
    /// repeated_progress(1, 14.0) == 1.0   // every iteration elapsed, the final value is held
    /// ```
    pub fn repeated_progress(&self, repeat: u32, song_time: SongTime) -> f32 {
        let duration = self.duration_song_time;
        if duration <= SongTime::ZERO {
            return 1.0;
        }

        let elapsed = self.elapsed(song_time);
        if elapsed >= duration * (repeat as f64 + 1.0) {
            return 1.0;
        }

        // this is faster than using a modulus and avoids floating point issues with very small durations
        let local = elapsed - duration * (elapsed / duration).floor();
        self.easing
            .interpolate((local / duration).clamp(0.0, 1.0) as f32)
    }

    /// Eased blend time of a path animation, or `None` once it has finished.
    ///
    /// `None` means the blend is over, so the previous path is dropped and only the new one is used.
    /// A zero duration never blends. Path animations ignore `repeat`.
    ///
    /// ```text
    /// path_blend(11.0) == Some(0.5)   // blending from the previous path to the new one
    /// path_blend(12.0) == None        // finished, use only the new path
    /// ```
    pub fn path_blend(&self, song_time: SongTime) -> Option<f32> {
        if self.duration_song_time <= SongTime::ZERO || self.has_finished(song_time) {
            return None;
        }

        Some(self.progress(song_time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The event used by the examples in the docs.
    fn timing() -> EventTiming {
        EventTiming {
            start_song_time: SongTime::new(10.0),
            duration_song_time: SongTime::new(2.0),
            easing: Functions::EaseLinear,
        }
    }

    fn at(t: f64) -> SongTime {
        SongTime::new(t)
    }

    #[test]
    fn doc_examples_hold() {
        let t = timing();
        assert_eq!(t.elapsed(at(9.0)), at(-1.0));
        assert_eq!(t.elapsed(at(11.0)), at(1.0));

        assert!(!t.has_finished(at(11.9)));
        assert!(t.has_finished(at(12.0)));

        assert_eq!(t.progress(at(9.0)), 0.0);
        assert_eq!(t.progress(at(11.0)), 0.5);
        assert_eq!(t.progress(at(15.0)), 1.0);

        assert_eq!(t.repeated_progress(1, at(11.0)), 0.5);
        assert_eq!(t.repeated_progress(1, at(12.0)), 0.0);
        assert_eq!(t.repeated_progress(1, at(13.0)), 0.5);
        assert_eq!(t.repeated_progress(1, at(14.0)), 1.0);

        assert_eq!(t.path_blend(at(11.0)), Some(0.5));
        assert_eq!(t.path_blend(at(12.0)), None);
    }
}
