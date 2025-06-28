use clap::{ArgAction, Parser};
use std::env;
use std::error::Error;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Seed the Dream API database with random data")]
struct Args {
    #[clap(short, long, default_value = "100")]
    count: u32,

    #[clap(long)]
    database_url: Option<String>,

    #[clap(short, long, action=ArgAction::SetTrue)]
    verbose: bool,
}

struct DreamData {
    text: String,
    interpretation: String,
    tags: Vec<String>,
}


fn generate_dream_texts() -> Vec<&'static str> {
    vec![
        "I was flying over a vast ocean with crystal clear water. The sun was setting and the sky was painted in brilliant oranges and purples.",
        "I found myself in a library with infinite shelves stretching up to the clouds. Books were floating around me like butterflies.",
        "I was walking through a forest made entirely of candy. The trees were lollipops and the ground was made of chocolate.",
        "I was in a city where all the buildings were upside down. People were walking on the ceilings and cars drove through the air.",
        "I was swimming in a pool of liquid starlight. Each stroke made ripples that turned into constellations.",
        "I was in a garden where flowers sang beautiful melodies. Each flower had a different voice and they harmonized together.",
        "I was riding a giant turtle through space. We passed by planets that looked like marbles and moons made of cheese.",
        "I was in a house where every room was a different season. The kitchen was spring, the bedroom was winter.",
        "I was painting with colors that don't exist in the real world. They were more vivid than anything I've ever seen.",
        "I was in a desert where the sand was made of tiny mirrors. My reflection was scattered across the landscape.",
        "I was climbing a mountain that reached into the clouds. At the top, I could see the entire world spread out below.",
        "I was in a cave filled with glowing crystals. Each crystal made a different musical note when touched.",
        "I was sailing on a ship made of clouds through a sky ocean. Fish were swimming in the air around us.",
        "I was in a maze made of books. The walls were shelves and I had to read my way through to find the exit.",
        "I was dancing with shadows that had lives of their own. They moved independently and taught me new dance steps.",
        "I was in a school where the teachers were wise old owls and the students were various woodland creatures.",
        "I was flying on the back of a dragon made of pure light. We soared through rainbow clouds and over silver mountains.",
        "I was in a bakery where the bread was baked with dreams. Each loaf contained a different person's aspirations.",
        "I was walking through a museum where the paintings came to life and told me their stories.",
        "I was in a laboratory where I could bottle emotions and mix them like potions to create new feelings.",
    ]
}

fn generate_interpretations() -> Vec<&'static str> {
    vec![
        "This dream represents your desire for freedom and escape from daily constraints. Flying symbolizes liberation from earthly concerns.",
        "The infinite library suggests your thirst for knowledge and continuous learning. Books as butterflies indicate knowledge that's alive and transformative.",
        "The candy forest represents your inner child's desire for sweetness and joy in life. It may indicate a need for more pleasure and fun.",
        "The upside-down city reflects feelings of disorientation or seeing the world from a different perspective. It suggests adaptability to change.",
        "Swimming in starlight symbolizes your connection to the universe and higher consciousness. It represents spiritual awakening and enlightenment.",
        "The singing flowers represent harmony in your life and the beauty you find in nature. It suggests you're in tune with your environment.",
        "Riding a turtle through space indicates patience in your spiritual journey. The turtle represents wisdom and the slow but steady path to enlightenment.",
        "Different seasons in rooms represent the various phases of your life and emotions. It shows your ability to adapt to different circumstances.",
        "Painting with impossible colors suggests your creative potential is unlimited. You may be discovering new aspects of your artistic abilities.",
        "The mirror sand desert reflects self-reflection and introspection. Your scattered reflection shows different aspects of your personality.",
        "Climbing to see the world represents your ambition and desire for a broader perspective on life. You're seeking higher understanding.",
        "Musical crystals in the cave represent hidden talents and the harmony you can create in your life. Caves symbolize the unconscious mind.",
        "Cloud ships and flying fish represent your imagination's boundless nature. You're navigating through your thoughts and dreams with ease.",
        "The book maze suggests you're searching for knowledge to solve a problem. Reading your way out shows learning as your path to solutions.",
        "Dancing with independent shadows represents your relationship with your unconscious mind. You're learning from hidden aspects of yourself.",
        "Animal teachers represent wisdom coming from unexpected sources. You're open to learning from nature and instinct rather than just intellect.",
        "The light dragon symbolizes your spiritual guide and transformation. Light represents enlightenment and positive energy in your life.",
        "Dream bread represents nourishment for the soul. You may be seeking to understand others' aspirations and dreams more deeply.",
        "Living paintings telling stories suggests your memories and experiences are becoming more vivid and meaningful. Art represents self-expression.",
        "Bottling emotions in a laboratory represents your analytical approach to understanding feelings. You may be learning to process emotions scientifically.",
    ]
}

fn generate_tags() -> Vec<&'static str> {
    vec![
        "flying", "freedom", "ocean", "sunset", "adventure", "nature", "peaceful", "colorful",
        "library", "books", "knowledge", "infinite", "learning", "wisdom", "floating", "magical",
        "candy", "forest", "sweet", "childhood", "joy", "fantasy", "playful", "innocent",
        "city", "upside-down", "perspective", "strange", "adaptation", "surreal", "confusion", "change",
        "starlight", "swimming", "cosmos", "spiritual", "enlightenment", "universe", "transcendent", "mystical",
        "flowers", "singing", "harmony", "garden", "music", "beauty", "nature", "melody",
        "turtle", "space", "journey", "patience", "wisdom", "travel", "cosmic", "steady",
        "house", "seasons", "rooms", "transformation", "cycle", "home", "change", "adaptation",
        "painting", "colors", "creativity", "art", "expression", "vivid", "imagination", "artistic",
        "desert", "mirrors", "reflection", "self", "identity", "scattered", "introspection", "contemplation",
        "mountain", "climbing", "achievement", "perspective", "height", "challenge", "goal", "success",
        "cave", "crystals", "music", "hidden", "discovery", "underground", "mystery", "treasure",
        "ship", "clouds", "sky", "ocean", "sailing", "navigation", "journey", "exploration",
        "maze", "books", "puzzle", "solution", "reading", "knowledge", "challenge", "learning",
        "dancing", "shadows", "movement", "unconscious", "rhythm", "grace", "expression", "fluid",
        "school", "animals", "teaching", "learning", "nature", "wisdom", "education", "growth",
        "dragon", "light", "flight", "power", "spiritual", "guide", "strength", "transformation",
        "bakery", "bread", "dreams", "nourishment", "sustenance", "hope", "aspiration", "comfort",
        "museum", "paintings", "stories", "art", "history", "culture", "memory", "narrative",
        "laboratory", "emotions", "science", "analysis", "experiment", "understanding", "research", "discovery",
    ]
}

fn generate_random_dream() -> DreamData {
    use std::collections::HashSet;
    
    let dream_texts = generate_dream_texts();
    let interpretations = generate_interpretations();
    let all_tags = generate_tags();
    
    let text = dream_texts[fastrand::usize(..dream_texts.len())].to_string();
    let interpretation = interpretations[fastrand::usize(..interpretations.len())].to_string();
    
    // Generate 2-5 random unique tags
    let num_tags = fastrand::usize(2..=5);
    let mut selected_tags = HashSet::new();
    
    while selected_tags.len() < num_tags {
        let tag = all_tags[fastrand::usize(..all_tags.len())];
        selected_tags.insert(tag);
    }
    
    let tags: Vec<String> = selected_tags.into_iter().map(|s| s.to_string()).collect();
    
    DreamData {
        text,
        interpretation,
        tags,
    }
}

async fn insert_seed_data(db_url: &str, count: u32, verbose: bool) -> Result<(), Box<dyn Error>> {
    let (client, connection) = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    println!("🌱 Inserting {} random dream records...", count);
    
    for i in 0..count {
        let dream = generate_random_dream();
        let dream_id = Uuid::new_v4();
        
        // Insert dream
        let insert_dream_sql = format!(
            "INSERT INTO dreams (id, text, interpretation) VALUES ('{}', '{}', '{}')",
            dream_id,
            dream.text.replace("'", "''"), // Escape single quotes
            dream.interpretation.replace("'", "''")
        );
        
        if let Err(e) = client.simple_query(&insert_dream_sql).await {
            eprintln!("Error inserting dream {}: {}", i + 1, e);
            continue;
        }
        
        // Insert tags
        for tag in dream.tags {
            let insert_tag_sql = format!(
                "INSERT INTO dream_tags (dream_id, tag) VALUES ('{}', '{}')",
                dream_id,
                tag.replace("'", "''")
            );
            
            if let Err(e) = client.simple_query(&insert_tag_sql).await {
                eprintln!("Error inserting tag '{}' for dream {}: {}", tag, i + 1, e);
            }
        }
        
        if verbose && (i + 1) % 10 == 0 {
            println!("  Inserted {} dreams...", i + 1);
        }
    }
    
    println!("✅ Successfully inserted {} dreams with tags", count);
    Ok(())
}

async fn get_table_counts(db_url: &str) -> Result<(), Box<dyn Error>> {
    println!("\n📊 Final table status:");

    let (client, connection) = tokio_postgres::connect(db_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Check dreams table
    let result = client.simple_query("SELECT COUNT(*) FROM dreams").await;
    match result {
        Ok(rows) => {
            if let Some(tokio_postgres::SimpleQueryMessage::Row(row)) = rows.first() {
                if let Some(count_str) = row.get(0) {
                    println!("  Dreams: {} records", count_str);
                }
            }
        }
        Err(e) => println!("  Dreams: Error counting - {}", e),
    }

    // Check dream_tags table
    let result = client.simple_query("SELECT COUNT(*) FROM dream_tags").await;
    match result {
        Ok(rows) => {
            if let Some(tokio_postgres::SimpleQueryMessage::Row(row)) = rows.first() {
                if let Some(count_str) = row.get(0) {
                    println!("  Dream Tags: {} records", count_str);
                }
            }
        }
        Err(e) => println!("  Dream Tags: Error counting - {}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file if it exists
    let _ = dotenvy::dotenv();
    
    let args = Args::parse();

    // Get database URL from command line or environment
    let database_url = args.database_url
        .or_else(|| env::var("DATABASE_URL").ok())
        .expect("DATABASE_URL must be provided either as --database-url argument or DATABASE_URL environment variable");

    println!("🚀 Dream API Database Seeder");
    println!("Connecting to database...");

    insert_seed_data(&database_url, args.count, args.verbose).await?;
    
    get_table_counts(&database_url).await?;

    println!("\n🎉 Database seeding complete!");

    Ok(())
}