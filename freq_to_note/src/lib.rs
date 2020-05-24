const A4_TUNING_HZ: f32 = 440.0;

/// 1-indexed
const A4_KEY_POSITION: u16 = 49;

/// See https://en.m.wikipedia.org/wiki/Piano_key_frequencies
fn a_freq_to_note(freq: f32) -> u16 {
    ((39.86 * (freq / A4_TUNING_HZ).log10()) as u16 + A4_KEY_POSITION) as u16
}

fn freq_to_note(freq: f32) -> u16 {
    (12.0 * (freq / A4_TUNING_HZ).log2()) as u16 + A4_KEY_POSITION
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_freq_to_note() {
        assert_eq!(A4_KEY_POSITION, freq_to_note(A4_TUNING_HZ));
        assert_eq!(103, freq_to_note(5919.911)); // F#8
        assert_eq!(26, freq_to_note(116.5409)); // Bb2
    }
}
