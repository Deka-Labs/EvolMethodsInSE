mod chromosome;
mod factory;
mod processor;

pub use chromosome::MulticriteriaChromosome;
pub use factory::MulticriteriaFactory;
pub use processor::MulticriteriaGeneticProcessor;

use crate::vector::VectorFitnessEvaluater;

#[derive(Clone)]
pub struct GeneticParameters {
    /// Fitness function
    pub fitness_evaluater: Vec<VectorFitnessEvaluater>,

    pub is_maximization: bool,

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

    /// Ranging value
    /// rang_value in [1, 2]
    pub rang_value: f64,
}
