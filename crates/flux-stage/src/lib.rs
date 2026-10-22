//! Flux stage layer (skeleton).

/// Cue event broadcast to lighting / visuals.
#[derive(Debug, Clone, PartialEq)]
pub struct Cue {
    pub name: String,
    pub bpm: f32,
    pub bar: u32,
}

/// OSC-style address for a cue.
pub fn osc_address(c: &Cue) -> String {
    format!("/flux/cue/{}", c.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn address() {
        let c = Cue { name: "drop".into(), bpm: 128.0, bar: 33 };
        assert_eq!(osc_address(&c), "/flux/cue/drop");
    }
}
