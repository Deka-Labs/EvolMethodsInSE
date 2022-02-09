pub mod multifactor;

/// Represents chromosome in Genetic Algorithms with basic operators
pub trait Chromosome<CrossType = Self> {
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
