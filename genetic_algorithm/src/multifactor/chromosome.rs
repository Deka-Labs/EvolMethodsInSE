use crate::{vector::VectorChromosome, Chromosome, FitnessEvaluater};

use super::VectorFitnessEvaluater;

#[derive(Clone)]
pub struct MultifactorChromosome<'ranges> {
    pub vector_chromosome: VectorChromosome<'ranges>,
    pub(super) fitness: &'ranges VectorFitnessEvaluater,
}

impl MultifactorChromosome<'_> {
    pub fn distance(&self, other: &Self) -> f64 {
        let self_point = &self.vector_chromosome.point;
        let other_point = &other.vector_chromosome.point;

        assert_eq!(self_point.len(), other_point.len());

        let mut sum = 0.0;
        for i in 0..self_point.len() {
            sum += (self_point[i] - other_point[i]).powi(2);
        }

        sum.sqrt()
    }

    pub fn fitness(&self) -> f64 {
        self.fitness.fitness(self)
    }
}

impl<'ranges> Chromosome for MultifactorChromosome<'ranges> {
    type CrossOutput = Vec<MultifactorChromosome<'ranges>>;

    fn cross(self, other: Self) -> Self::CrossOutput {
        let vec_result = self.vector_chromosome.cross(other.vector_chromosome);
        let mut out_result = Vec::with_capacity(vec_result.len());
        for v in vec_result {
            let out_ch = MultifactorChromosome {
                vector_chromosome: v,
                fitness: self.fitness,
            };
            out_result.push(out_ch);
        }

        out_result
    }

    fn mutate(self) -> Self {
        let fit_fun = self.fitness;
        let vec_ch = self.vector_chromosome.mutate();

        MultifactorChromosome {
            vector_chromosome: vec_ch,
            fitness: fit_fun,
        }
    }
}
