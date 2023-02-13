#[derive(PartialEq, Clone)]
pub struct LineStyle {
    pub color: &'static str,
    pub width: f64,
}

pub static ACTIVE_TILE_BG_COLOR: &str = "lightyellow";
pub static ACTIVE_TILE_LINE: LineStyle = LineStyle {
    color: "red",
    width: 3.0,
};
pub static GLAY_OUT_TILE_BG_COLOR: &str = "gray";
