extern crate clap;
extern crate image;
extern crate rand;

mod config;
mod drawing;
mod genetic;
mod rendering;

use clap::Clap;
use config::Config;
use drawing::{ImageDescription, mutate};
use rand::prelude::SliceRandom;

fn main() {
    let config: Config = Config::parse();
    let input_image = image::open(&config.image_file).unwrap().to_rgb();
    let (width, height) = input_image.dimensions();
    println!("Image loaded: {}x{}", width, height);

    let initial_generation: Vec<ImageDescription> = (0..config.generation_size).map(move |_| ImageDescription {
        width: width.into(),
        height: height.into(),
        items: vec![],
    }).collect();
    let initial_error = genetic::fitness(&input_image, &initial_generation[0]);

    let mut current_generation = initial_generation;
    let mut current_best_error = initial_error;

    let mut rng = rand::thread_rng();

    let mut i = 0;
    loop {
        // Calculate fitness for current generation
        let mut fitness_per_individual: Vec<(ImageDescription, u64)> = current_generation.into_iter()
            .map(|i| (i.clone(), genetic::fitness(&input_image, &i)))
            .collect();

        fitness_per_individual.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

        let (current_best_individual, current_best_individual_error) = fitness_per_individual[0].clone();
        current_best_error = current_best_individual_error;

        // Select n best individuals
        let best_individuals: Vec<(ImageDescription, u64)> = fitness_per_individual
            .into_iter()
            .take(config.selection_size as usize)
            .collect();

        // Generate a new population from the fittest individuals
        let new_candidates: Vec<ImageDescription> = (0..config.generation_size).map(|_| {
            let (random_individual, _) = best_individuals.choose(&mut rng).expect("Tried to pick an individual from an empty generation");
            let mutated_individual = mutate(&random_individual, &config);
            mutated_individual
        }).collect();

        println!("Generation #{}: Best error = {}", i, current_best_error);

        current_generation = new_candidates;

        let img = rendering::draw_image(&current_best_individual);
        img.save(format!("generated/{}.png", i)).unwrap();
        i += 1
    }

    // let final_image = rendering::draw_image(&image_desc);
    // final_image.save("generated/after.png").unwrap();
}
