use std::{fs::File, io::Write};

use clap::Parser;
use genetic_algorithm::{
    multifactor::{
        GeneticParameters, VectorChromosome, VectorFitnessEvaluater, VectorGeneticFactory,
    },
    FitnessEvaluater, GeneticFactory, GeneticProcessor,
};

#[derive(Parser)]
#[clap(name = "LR1")]
pub struct CLIParameters {
    /// Radius for searching
    #[clap(long, default_value = "3")]
    pub search_radius: f64,
    /// Radius that allows two chromosomes to cross
    #[clap(long, default_value = "2")]
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
    #[clap(long, default_value = "300")]
    pub population_size: usize,

    /// Population dump
    #[clap(long, default_value = "dump_pop.csv")]
    pub population_dump: String,

    /// Ranging value
    /// rang_value in [1, 2]
    #[clap(long, default_value = "1.3")]
    pub rang_value: f64,

    /// A distance tolerance for displaying best in population
    #[clap(long, default_value = "3")]
    pub range: f64,
}

fn main() {
    let cli = CLIParameters::parse();

    let fitness_func: fn(&Vec<f64>) -> f64 =
        |v| (2500.0 - (v[0].powi(2) + v[1] - 11.0).powi(2) - (v[0] + v[1].powi(2) - 7.0).powi(2));
    let fe = VectorFitnessEvaluater::new(fitness_func);

    let genetic_parameters = GeneticParameters {
        fitness_evaluater: fe.clone(),

        search_radius: cli.search_radius,
        cross_allow_radius: cli.cross_allow_radius,
        max_cross_choices: cli.max_cross_choices,
        mutation_chance: cli.mutation_chance,
        rang_value: cli.rang_value,

        min: vec![-6.0, -6.0],
        max: vec![6.0, 6.0],
    };

    let mut population_file = File::create(cli.population_dump).unwrap();

    let factory = VectorGeneticFactory::new(genetic_parameters);

    let mut population = Vec::new();
    for _ in 0..cli.population_size {
        population.push(factory.new_chromosome());
    }

    let mut processor = factory.new_processor();
    processor = processor.init_population(population);
    for i in 0..cli.iteration_count {
        println!("Iteration is {}", i);
        let tmp_pop = processor.population();
        dump_population_to_file(i, &mut population_file, tmp_pop);

        processor = processor.populate().cross().mutate();
    }

    processor = processor.populate(); // Reduce population

    let mut pop = processor.take_population();
    dump_population_to_file(cli.iteration_count, &mut population_file, &pop);

    // Take only points placed at least cli.range far
    let mut old_size = pop.len();
    pop = optimize_population(pop, cli.range);
    while old_size != pop.len() {
        old_size = pop.len();
        pop = optimize_population(pop, cli.range)
    }

    pop.sort_unstable_by(|l, r| fe.fitness(r).partial_cmp(&fe.fitness(l)).unwrap());
    println!("Top chromosomes: ");
    for i in 0..pop.len() {
        println!(
            "#{}: [{:?}] -- {}",
            i + 1,
            &pop[i].point,
            fe.fitness(&pop[i])
        )
    }
}

fn dump_population_to_file(iteration: usize, file: &mut File, population: &Vec<VectorChromosome>) {
    for ch in population {
        writeln!(
            file,
            "{}, {}, {}, {}",
            iteration,
            ch.point[0],
            ch.point[1],
            ch.fitness()
        )
        .unwrap();
    }
}

/// Rebuild population where only 1 chromosome placed in specified range
/// WARNING. If the range around 2 point intesects then the output can have 2 points placed near
fn optimize_population(mut population: Vec<VectorChromosome>, tol: f64) -> Vec<VectorChromosome> {
    let mut origin_population = Vec::new();
    origin_population.push(population.swap_remove(0));
    let mut new_population = origin_population.clone();

    // Fast check
    while !population.is_empty() {
        // take a first element
        let processed_ch = population.swap_remove(0);
        // check if it is in range of origin chromosomes
        let mut breaked = false;
        for i in 0..new_population.len() {
            if processed_ch.distance(&origin_population[i]) < tol {
                // If yes, check if it is max in range
                if processed_ch.fitness() > new_population[i].fitness() {
                    new_population[i] = processed_ch.clone()
                }
                breaked = true;
                break;
            }
        }
        // If reached end ->  there are no such chromosomes
        if !breaked {
            new_population.push(processed_ch.clone());
            origin_population.push(processed_ch);
        }
    }

    new_population
}
