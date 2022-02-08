use self::evaluater::VectorFitnessEvaluater;

mod chromosome;
mod evaluater;
mod processor;

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
}

impl GeneticParameters {
    pub fn new(evaluater: VectorFitnessEvaluater) -> Self {
        Self {
            fitness_evaluater: evaluater,
            search_radius: 0.5,
            cross_allow_radius: 0.25,
            max_cross_choices: 5,
            mutation_chance: 0.2,
        }
    }
}
