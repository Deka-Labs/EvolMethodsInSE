use crate::{
    vector::{VectorChromosome, VectorFitnessEvaluater},
    Chromosome, FitnessEvaluater,
};

#[derive(Clone)]
pub struct MulticriteriaChromosome<'ranges> {
    pub vector_chromosome: VectorChromosome<'ranges>,
    pub(super) fitness_evaluaters: &'ranges Vec<VectorFitnessEvaluater>,
}

impl MulticriteriaChromosome<'_> {
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

    pub fn fitness(&self, id: usize) -> f64 {
        self.fitness_evaluaters[id].fitness(&self.vector_chromosome)
    }
}

impl<'ranges> Chromosome for MulticriteriaChromosome<'ranges> {
    type CrossOutput = Vec<MulticriteriaChromosome<'ranges>>;

    fn cross(self, other: Self) -> Self::CrossOutput {
        let vec_result = self.vector_chromosome.cross(other.vector_chromosome);
        let mut out_result = Vec::with_capacity(vec_result.len());
        for v in vec_result {
            let out_ch = MulticriteriaChromosome {
                vector_chromosome: v,
                fitness_evaluaters: self.fitness_evaluaters,
            };
            out_result.push(out_ch);
        }

        out_result
    }

    fn mutate(self) -> Self {
        let fit_fun = self.fitness_evaluaters;
        let vec_ch = self.vector_chromosome.mutate();

        MulticriteriaChromosome {
            vector_chromosome: vec_ch,
            fitness_evaluaters: fit_fun,
        }
    }
}
