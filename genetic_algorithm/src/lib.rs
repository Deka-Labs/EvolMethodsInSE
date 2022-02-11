pub mod multifactor;

/// Represents chromosome in Genetic Algorithms with basic operators
pub trait Chromosome<CrossType = Self>: Clone {
    type CrossOutput;

    /// Returns one or more chromosome after crossingover()
    fn cross(self, other: CrossType) -> Self::CrossOutput;
    /// Mutate chromosome
    fn mutate(self) -> Self;
}

pub trait FitnessEvaluater<C: Chromosome> {
    fn fitness(&self, chromosome: &C) -> f64;
}

/// It sets 4 main genetic operators on population of chromosomes
pub trait GeneticProcessor<ChromosomeType: Chromosome> {
    /// Sets init population for apply operators
    fn init_population(self, start_population: Vec<ChromosomeType>) -> Self;

    fn populate(self) -> Self;
    fn cross(self) -> Self;
    fn mutate(self) -> Self;

    fn population(&self) -> &Vec<ChromosomeType>;
    fn take_population(self) -> Vec<ChromosomeType>;

    fn top_chromosomes<FE: FitnessEvaluater<ChromosomeType>>(
        &self,
        count: usize,
        fe: FE,
    ) -> Vec<ChromosomeType> {
        let mut pop = self.population().clone();
        assert!(0 < count);
        assert!(count < pop.len());

        pop.sort_unstable_by(|l, r| fe.fitness(r).partial_cmp(&fe.fitness(l)).unwrap());

        let mut out = Vec::with_capacity(count);

        for i in 0..count {
            out.push(pop[i].clone());
        }

        out
    }
}

pub trait GeneticFactory<
    'fact,
    ChromosomeType: 'fact + Chromosome,
    ProcessorType: 'fact + GeneticProcessor<ChromosomeType>,
>
{
    fn new_chromosome(&'fact self) -> ChromosomeType;
    fn new_processor(&'fact self) -> ProcessorType;
}
