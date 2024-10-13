use rand::seq::SliceRandom; // TODO: best way to test this?

const WELCOME_PHRASES: [&str; 2] = ["Siema!", "Dzień dobry!"];

const EMOJIS: [&str; 9] = [
    ":partyparrot:",
    ":fastparrot:",
    ":discoparrot:",
    ":congaparrot:",
    ":parrotdad:",
    ":nyanparrot:",
    ":dealwithitparrot:",
    ":headbanging_parrot:",
    ":crab:",
];

pub fn greet() -> String {
    // TODO: There should be a better way to handle Option here
    let welcome: &str = WELCOME_PHRASES
        .choose(&mut rand::thread_rng())
        .map_or_else(|| WELCOME_PHRASES[0], |s| s);
    let emoji: &str = EMOJIS
        .choose(&mut rand::thread_rng())
        .map_or_else(|| EMOJIS[0], |s| s);

    format!("{welcome} {emoji}")
}
