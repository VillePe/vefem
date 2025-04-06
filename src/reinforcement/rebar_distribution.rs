use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum RebarDistribution {
    /// Even rebar distribution. To calculate the real positions, the profile values need to be known.
    /// If there is only one rebar, the rebar will be placed with cc_left only and ignoring the cc_right.
    Even{diam: f64, count: isize, cc_start: String, cc_end: String},
    /// Rebar distribution by given spacing value. If the spacing doesn't fit exactly, 
    /// the spacing will be adjusted to fit between to the distance of L-cc_start-cc_end with
    /// 'full' rebar count.
    /// e.g. L=4000, diam=10, cc_start=95, cc_end=95, spacing=300 => 
    /// Count = ceil((4000-95-95-10)/300) = 13 (note that this is the count of spacings, the count of rebars is one more)
    /// Spacing = (4000-95-95-10)/count = 292,31 mm
    Spacing {diam: f64, spacing: f64, cc_start: String, cc_end: String},
    /// Distributed rebar by a distribution string. The first rebar is the first value of the
    /// distribution string. Distribution spaces are separated by a space and multipliers can
    /// be used by using a '*' character (e.g. 30 5*30 60)
    Distributed{diam: f64, distr: String},    
    /// Single rebar at a specific position. The offsets are to the center of the rebar
    Single{diam: f64, off_left: String, off_bot: String},
    /// No real distribution used, only the full area of the reinforcement (not suggested to be used unless testing)
    ByArea{area: f64, sec_mom_of_area: f64},
}