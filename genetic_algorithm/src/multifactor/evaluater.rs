use super::chromosome::MultifactorChromosome;
use crate::{vector::VectorFitnessEvaluater, FitnessEvaluater};

#[derive(Clone)]
pub struct MultifactorFitnessEvaluater {
    vector_evaluater: VectorFitnessEvaluater,
}

impl MultifactorFitnessEvaluater {
    pub fn new(fitness: fn(&Vec<f64>) -> f64) -> MultifactorFitnessEvaluater {
        return MultifactorFitnessEvaluater {
            vector_evaluater: VectorFitnessEvaluater::new(fitness),
        };
    }
}

impl FitnessEvaluater<MultifactorChromosome<'_>> for MultifactorFitnessEvaluater {
    type FitnessType = f64;

    fn fitness(&self, chromosome: &MultifactorChromosome<'_>) -> f64 {
        return self.vector_evaluater.fitness(&chromosome.vector_chromosome);
    }
}
