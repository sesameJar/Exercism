#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.iter().last() {
            Some(&n) => Some(n),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
            Some(&n) => Some(n),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v: Vec<u32> = self.scores().iter().copied().collect();
        v.sort();
        v.reverse();

        if v.len() > 3 {
            return v[..3].to_vec();
        }
        v
    }
}
