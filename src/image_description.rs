use drawing::ColoredPolygon;

pub struct ImageDescription {
    pub width: u32,
    pub height: u32,
    pub polygons: Vec<ColoredPolygon>
}
