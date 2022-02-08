use clap::Parser;
use genetic_algorithm::{
    multifactor::{GeneticParameters, VectorFitnessEvaluater, VectorGeneticFactory},
    GeneticFactory, GeneticProcessor,
};

#[derive(Parser)]
#[clap(name = "LR1")]
pub struct CLIParameters {
    /// Radius for searching
    #[clap(long, default_value = "0.5")]
    pub search_radius: f64,
    /// Radius that allows two chromosomes to cross
    #[clap(long, default_value = "0.25")]
    pub cross_allow_radius: f64,
    /// Max try count for search chromosome in cross_allow_radius
    #[clap(long, default_value = "5")]
    pub max_cross_choices: usize,

    /// Mutation chance
    #[clap(long, default_value = "0.2")]
    pub mutation_chance: f64,

    /// Iteration count
    #[clap(long, default_value = "100")]
    pub iteration_count: usize,
    /// Population size
    #[clap(long, default_value = "100")]
    pub population_size: usize,
}

fn main() {
    let cli = CLIParameters::parse();

    let fitness_func: fn(&Vec<f64>) -> f64 = |v| v[0].powi(2) + v[1].powi(2);

    let genetic_parameters = GeneticParameters {
        fitness_evaluater: VectorFitnessEvaluater::new(fitness_func),

        search_radius: cli.search_radius,
        cross_allow_radius: cli.cross_allow_radius,
        max_cross_choices: cli.max_cross_choices,
        mutation_chance: cli.mutation_chance,

        min: vec![-6.0, -6.0],
        max: vec![6.0, 6.0],
    };

    let factory = VectorGeneticFactory::new(genetic_parameters);

    let mut population = Vec::new();
    for _ in 0..cli.population_size {
        population.push(factory.new_chromosome());
    }

    let mut processor = factory.new_processor();
    processor = processor.init_population(population);
    for i in 0..cli.iteration_count {
        println!("Iteration is {}", i);

        processor = processor.populate().cross().mutate();
    }

    let pop = processor.take_population();
    println!("Pop #1: {:?}", pop[0]);
}
