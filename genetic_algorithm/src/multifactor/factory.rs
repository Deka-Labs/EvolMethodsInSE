use rand::thread_rng;
use rand_distr::{Distribution, Uniform};

use crate::GeneticFactory;

use super::{GeneticParameters, MultifactorChromosome, VectorGeneticProcessor};

pub struct VectorGeneticFactory {
    p: GeneticParameters,
}

impl VectorGeneticFactory {
    pub fn new(p: GeneticParameters) -> Self {
        Self { p: p }
    }
}

impl<'fact> GeneticFactory<'fact, MultifactorChromosome<'fact>, VectorGeneticProcessor<'fact>>
    for VectorGeneticFactory
{
    fn new_chromosome(&'fact self) -> MultifactorChromosome<'fact> {
        let mut point = Vec::new();
        point.reserve(self.p.min.len());

        for i in 0..self.p.min.len() {
            let distr = Uniform::new(self.p.min[i], self.p.max[i]);
            point.push(distr.sample(&mut thread_rng()));
        }

        MultifactorChromosome {
            max: &self.p.max,
            min: &self.p.min,
            point: point,
            rand: thread_rng(),
            fitness: &self.p.fitness_evaluater,
        }
    }

    fn new_processor(&'fact self) -> VectorGeneticProcessor<'_> {
        VectorGeneticProcessor::new(self.p.clone())
    }
}
