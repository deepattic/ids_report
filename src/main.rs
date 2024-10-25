use csv::Writer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct ChampionInfo {
    attack: i32,
    defense: i32,
    magic: i32,
    difficulty: i32,
}

#[derive(Debug, Deserialize)]
struct ChampionStats {
    hp: f32,
    hpperlevel: f32,
    mp: f32,
    mpperlevel: f32,
    movespeed: i32,
    armor: f32,
    armorperlevel: f32,
    spellblock: f32,
    spellblockperlevel: f32,
    attackrange: i32,
    hpregen: f32,
    hpregenperlevel: f32,
    mpregen: f32,
    mpregenperlevel: f32,
    crit: f32,
    critperlevel: f32,
    attackdamage: f32,
    attackdamageperlevel: f32,
    attackspeedperlevel: f32,
    attackspeed: f32,
}

#[derive(Debug, Deserialize)]
struct Champion {
    version: String,
    id: String,
    key: String,
    name: String,
    title: String,
    blurb: String,
    info: ChampionInfo,
    tags: Vec<String>,
    partype: String,
    stats: ChampionStats,
}

#[derive(Debug, Serialize)]
struct ChampionRow {
    name: String,
    version: String,
    key: String,
    title: String,
    tags: String,
    attack: i32,
    defense: i32,
    magic: i32,
    difficulty: i32,
    hp: f32,
    mp: f32,
    movespeed: i32,
    armor: f32,
    spellblock: f32,
    attackrange: i32,
    hpregen: f32,
    mpregen: f32,
    attackdamage: f32,
    attackspeed: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the JSON file
    let file = File::open("indi.json")?;
    let champions: HashMap<String, Champion> = serde_json::from_reader(file)?;

    // Create a new CSV file
    let mut wtr = Writer::from_path("champions.csv")?;

    // Write data for each champion
    for (_, champion) in champions {
        let row = ChampionRow {
            name: champion.name,
            version: champion.version,
            key: champion.key,
            title: champion.title,
            tags: champion.tags.join(", "),
            attack: champion.info.attack,
            defense: champion.info.defense,
            magic: champion.info.magic,
            difficulty: champion.info.difficulty,
            hp: champion.stats.hp,
            mp: champion.stats.mp,
            movespeed: champion.stats.movespeed,
            armor: champion.stats.armor,
            spellblock: champion.stats.spellblock,
            attackrange: champion.stats.attackrange,
            hpregen: champion.stats.hpregen,
            mpregen: champion.stats.mpregen,
            attackdamage: champion.stats.attackdamage,
            attackspeed: champion.stats.attackspeed,
        };

        wtr.serialize(row)?;
    }

    wtr.flush()?;
    println!("CSV file has been created successfully!");

    Ok(())
}
