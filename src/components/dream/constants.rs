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
    "Transcending dimensions",
    "Cosmic exploration",
    "Ancient wisdom",
    "Discovering abilities",
    "Finding a new realm",
    "Water as consciousness",
    "Spiritual rebirth",
    "Universal connection",
    "Dragons",
    "Abundance",
    "Meeting guides",
    "Alternate timelines",
    "Shapeshifting",
    "Astral projection",
    "Cosmic library",
    "Reunited with soul family",
    "Time travel",
    "Mythical creatures",
    "Ancestral memories",
    "Phoenix transformation",
];

// Theme sentence templates
pub fn get_theme_sentence(theme: &str) -> &'static str {
    match theme {
        "Flying" => "I was soaring through luminous clouds, merging with the infinite sky.",
        "Transcending dimensions" => "I moved effortlessly between dimensions, witnessing realities beyond physical limitations.",
        "Cosmic exploration" => "I journeyed through vast cosmic landscapes, communing with celestial energies.",
        "Ancient wisdom" => "Ancient knowledge flowed through me as I connected with timeless wisdom keepers.",
        "Discovering abilities" => "I discovered extraordinary abilities within myself that revealed my unlimited potential.",
        "Finding a new realm" => "I discovered a hidden realm that seemed to exist between thoughts and reflected my inner universe.",
        "Water as consciousness" => "I became one with an infinite ocean that represented the collective consciousness.",
        "Spiritual rebirth" => "I experienced a profound rebirth that transformed my understanding of existence.",
        "Universal connection" => "I felt intimately connected to all beings in the universe, experiencing oneness.",
        "Dragons" => "Majestic dragons served as guides, revealing ancient truths through their wisdom.",
        "Abundance" => "Streams of golden light flowed through my hands, manifesting whatever I envisioned.",
        "Meeting guides" => "Luminous beings of light appeared as mentors, sharing profound universal insights.",
        "Alternate timelines" => "I witnessed parallel versions of my life unfolding across multiple timelines.",
        "Shapeshifting" => "I shifted between different forms, experiencing existence through multiple perspectives.",
        "Astral projection" => "My consciousness expanded beyond my body, allowing me to explore the cosmos freely.",
        "Cosmic library" => "I entered a vast cosmic library containing the knowledge of all civilizations.",
        "Reunited with soul family" => "I reunited with beings who felt deeply familiar, as if we had known each other across many lifetimes.",
        "Time travel" => "I moved fluidly through the river of time, witnessing the interconnection of all events.",
        "Mythical creatures" => "Mythical beings revealed themselves as embodiments of cosmic forces and universal archetypes.",
        "Ancestral memories" => "I accessed memories from my ancestral lineage, experiencing their wisdom and insights.",
        "Phoenix transformation" => "I underwent a magnificent transformation, emerging renewed from sacred flames.",
        _ => match theme.to_lowercase().as_str() {
            "dogs" | "dog" => "A wise canine companion appeared as a spirit guide, offering profound insights.",
            "cats" | "cat" => "A feline being with ancient eyes revealed secrets from other realms.",
            "ocean" | "sea" => "The vast cosmic ocean revealed the fluid nature of consciousness and reality.",
            "mountains" | "mountain" => "Crystalline mountains resonated with frequencies that awakened dormant knowledge within me.",
            "family" | "parents" => "Ancestral guides appeared, showing me the golden threads connecting our souls across time.",
            "work" | "job" => "I discovered my cosmic purpose and how it weaves into the greater tapestry of existence.",
            "baby" | "child" => "A radiant child of stars showed me the eternal, innocent wonder at the core of being.",
            _ => "My dream revealed this theme as a gateway to profound universal understanding.",
        },
    }
}
