use super::chromosome::VectorChromosome;
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

impl FitnessEvaluater<VectorChromosome<'_>> for VectorFitnessEvaluater {
    fn fitness(&self, chromosome: &VectorChromosome<'_>) -> f64 {
        return (self.fitness_func)(&chromosome.point);
    }
}
