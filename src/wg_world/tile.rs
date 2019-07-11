#[derive(Debug, Clone, Deserialize)]
pub struct TileMeta {
    pub movement_cost: f32,
    pub blocks_sight: bool,
    pub weight: u32,
}

impl Default for TileMeta {
    fn default() -> Self {
        TileMeta {
            movement_cost: 1.0,
            blocks_sight: false,
            weight: 999999,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tile {
    pub name: String,
    pub display_name: String,
    pub glyph: String,
    pub color: Vec<u8>,
    pub meta: TileMeta,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            name: "".into(),
            display_name: "".into(),
            glyph: " ".into(),
            color: vec![0, 0, 0],
            meta: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SoilMeta {
    pub weight: u32,
}

impl Default for SoilMeta {
    fn default() -> SoilMeta {
        SoilMeta { weight: 999999 }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Soil {
    pub name: String,
    pub display_name: String,
    pub color: Vec<u8>,
    pub meta: SoilMeta,
}

impl Default for Soil {
    fn default() -> Soil {
        Soil {
            name: "".into(),
            display_name: "".into(),
            color: vec![0, 0, 0],
            meta: Default::default(),
        }
    }
}
