//! Strongly typed time units.
//!
//! Beatmaps describe durations in beats ([`BpmTime`]). Playback runs on the song clock in
//! seconds ([`SongTime`]). These newtypes keep the two units from being mixed by accident.
//! Converting between them always needs the map's BPM: see [`BpmTime::to_song_time`] and
//! [`SongTime::to_bpm_time`].
//!
//! Both types:
//! - wrap an `f64` and are `#[repr(transparent)]`, so they have the same layout as `f64`,
//! - dereference to the raw `f64` and convert from and into `f32` and `f64`,
//! - support adding and subtracting the same unit, and scaling by a plain number.
//!
//! Dividing one time by another of the same unit gives a unitless `f64` ratio, for example
//! `elapsed / duration` for animation progress.

use std::fmt;
use std::ops::{Add, AddAssign, Deref, Div, Mul, Sub, SubAssign};

/// A time or duration measured in beats, as written in the beatmap.
///
/// Convert it to seconds with [`BpmTime::to_song_time`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BpmTime(pub f64);

/// A time or duration measured in seconds on the song clock.
///
/// Convert it to beats with [`SongTime::to_bpm_time`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct SongTime(pub f64);

impl BpmTime {
    pub const ZERO: Self = Self(0.0);

    pub const fn new(beats: f64) -> Self {
        Self(beats)
    }

    /// The number of beats.
    pub const fn beats(self) -> f64 {
        self.0
    }

    /// Converts beats to seconds at the given BPM: `beats * 60 / bpm`.
    pub fn to_song_time(self, bpm: f64) -> SongTime {
        SongTime(self.0 * 60.0 / bpm)
    }
}

impl SongTime {
    pub const ZERO: Self = Self(0.0);

    pub const fn new(seconds: f64) -> Self {
        Self(seconds)
    }

    /// The number of seconds.
    pub const fn seconds(self) -> f64 {
        self.0
    }

    /// Converts seconds to beats at the given BPM: `seconds * bpm / 60`.
    pub fn to_bpm_time(self, bpm: f64) -> BpmTime {
        BpmTime(self.0 * bpm / 60.0)
    }
}

/// Implements conversions, `Deref`, arithmetic and `Display` for a time newtype.
macro_rules! impl_time_unit {
    ($ty:ident, $suffix:literal) => {
        impl Deref for $ty {
            type Target = f64;

            fn deref(&self) -> &f64 {
                &self.0
            }
        }

        impl From<f64> for $ty {
            fn from(value: f64) -> Self {
                Self(value)
            }
        }

        impl From<f32> for $ty {
            fn from(value: f32) -> Self {
                Self(value as f64)
            }
        }

        impl From<$ty> for f64 {
            fn from(value: $ty) -> Self {
                value.0
            }
        }

        /// Lossy: narrows to `f32`, the precision used by the FFI and interpolation.
        impl From<$ty> for f32 {
            fn from(value: $ty) -> Self {
                value.0 as f32
            }
        }

        impl Add for $ty {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl AddAssign for $ty {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl Sub for $ty {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl SubAssign for $ty {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        /// Scales the time, for example by a repeat count.
        impl Mul<f64> for $ty {
            type Output = Self;

            fn mul(self, rhs: f64) -> Self {
                Self(self.0 * rhs)
            }
        }

        impl Div<f64> for $ty {
            type Output = Self;

            fn div(self, rhs: f64) -> Self {
                Self(self.0 / rhs)
            }
        }

        /// The unitless ratio between two times of the same unit.
        impl Div for $ty {
            type Output = f64;

            fn div(self, rhs: Self) -> f64 {
                self.0 / rhs.0
            }
        }

        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}", self.0, $suffix)
            }
        }
    };
}

impl_time_unit!(BpmTime, " beats");
impl_time_unit!(SongTime, "s");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_between_beats_and_seconds() {
        let beats = BpmTime::new(2.0);
        let seconds = beats.to_song_time(120.0);
        assert_eq!(seconds, SongTime::new(1.0));
        assert_eq!(seconds.to_bpm_time(120.0), beats);
    }

    #[test]
    fn arithmetic_keeps_units() {
        let mut start = SongTime::new(1.0);
        start += SongTime::new(0.5);
        let elapsed = SongTime::new(2.0) - start;
        assert_eq!(elapsed, SongTime::new(0.5));
        assert_eq!(elapsed / SongTime::new(1.0), 0.5);
        assert_eq!(SongTime::new(1.5) * 2.0, SongTime::new(3.0));
        assert_eq!(*elapsed, 0.5);
        assert_eq!(f32::from(elapsed), 0.5);
    }
}
