use rand::thread_rng;
use rand_distr::{Distribution, Uniform};

use crate::{vector::VectorChromosome, GeneticFactory};

use super::{processor::MulticriteriaGeneticProcessor, GeneticParameters, MulticriteriaChromosome};

pub struct MulticriteriaFactory {
    p: GeneticParameters,
}

impl MulticriteriaFactory {
    pub fn new(p: GeneticParameters) -> Self {
        Self { p: p }
    }
}

impl<'fact>
    GeneticFactory<'fact, MulticriteriaChromosome<'fact>, MulticriteriaGeneticProcessor<'fact>>
    for MulticriteriaFactory
{
    fn new_chromosome(&'fact self) -> MulticriteriaChromosome<'fact> {
        let mut point = Vec::new();
        point.reserve(self.p.min.len());

        for i in 0..self.p.min.len() {
            let distr = Uniform::new(self.p.min[i], self.p.max[i]);
            point.push(distr.sample(&mut thread_rng()));
        }

        MulticriteriaChromosome {
            vector_chromosome: VectorChromosome {
                max: &self.p.max,
                min: &self.p.min,
                point: point,
                rand: thread_rng(),
            },

            fitness_evaluaters: &self.p.fitness_evaluater,
        }
    }

    fn new_processor(&'fact self) -> MulticriteriaGeneticProcessor<'fact> {
        MulticriteriaGeneticProcessor::new(self.p.clone())
    }
}
