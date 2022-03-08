use std::{fs::File, io::Write, sync::mpsc::channel, thread};

use clap::Parser;
use genetic_algorithm::{
    multifactor::{
        GeneticParameters, MultifactorChromosome, MultifactorFitnessEvaluater, VectorGeneticFactory,
    },
    FitnessEvaluater, GeneticFactory, GeneticProcessor,
};

#[derive(Parser, Clone)]
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

    println!("Evaluating error...");
    let optimal_points = vec![
        vec![3.0, 2.0],
        vec![-2.805118, 3.131312],
        vec![-3.779310, -3.283186],
        vec![3.584428, -1.84812],
    ];

    let res = avg_error_evalualte(&cli, &optimal_points);
    let full_err: f64 = res.iter().sum::<f64>() / res.len() as f64;

    println!("Avg error for all points: {}", full_err);
    println!("Average errors: ");
    for i in 0..optimal_points.len() {
        println!("    Point: {:?} -- {}", optimal_points[i], res[i]);
    }
}

/// Run genetic algorithm
/// Returns vector of (point, fitness)
fn ga_run<IterF>(cli: &CLIParameters, mut iter_func: IterF) -> Vec<(Vec<f64>, f64)>
where
    for<'inter> IterF: FnMut(usize, &Vec<MultifactorChromosome>),
{
    let fitness_func: fn(&Vec<f64>) -> f64 =
        |v| (2500.0 - (v[0].powi(2) + v[1] - 11.0).powi(2) - (v[0] + v[1].powi(2) - 7.0).powi(2));
    let fe = MultifactorFitnessEvaluater::new(fitness_func);

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

    let factory = VectorGeneticFactory::new(genetic_parameters);

    let mut population = Vec::new();
    for _ in 0..cli.population_size {
        population.push(factory.new_chromosome());
    }

    let mut bests_pop = Vec::new();

    let mut processor = factory.new_processor();
    processor = processor.init_population(population);
    for i in 0..cli.iteration_count {
        processor = processor.populate();

        let tmp_pop = processor.population();
        iter_func(i, tmp_pop);
        bests_pop.extend(processor.top_chromosomes(cli.elite_count, fe.clone()));

        processor = processor.cross().mutate();
    }

    processor = processor.populate(); // Reduce population

    let pop = processor.take_population();
    iter_func(cli.iteration_count, &pop);

    // Take only points placed at least cli.range far
    let mut old_size = bests_pop.len();
    bests_pop = optimize_population(bests_pop, cli.range);
    while old_size != bests_pop.len() {
        old_size = bests_pop.len();
        bests_pop = optimize_population(bests_pop, cli.range)
    }

    bests_pop.sort_unstable_by(|l, r| fe.fitness(r).partial_cmp(&fe.fitness(l)).unwrap());

    // Convert to (points, fitness)
    let mut out = Vec::new();
    for ch in bests_pop {
        let fitness = ch.fitness();
        out.push((ch.vector_chromosome.point, fitness));
    }

    return out;
}

/// Run GA 1 time to dump population info
fn population_dump_run(cli: &CLIParameters, filename: &str) {
    let mut population_file = File::create(filename).unwrap();

    let result = ga_run(cli, move |i, p: &Vec<MultifactorChromosome>| {
        dump_population_to_file(i, &mut population_file, p);
    });

    println!("Top chromosomes:");
    for i in 0..result.len() {
        println!(
            "    #{} : ({}, {}) ==> {}",
            i + 1,
            result[i].0[0],
            result[i].0[1],
            result[i].1
        );
    }
}

fn dump_population_to_file(
    iteration: usize,
    file: &mut File,
    population: &Vec<MultifactorChromosome>,
) {
    for ch in population {
        writeln!(
            file,
            "{}, {}, {}, {}",
            iteration,
            ch.vector_chromosome.point[0],
            ch.vector_chromosome.point[1],
            ch.fitness()
        )
        .unwrap();
    }
}

/// Rebuild population where only 1 chromosome placed in specified range
/// WARNING. If the range around 2 point intesects then the output can have 2 points placed near
fn optimize_population(
    mut population: Vec<MultifactorChromosome>,
    tol: f64,
) -> Vec<MultifactorChromosome> {
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

/// Run N time to find out average error from optimal points
fn avg_error_evalualte(cli: &CLIParameters, optimal_points: &Vec<Vec<f64>>) -> Vec<f64> {
    let (tx, rx) = channel();
    let mut threads = Vec::new();

    for _ in 0..cli.error_evaluate_trials {
        let thr_tx = tx.clone();
        let thr_cli = cli.clone();
        let thr_pts = optimal_points.clone();

        let thr = thread::spawn(move || {
            let err = error_evaluate(thr_cli, thr_pts);
            thr_tx.send(err).unwrap();
        });

        threads.push(thr);
    }

    let mut avg_err: Vec<f64> = Vec::new();
    avg_err.resize(optimal_points.len(), 0.0);
    for i in 0..cli.error_evaluate_trials {
        let res = rx.recv().unwrap();
        avg_err = avg_err.iter().zip(&res).map(|(x, y)| x + y).collect();
        println!("Progress: {}/{}", i, cli.error_evaluate_trials)
    }

    for th in threads {
        th.join().expect("Unknown error");
    }

    avg_err = avg_err
        .iter()
        .map(|x| x / cli.error_evaluate_trials as f64)
        .collect();

    return avg_err;
}

/// Run to calculate error from real maximum
fn error_evaluate(cli: CLIParameters, optimal_points: Vec<Vec<f64>>) -> Vec<f64> {
    let result = ga_run(&cli, |_, _| {});

    let distance = |x: &Vec<f64>, y: &Vec<f64>| {
        let mut s = 0.0;
        for i in 0..x.len() {
            s += (x[i] - y[i]).powi(2);
        }
        s.sqrt()
    };

    let mut out = Vec::new();
    out.reserve(optimal_points.len());
    for p in &optimal_points {
        let mut min_dist = distance(p, &result.first().unwrap().0);
        for i in 1..result.len() {
            let dist = distance(p, &result[i].0);
            if dist < min_dist {
                min_dist = dist;
            }
        }
        out.push(min_dist);
    }

    out
}
