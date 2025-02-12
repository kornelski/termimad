use std::fmt;

// See https://en.wikipedia.org/wiki/Block_Elements for more fun
static CHARS: [char; 8] = ['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];

/// A pixel precise horizontal bar
pub struct ProgressBar {
    pub part: f32,
    pub chars_len: usize,
}

impl ProgressBar {
    ///
    /// `part` must be in `[0,1]`.
    /// `chars_len` is the max width of the bar in characters
    pub fn new(part: f32, chars_len: usize) -> Self {
        Self {part, chars_len}
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        let mp = (self.chars_len as f32) * self.part;
        let full = mp as usize;
        for _ in 0..full {
            s.push(CHARS[7]);
        }
        let remain = (mp.fract() * 8.0).round() as usize;
        if remain>0 {
            s.push(CHARS[remain-1]);
        }
        s
    }
}

impl fmt::Display for ProgressBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.to_string())
    }
}
