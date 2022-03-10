use std::{fs::File, io::Write};

use clap::Parser;
use genetic_algorithm::{
    multicriteria::{GeneticParameters, MulticriteriaChromosome, MulticriteriaFactory},
    vector::VectorFitnessEvaluater,
    GeneticFactory, GeneticProcessor,
};

#[derive(Parser, Clone)]
#[clap(name = "LR1")]
pub struct CLIParameters {
    /// Radius for searching
    #[clap(long, default_value = "1")]
    pub search_radius: f64,
    /// Radius that allows two chromosomes to cross
    #[clap(long, default_value = "0.2")]
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
    /// Count of best ant chosen for final result
    #[clap(long, default_value = "5")]
    pub elite_count: usize,

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

    /// Count of trials to calc avg error
    #[clap(long, default_value = "100")]
    pub error_evaluate_trials: usize,
}

fn main() {
    let cli = CLIParameters::parse();

    println!("First run to create population dump...");
    population_dump_run(&cli, &cli.population_dump);
}

/// Run genetic algorithm
/// Returns vector of (point, fitness)
fn ga_run<IterF>(cli: &CLIParameters, mut iter_func: IterF) -> Vec<(Vec<f64>, Vec<f64>)>
where
    for<'inter> IterF: FnMut(usize, &Vec<MulticriteriaChromosome>),
{
    let mut crits = Vec::with_capacity(3);
    crits.push(VectorFitnessEvaluater::new(|v| {
        v[0].powi(2) / 2.0 + (v[1] + 1.0).powi(2) / 13.0 + 3.0
    }));
    crits.push(VectorFitnessEvaluater::new(|v| {
        v[0].powi(2) / 2.0 + (2.0 * v[1] + 2.0).powi(2) / 15.0 + 1.0
    }));
    crits.push(VectorFitnessEvaluater::new(|v| {
        (v[0] + 2.0 * v[1] - 1.0).powi(2) / 175.0 + (2.0 * v[1] - v[0]).powi(2) / 27.0 - 13.0
    }));

    let genetic_parameters = GeneticParameters {
        fitness_evaluater: crits.clone(),

        search_radius: cli.search_radius,
        cross_allow_radius: cli.cross_allow_radius,
        max_cross_choices: cli.max_cross_choices,
        mutation_chance: cli.mutation_chance,
        rang_value: cli.rang_value,

        min: vec![-4.0, -4.0],
        max: vec![4.0, 4.0],

        is_maximization: false,
    };

    let factory = MulticriteriaFactory::new(genetic_parameters);

    let mut population = Vec::new();
    for _ in 0..cli.population_size {
        population.push(factory.new_chromosome());
    }

    let mut processor = factory.new_processor();
    processor = processor.init_population(population);
    for i in 0..cli.iteration_count {
        processor = processor.populate();

        let tmp_pop = processor.population();
        iter_func(i, tmp_pop);

        processor = processor.cross().mutate();
    }

    processor = processor.populate(); // Reduce population

    let raw_population = processor.population();
    iter_func(cli.iteration_count, &raw_population);
    let pop = processor.finalyze();

    let mut out = Vec::with_capacity(pop.len());
    for ch in pop {
        let mut fitness_eval = Vec::with_capacity(ch.criterions_count());

        for i in 0..ch.criterions_count() {
            let fitness = ch.fitness(i);
            fitness_eval.push(fitness);
        }

        out.push((ch.vector_chromosome.point, fitness_eval));
    }

    return out;
}

/// Run GA 1 time to dump population info
fn population_dump_run(cli: &CLIParameters, filename: &str) {
    let mut population_file = File::create(filename).unwrap();

    let _ = ga_run(cli, move |i, p: &Vec<MulticriteriaChromosome>| {
        dump_population_to_file(i, &mut population_file, p);
    });
    // Select pareto optimals

    println!("Ready!");
}

fn dump_population_to_file(
    iteration: usize,
    file: &mut File,
    population: &Vec<MulticriteriaChromosome>,
) {
    for ch in population {
        let mut fitness_eval = Vec::with_capacity(ch.criterions_count());

        for i in 0..ch.criterions_count() {
            let fitness = ch.fitness(i);
            fitness_eval.push(fitness);
        }

        let values: Vec<_> = fitness_eval.iter().map(|x| x.to_string()).collect();
        let values_str = values.join(", ");

        writeln!(
            file,
            "{}, {}, {}, {}",
            iteration, ch.vector_chromosome.point[0], ch.vector_chromosome.point[1], values_str
        )
        .unwrap();
    }
}
