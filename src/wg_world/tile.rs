#[derive(Debug, Clone, Deserialize)]
pub struct Tile {
    pub name: String,
    pub display_name: String,
    pub glyph: String,
    pub fg_color: Vec<u8>,
    pub bg_color: Vec<u8>,
    pub weight: u8,
}
