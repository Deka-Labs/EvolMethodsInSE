mod chromosome;
mod evaluater;
mod factory;
mod processor;

pub use chromosome::VectorChromosome;
pub use evaluater::VectorFitnessEvaluater;
pub use factory::VectorGeneticFactory;
pub use processor::VectorGeneticProcessor;

#[derive(Clone)]
pub struct GeneticParameters {
    /// Fitness function
    pub fitness_evaluater: VectorFitnessEvaluater,

    /// Radius for searching
    pub search_radius: f64,
    /// Radius that allows two chromosomes to cross
    pub cross_allow_radius: f64,
    /// Max try count for search chromosome in cross_allow_radius
    pub max_cross_choices: usize,

    /// Mutation chance
    pub mutation_chance: f64,

    /// Min values
    pub min: Vec<f64>,
    /// Max values
    pub max: Vec<f64>,
}

impl GeneticParameters {
    pub fn new(evaluater: VectorFitnessEvaluater) -> Self {
        Self {
            fitness_evaluater: evaluater,
            search_radius: 0.5,
            cross_allow_radius: 0.25,
            max_cross_choices: 5,
            mutation_chance: 0.2,

            min: vec![-6.0, -6.0],
            max: vec![6.0, 6.0],
        }
    }
}
