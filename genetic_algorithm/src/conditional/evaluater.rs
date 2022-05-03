use super::chromosome::MultifactorChromosome;
use crate::FitnessEvaluater;

#[derive(Clone)]
pub struct MultifactorFitnessEvaluater {
    pub fitness_func: fn(&Vec<f64>) -> f64,
    pub restrictions: Vec<fn(&Vec<f64>) -> f64>, // Must be zero
    pub eps: f64,
    pub weights: Vec<f64>,
}

impl FitnessEvaluater<MultifactorChromosome<'_>> for MultifactorFitnessEvaluater {
    type FitnessType = f64;

    fn fitness(&self, chromosome: &MultifactorChromosome<'_>) -> f64 {
        let point = &chromosome.vector_chromosome.point;

        let mut penalty = 0.0;
        for i in 0..self.restrictions.len() {
            let p = (self.restrictions[i])(point);
            if p.abs() < self.eps {
                continue;
            } else {
                penalty += self.weights[i] * p.abs();
            }
        }

        (self.fitness_func)(point) + penalty
    }
}
