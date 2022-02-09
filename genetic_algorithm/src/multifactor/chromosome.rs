use crate::{Chromosome, FitnessEvaluater};
use rand::{distributions::Uniform, prelude::*};
use rand_distr::Normal;

use super::VectorFitnessEvaluater;

#[derive(Clone)]
pub struct VectorChromosome<'ranges> {
    pub point: Vec<f64>,
    pub(super) rand: ThreadRng,

    pub(super) min: &'ranges Vec<f64>,
    pub(super) max: &'ranges Vec<f64>,
    pub(super) fitness: &'ranges VectorFitnessEvaluater,
}

impl VectorChromosome<'_> {
    pub fn distance(&self, other: &Self) -> f64 {
        assert_eq!(self.point.len(), other.point.len());

        let mut sum = 0.0;
        for i in 0..self.point.len() {
            sum += (self.point[i] - other.point[i]).powi(2);
        }

        sum.sqrt()
    }

    pub fn fitness(&self) -> f64 {
        self.fitness.fitness(self)
    }
}

impl<'ranges> Chromosome for VectorChromosome<'ranges> {
    type CrossOutput = Vec<VectorChromosome<'ranges>>;

    fn cross(mut self, other: Self) -> Self::CrossOutput {
        let distr = Uniform::new(0.0, 1.0);
        let weight = distr.sample(&mut self.rand);

        let mut out_chromosomes = Vec::new();
        out_chromosomes.reserve(4);
        // Childs insert
        out_chromosomes.push(self.clone());
        out_chromosomes.push(self.clone());

        for i in 0..self.point.len() {
            out_chromosomes[0].point[i] = weight * self.point[i] + (1.0 - weight) * other.point[i];
            out_chromosomes[1].point[i] = weight * other.point[i] + (1.0 - weight) * self.point[i];
        }

        // Parents insert
        out_chromosomes.push(self);
        out_chromosomes.push(other);

        out_chromosomes
    }

    fn mutate(self) -> Self {
        let mut mutated = self;

        if mutated.rand.gen_bool(0.5) {
            // Full random mutation
            for i in 0..mutated.point.len() {
                let min = mutated.min[i];
                let max = mutated.max[i];

                let distr = Uniform::new(min, max);

                mutated.point[i] = distr.sample(&mut mutated.rand);
            }
        } else {
            // Normal distribution mutation
            for i in 0..mutated.point.len() {
                let min = mutated.min[i];
                let max = mutated.max[i];

                let range = max - min;

                let distr = Normal::new(0.0, range / 100.0 / 3.0).unwrap();

                mutated.point[i] += distr.sample(&mut mutated.rand);
            }
        }

        mutated
    }
}
