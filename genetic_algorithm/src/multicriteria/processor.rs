use rand::prelude::*;
use rand_distr::{Uniform, WeightedAliasIndex};

use crate::{Chromosome, FitnessEvaluater, GeneticProcessor};

use super::{GeneticParameters, MulticriteriaChromosome};

pub struct MulticriteriaGeneticProcessor<'pop> {
    population: Option<Vec<MulticriteriaChromosome<'pop>>>,
    population_size: usize,
    rand: ThreadRng,

    parameters: GeneticParameters,
}

impl MulticriteriaGeneticProcessor<'_> {
    pub fn new(parameters: GeneticParameters) -> Self {
        Self {
            population: None,
            population_size: 0,
            rand: thread_rng(),
            parameters: parameters,
        }
    }
}

impl<'pop> GeneticProcessor<MulticriteriaChromosome<'pop>> for MulticriteriaGeneticProcessor<'pop> {
    fn init_population(self, start_population: Vec<MulticriteriaChromosome<'pop>>) -> Self {
        let size = start_population.len();
        Self {
            population: Some(start_population),
            population_size: size,
            rand: self.rand,
            parameters: self.parameters,
        }
    }

    fn populate(mut self) -> Self {
        let criterion_count = self.parameters.fitness_evaluater.len();
        let subpopulation_size = self.population_size / criterion_count;

        let mut population = self.population.unwrap();
        let mut new_population = Vec::new();
        new_population.reserve(self.population_size);

        for i in 0..criterion_count {
            let fe = &self.parameters.fitness_evaluater[i];
            population.sort_unstable_by(|l, r| {
                fe.fitness(&r.vector_chromosome)
                    .partial_cmp(&fe.fitness(&l.vector_chromosome))
                    .unwrap()
            });

            if !self.parameters.is_maximization {
                population.reverse();
            }

            // Ranging probs calculate
            let mut weights = Vec::new();
            weights.reserve(population.len());
            let a = self.parameters.rang_value;
            let b = 2.0 - a;
            let n = population.len() as f64;
            for i in 0..population.len() {
                let pos = i as f64;
                weights.push((1.0 / n) * (a - (a - b) * pos / (n - 1.0)));
            }

            // Select part of new population
            for _ in 0..subpopulation_size {
                let distr = WeightedAliasIndex::new(weights.clone()).unwrap();
                let index = distr.sample(&mut self.rand);

                let ch = population.remove(index);
                let _ = weights.remove(index);

                new_population.push(ch);
            }
        }

        if new_population.len() < self.population_size {
            // There are some empty space in population, we will fill it by fully random
            let required_count = self.population_size - new_population.len();

            for _ in 0..required_count {
                let distr = Uniform::new(0, population.len());
                let index = distr.sample(&mut self.rand);

                let selected = population.remove(index);
                new_population.push(selected);
            }
        }

        Self {
            population: Some(new_population),
            population_size: self.population_size,
            rand: self.rand,
            parameters: self.parameters,
        }
    }

    fn cross(mut self) -> Self {
        let mut population = self.population.unwrap();
        population.shuffle(&mut self.rand); // Shuffle to make choices of parents random

        let mut new_population = Vec::new();
        new_population.reserve(2 * population.len());

        while population.len() > 1 {
            let first_element = population.swap_remove(0);
            // Taking chromosomes in radius search_radius
            let mut chrs_in_radius = Vec::new();
            let mut real_indexes = Vec::new();
            for i in 0..population.len() {
                let ch = &population[i];
                if first_element.distance(ch) <= self.parameters.search_radius {
                    chrs_in_radius.push(ch);
                    real_indexes.push(i);
                }
            }
            // Shuffle vector
            chrs_in_radius.shuffle(&mut self.rand);
            // Search for chromosome in cross_allow_radius
            let i = 0;
            for i in 0..chrs_in_radius.len() {
                let ch = chrs_in_radius[i];
                if first_element.distance(ch) <= self.parameters.cross_allow_radius {
                    break;
                }
            }

            let second_element_id;
            if i < chrs_in_radius.len() {
                // We found required result
                second_element_id = real_indexes[i];
            } else {
                // Choose a random chromosome
                second_element_id = self.rand.gen_range(0..population.len());
            }
            let second_element = population.swap_remove(second_element_id);
            let cross_childs = first_element.cross(second_element);
            new_population.extend(cross_childs);
        }

        Self {
            population: Some(new_population),
            population_size: self.population_size,
            rand: self.rand,
            parameters: self.parameters,
        }
    }

    fn mutate(mut self) -> Self {
        let mut population = self.population.unwrap();

        population = population
            .into_iter()
            .map(|mut ch| {
                if self.rand.gen_bool(self.parameters.mutation_chance) {
                    ch = ch.mutate()
                }
                ch
            })
            .collect();

        Self {
            population: Some(population),
            population_size: self.population_size,
            rand: self.rand,
            parameters: self.parameters,
        }
    }

    fn population(&self) -> &Vec<MulticriteriaChromosome<'pop>> {
        if let Some(pop) = &self.population {
            pop
        } else {
            unimplemented!()
        }
    }

    fn take_population(self) -> Vec<MulticriteriaChromosome<'pop>> {
        self.population.unwrap()
    }
}
