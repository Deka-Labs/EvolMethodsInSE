use std::{
    cell::RefCell,
    fs::File,
    io::Write,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{self, channel},
        Arc,
    },
    thread,
    time::Instant,
};

use clap::Parser;
use genetic_algorithm::{
    conditional::{
        GeneticParameters, MultifactorChromosome, MultifactorFitnessEvaluater, VectorGeneticFactory,
    },
    GeneticFactory, GeneticProcessor,
};

#[derive(Parser, Clone)]
#[clap(name = "LR1")]
pub struct CLIParameters {
    /// Radius for searching
    #[clap(long, default_value = "0.01")]
    pub search_radius: f64,
    /// Radius that allows two chromosomes to cross
    #[clap(long, default_value = "0.005")]
    pub cross_allow_radius: f64,
    /// Max try count for search chromosome in cross_allow_radius
    #[clap(long, default_value = "5")]
    pub max_cross_choices: usize,

    /// Mutation chance
    #[clap(long, default_value = "0.3")]
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
    #[clap(long, default_value = "0.7")]
    pub range: f64,

    /// Count of trials to calc avg error
    #[clap(long, default_value = "20")]
    pub error_evaluate_trials: usize,

    /// Weight for h(x) condition
    #[clap(long, default_value = "1")]
    pub weight_h: f64,

    /// Weight for bound condition for x
    #[clap(long, default_value = "1")]
    pub weight_x: f64,

    /// required accuracy for h(x) in 1 step
    #[clap(long, default_value = "0.01")]
    pub start_eps: f64,

    /// required accuracy for h(x) in last step
    #[clap(long, default_value = "0.005")]
    pub end_eps: f64,

    /// Count of trials to calc avg errors, time, etc
    #[clap(long, default_value = "50")]
    pub trials: usize,

    /// Thread count to calc errors
    #[clap(long, default_value = "16")]
    pub thread_count: usize,

    /// File for dump errors from population
    #[clap(long, default_value = "dump_pop_err_pop.csv")]
    pub errors_dump_pop_from_pop: String,

    /// File for dump errors from population
    #[clap(long, default_value = "dump_pop_err_mutation.csv")]
    pub errors_dump_pop_from_mutation: String,

    /// Disable test
    #[clap(long)]
    pub disable_tests: bool,
}

fn main() {
    let cli = CLIParameters::parse();

    println!("First run to create population dump...");
    population_dump_run(&cli, &cli.population_dump);

    println!("Evaluating error...");
    let optimal_points = vec![vec![0.70711, 0.5], vec![-0.70711, 0.5]];

    let res = avg_error_evalualte(&cli, &optimal_points);
    let full_err: f64 = res.iter().sum::<f64>() / res.len() as f64;

    println!("Avg error for all points: {}", full_err);
    println!("Average errors: ");
    for i in 0..optimal_points.len() {
        println!("    Point: {:?} -- {}", optimal_points[i], res[i]);
    }

    if cli.disable_tests {
        return;
    }

    println!("Evaluating time and error for different population sizes...");
    let pop_min = 50;
    let pop_max = 500;
    let pop_step = 25;
    let mut parameters = Vec::with_capacity((pop_max - pop_min) / pop_step);
    for pop_size in (pop_min..=pop_max).step_by(pop_step) {
        let mut copy_cli = cli.clone();
        copy_cli.population_size = pop_size;
        parameters.push(copy_cli);
    }
    evaluate_errors_and_time(
        parameters,
        cli.thread_count,
        cli.trials,
        &cli.errors_dump_pop_from_pop,
    );
    println!("Ready!");

    println!("Evaluating time and error for different mutation chances...");
    let mutation_min = 0;
    let mutation_max = 100;
    let mutation_step = 5;
    let mut parameters = Vec::with_capacity((mutation_max - mutation_min) / mutation_step);
    for mutation_ch in (mutation_min..=mutation_max).step_by(mutation_step) {
        let mut copy_cli = cli.clone();
        copy_cli.mutation_chance = (mutation_ch as f64) / 100.0;
        parameters.push(copy_cli);
    }
    evaluate_errors_and_time(
        parameters,
        cli.thread_count,
        cli.trials,
        &cli.errors_dump_pop_from_mutation,
    );

    println!("Ready!");
}

/// Run genetic algorithm
/// Returns vector of (point, fitness)
fn ga_run<IterF>(cli: &CLIParameters, mut iter_func: IterF) -> Vec<(Vec<f64>, f64, f64)>
where
    for<'inter> IterF: FnMut(usize, &Vec<MultifactorChromosome>),
{
    let cell_eps = RefCell::new(cli.start_eps);

    let fe = MultifactorFitnessEvaluater {
        fitness_func: |v| v[0].powi(2) + (v[1] - 1.0).powi(2),
        restrictions: vec![
            |v| v[1] - v[0].powi(2),
            |v| {
                let o = v[0].abs() - 1.0;
                if o > 0.0 {
                    o
                } else {
                    0.0
                }
            },
            |v| {
                let o = v[1].abs() - 1.0;
                if o > 0.0 {
                    o
                } else {
                    0.0
                }
            },
        ],
        weights: vec![cli.weight_h, cli.weight_x, cli.weight_x],
        eps: cell_eps.clone(),
    };

    let genetic_parameters = GeneticParameters {
        fitness_evaluater: fe.clone(),

        search_radius: cli.search_radius,
        cross_allow_radius: cli.cross_allow_radius,
        max_cross_choices: cli.max_cross_choices,
        mutation_chance: cli.mutation_chance,
        rang_value: cli.rang_value,

        min: vec![-1.0, -1.0],
        max: vec![1.0, 1.0],
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

        *cell_eps.borrow_mut() += (cli.end_eps - cli.start_eps) / cli.iteration_count as f64;
    }

    processor = processor.populate(); // Reduce population

    let pop = processor.finalyze();
    iter_func(cli.iteration_count, &pop);

    // Take only points placed at least cli.range far
    let mut old_size = bests_pop.len();
    bests_pop = optimize_population(bests_pop, cli.range, fe.clone());
    while old_size != bests_pop.len() {
        old_size = bests_pop.len();
        bests_pop = optimize_population(bests_pop, cli.range, fe.clone())
    }

    bests_pop.sort_unstable_by(|l, r| {
        let penalty_diff = fe.penalty(l) - fe.penalty(r);

        if penalty_diff.abs() < 0.00000000000001 {
            return fe.real_fitness(l).partial_cmp(&fe.real_fitness(r)).unwrap();
        }

        fe.penalty(l).partial_cmp(&fe.penalty(r)).unwrap()
    });

    // Convert to (points, fitness, real_fitness)
    let fitness_func: fn(&Vec<f64>) -> f64 = |v| v[0].powi(2) + (v[1] - 1.0).powi(2);
    let mut out = Vec::new();
    for ch in bests_pop {
        let fitness = ch.fitness();
        let fitness_real = fitness_func(&ch.vector_chromosome.point);
        out.push((ch.vector_chromosome.point, fitness, fitness_real));
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
            "    #{} : ({}, {}) ==> Full: {}, Real: {}",
            i + 1,
            result[i].0[0],
            result[i].0[1],
            result[i].1,
            result[i].2
        );
    }
}

fn dump_population_to_file(
    iteration: usize,
    file: &mut File,
    population: &Vec<MultifactorChromosome>,
) {
    let fitness_func: fn(&Vec<f64>) -> f64 = |v| v[0].powi(2) + (v[1] - 1.0).powi(2);
    for ch in population {
        writeln!(
            file,
            "{}, {}, {}, {}",
            iteration,
            ch.vector_chromosome.point[0],
            ch.vector_chromosome.point[1],
            fitness_func(&ch.vector_chromosome.point),
        )
        .unwrap();
    }
}

/// Rebuild population where only 1 chromosome placed in specified range
/// WARNING. If the range around 2 point intesects then the output can have 2 points placed near
fn optimize_population(
    mut population: Vec<MultifactorChromosome>,
    tol: f64,
    fe: MultifactorFitnessEvaluater,
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
                // If yes, check if it is min in range

                let mut is_min = fe.penalty(&processed_ch) < fe.penalty(&new_population[i]);

                let penalty_diff = fe.penalty(&processed_ch) - fe.penalty(&new_population[i]);
                if penalty_diff.abs() < 0.00000000000001 {
                    is_min = fe.real_fitness(&processed_ch) < fe.real_fitness(&new_population[i]);
                }

                if is_min {
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

fn evaluate_errors_and_time(
    parameters: Vec<CLIParameters>,
    thread_count: usize,
    trials_count: usize,
    file_path: &str,
) -> Vec<(f64, f64)> {
    let position = Arc::new(AtomicUsize::new(0));
    let arc_parameters = Arc::new(parameters);

    let (sender, recv) = mpsc::channel();

    // Start threads
    let mut threads = Vec::with_capacity(thread_count);
    for _ in 0..thread_count {
        let thr_pos = Arc::clone(&position);
        let thr_parameters = Arc::clone(&arc_parameters);
        let thr_sender = sender.clone();

        let thr = thread::spawn(move || {
            let mut id = thr_pos.fetch_add(1, Ordering::SeqCst);

            while id < (trials_count * thr_parameters.len()) {
                let p = &thr_parameters[id % thr_parameters.len()];
                let (time, error) = evaluate_errors_and_time_single(p);

                thr_sender
                    .send((id % thr_parameters.len(), time, error))
                    .unwrap();

                id = thr_pos.fetch_add(1, Ordering::SeqCst);
            }
        });

        threads.push(thr);
    }
    // drop original sender
    drop(sender);

    // getting messages
    let mut out = vec![(0.0, 0.0); arc_parameters.len()];
    let mut trials_count_arr = vec![0 as usize; arc_parameters.len()];

    for (id, time, error) in recv {
        let (old_time, old_error) = out[id];
        out[id] = (old_time + time, old_error + error);
        trials_count_arr[id] = trials_count_arr[id] + 1;

        let abs_progress: usize = trials_count_arr.clone().iter().sum();
        println!(
            "Progress: {:.2}%...",
            abs_progress as f64 / (trials_count * arc_parameters.len()) as f64 * 100.0
        )
    }

    let l = arc_parameters.len() as f64;
    for (t, e) in &mut out {
        *t = *t / l;
        *e = *e / l;
    }

    let mut out_file = File::create(file_path).unwrap();

    for i in 0..out.len() {
        writeln!(&mut out_file, "{}, {}, {}", i, out[i].0, out[i].1).unwrap();
    }

    return out;
}

fn evaluate_errors_and_time_single(parameters: &CLIParameters) -> (f64, f64) {
    let start_time = Instant::now();

    // Run algorithm
    let result = ga_run(&parameters, |_, _| {});
    let time = start_time.elapsed().as_secs_f64();

    // Find errors
    let best_points = vec![vec![0.70711, 0.5], vec![-0.70711, 0.5]];
    let mut best_error = vec![0.0; best_points.len()];

    // Distance between points function
    let distance: fn(l: &Vec<f64>, r: &Vec<f64>) -> f64 = |l, r| {
        assert_eq!(l.len(), r.len());

        let mut sum = 0.0;
        for i in 0..l.len() {
            sum += (l[i] - r[i]).powi(2);
        }

        sum.sqrt()
    };

    // Init start errors
    let (first_point, _, _) = result.first().unwrap();
    for i in 0..best_points.len() {
        best_error[i] = distance(&first_point, &best_points[i]);
    }

    // Calculate errors
    for (point, _, _) in result {
        for i in 0..best_points.len() {
            let dist = distance(&point, &best_points[i]);

            if dist < best_error[i] {
                best_error[i] = dist;
            }
        }
    }

    // Average errors
    let mut sum = 0.0;
    for v in &best_error {
        sum += v;
    }
    sum /= best_error.len() as f64;

    return (time, sum);
}
