use clap::Clap;

#[derive(Clap, Debug)]
#[clap(author = "Jonathan Gilchrist")]
pub struct Config {
    pub image_file: String,

    #[clap(long)]
    pub population_size: u32,

    #[clap(long)]
    pub add_polygon_chance: f32,
    #[clap(long)]
    pub remove_polygon_chance: f32,
    #[clap(long)]
    pub move_polygon_chance: f32,
    #[clap(long)]
    pub alter_polygon_color_chance: f32,

    #[clap(long)]
    pub max_move_amount: f32,
}
