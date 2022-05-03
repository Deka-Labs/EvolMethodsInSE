use std::cell::RefCell;

use super::chromosome::MultifactorChromosome;
use crate::FitnessEvaluater;

#[derive(Clone)]
pub struct MultifactorFitnessEvaluater {
    pub fitness_func: fn(&Vec<f64>) -> f64,
    pub restrictions: Vec<fn(&Vec<f64>) -> f64>, // Must be zero
    pub eps: RefCell<f64>,
    pub weights: Vec<f64>,
}

impl MultifactorFitnessEvaluater {
    pub fn real_fitness(&self, chromosome: &MultifactorChromosome<'_>) -> f64 {
        let point = &chromosome.vector_chromosome.point;
        (self.fitness_func)(point)
    }

    pub fn penalty(&self, chromosome: &MultifactorChromosome<'_>) -> f64 {
        let point = &chromosome.vector_chromosome.point;

        let mut penalty = 0.0;
        for i in 0..self.restrictions.len() {
            let p = (self.restrictions[i])(point);
            if p.abs() < *self.eps.borrow() {
                continue;
            } else {
                penalty += self.weights[i] * p.abs();
            }
        }

        penalty
    }
}

impl FitnessEvaluater<MultifactorChromosome<'_>> for MultifactorFitnessEvaluater {
    type FitnessType = f64;

    fn fitness(&self, chromosome: &MultifactorChromosome<'_>) -> f64 {
        self.real_fitness(chromosome) + self.penalty(chromosome)
    }
}
