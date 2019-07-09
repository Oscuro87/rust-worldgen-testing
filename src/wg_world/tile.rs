#[derive(Debug, Clone, Deserialize)]
pub struct TileMeta {
    pub movement_cost: f32,
    pub blocks_sight: bool,
    pub weight: u32,
    pub cluster_chance: u32,
    pub cluster_size_min: u32,
    pub cluster_size_max: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tile {
    pub name: String,
    pub display_name: String,
    pub glyph: String,
    pub fg_color: Vec<u8>,
    pub bg_color: Vec<u8>,
    pub meta: TileMeta,
}