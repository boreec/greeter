use crate::random::Rng;

static SENTENCES: &[&str] = &[
    "Don't work too hard. The sun will expand and engulf this CPU anyway.",
    "Everything you do today will eventually be overwritten.",
    "Nothing matters. Build good software anyway.",
    "That is all.",
    "The loop continues.",
    "The universe has not noticed.",
    "Try not to take it too seriously.",
];

pub fn welcome_message() -> &'static str {
    let mut rng = Rng::new();

    let index = rng.gen_range(SENTENCES.len() as u32) as usize;

    SENTENCES
        .get(index)
        .copied()
        .unwrap_or("The void is silent.")
}
