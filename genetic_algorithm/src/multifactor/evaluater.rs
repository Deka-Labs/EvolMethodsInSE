use super::chromosome::MultifactorChromosome;
use crate::FitnessEvaluater;

#[derive(Clone)]
pub struct VectorFitnessEvaluater {
    fitness_func: fn(&Vec<f64>) -> f64,
}

impl VectorFitnessEvaluater {
    pub fn new(fitness: fn(&Vec<f64>) -> f64) -> VectorFitnessEvaluater {
        return VectorFitnessEvaluater {
            fitness_func: fitness,
        };
    }
}

impl FitnessEvaluater<MultifactorChromosome<'_>> for VectorFitnessEvaluater {
    type FitnessType = f64;

    fn fitness(&self, chromosome: &MultifactorChromosome<'_>) -> f64 {
        return (self.fitness_func)(&chromosome.point);
    }
}
