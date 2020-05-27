use clap::Clap;

#[derive(Clap, Debug)]
#[clap(author = "Jonathan Gilchrist")]
pub struct Config {
    pub image_file: String,

    #[clap(long)]
    pub generation_size: u64,

    #[clap(long)]
    pub selection_size: u64,

    #[clap(long)]
    pub add_chance: f64,
    #[clap(long)]
    pub remove_chance: f64,
    #[clap(long)]
    pub move_chance: f64,
    #[clap(long)]
    pub alter_color_chance: f64,

    #[clap(long)]
    pub max_move_amount: i64,

    #[clap(long)]
    pub max_radius: u64,
}
