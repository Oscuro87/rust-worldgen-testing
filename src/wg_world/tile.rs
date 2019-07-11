#[derive(Debug, Clone, Deserialize)]
pub struct TileMeta {
    pub movement_cost: f32,
    pub blocks_sight: bool,
    pub weight: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tile {
    pub name: String,
    pub display_name: String,
    pub glyph: String,
    pub color: Vec<u8>,
    pub meta: TileMeta,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SoilMeta {
    pub weight: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Soil {
    pub name: String,
    pub display_name: String,
    pub color: Vec<u8>,
    pub meta: SoilMeta,
}
