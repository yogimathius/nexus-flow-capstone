// Dream interpretation tips
pub const DREAM_TIPS: &[&str] = &[
    "Record your dreams immediately upon waking for the most accurate recall.",
    "Keep a dream journal by your bed to capture details before they fade.",
    "Pay attention to emotions felt during the dream - they often hold the key to interpretation.",
    "Look for recurring symbols or themes across multiple dreams.",
    "Consider the context of your waking life when interpreting dream symbols.",
    "Dreams often process unresolved issues from your day or deeper subconscious concerns.",
    "Practice reality checks during the day to increase dream awareness and recall.",
    "Some dreams may be metaphorical rather than literal - look for deeper meanings.",
];

// Common dream themes
pub const COMMON_DREAM_THEMES: &[&str] = &[
    "Flying",
    "Falling",
    "Being chased",
    "Teeth falling out",
    "Being unprepared",
    "Finding a new room",
    "Water",
    "Death or dying",
    "Being trapped",
    "Snakes",
    "Money",
    "Celebrity",
    "School or work",
    "Naked in public",
    "Unable to move",
    "Lost",
    "Reunited",
    "Time travel",
    "Animals",
    "Childhood home",
    "Fire",
];

// Theme sentence templates
pub fn get_theme_sentence(theme: &str) -> &'static str {
    match theme {
        "Flying" => "I was flying over a landscape and felt a sense of freedom.",
        "Falling" => "I experienced falling from a great height and felt scared.",
        "Being chased" => "Something or someone was chasing me and I couldn't escape.",
        "Teeth falling out" => "My teeth started to crumble and fall out unexpectedly.",
        "Being unprepared" => "I found myself in a situation where I was completely unprepared.",
        "Finding a new room" => "I discovered a new room in a familiar building that I never knew existed.",
        "Water" => "There was a large body of water that played an important role in my dream.",
        "Death or dying" => "The dream involved death or dying in some way that felt significant.",
        "Being trapped" => "I felt trapped in a confined space with no way to escape.",
        "Snakes" => "There were snakes in my dream that seemed to have some special meaning.",
        "Money" => "Money played an important role in my dream, affecting the events that unfolded.",
        "Celebrity" => "A celebrity appeared in my dream and interacted with me directly.",
        "School or work" => "I was back at school or work, but things were different than in real life.",
        "Naked in public" => "I realized I was naked or underdressed in a public place and felt embarrassed.",
        "Unable to move" => "I tried to move but my body wouldn't respond, leaving me paralyzed.",
        "Lost" => "I was lost in an unfamiliar place and couldn't find my way back.",
        "Reunited" => "I was reunited with someone I haven't seen in a long time.",
        "Time travel" => "I traveled through time and experienced events from a different era.",
        "Animals" => "Animals played a significant role in my dream with unusual behaviors.",
        "Childhood home" => "I was back in my childhood home, noticing details I had forgotten.",
        "Fire" => "There was a fire that both frightened and fascinated me in the dream.",
        _ => match theme.to_lowercase().as_str() {
            "dogs" | "dog" => "I saw a dog in my dream and it seemed to represent something important.",
            "cats" | "cat" => "A cat appeared in my dream and its behavior seemed symbolic.",
            "ocean" | "sea" => "I was near the ocean, and the waves seemed to reflect my emotions.",
            "mountains" | "mountain" => "I was in the mountains, feeling both small and powerful.",
            "family" | "parents" => "My family appeared in my dream in an unusual context.",
            "work" | "job" => "I was at work in my dream, but things were different than reality.",
            "baby" | "child" => "There was a baby or child in my dream that seemed significant.",
            _ => "My dream featured this theme in a way that felt meaningful.",
        },
    }
} 