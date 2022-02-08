use rand::prelude::*;
use rand_distr::Exp;

use crate::{Chromosome, FitnessEvaluater, GeneticProcessor};

use super::{chromosome::VectorChromosome, GeneticParameters};

pub struct VectorGeneticProcessor<'pop> {
    population: Option<Vec<VectorChromosome<'pop>>>,
    population_size: usize,
    rand: ThreadRng,

    parameters: GeneticParameters,
}

impl VectorGeneticProcessor<'_> {
    pub fn new(parameters: GeneticParameters) -> Self {
        Self {
            population: None,
            population_size: 0,
            rand: thread_rng(),
            parameters: parameters,
        }
    }
}

impl<'pop> GeneticProcessor<VectorChromosome<'pop>> for VectorGeneticProcessor<'pop> {
    fn init_population(self, start_population: Vec<VectorChromosome<'pop>>) -> Self {
        let size = start_population.len();
        Self {
            population: Some(start_population),
            population_size: size,
            rand: self.rand,
            parameters: self.parameters,
        }
    }

    fn populate(mut self) -> Self {
        // Sort population
        let mut population = self.population.unwrap();
        let fe = &self.parameters.fitness_evaluater;
        population.sort_unstable_by(|l, r| fe.fitness(l).partial_cmp(&fe.fitness(r)).unwrap());

        // Will select by using Exponential distribution
        // Mean will be index of center element
        let lambda = 1.0 / (self.population_size / 2) as f64;
        let distr = Exp::new(lambda).unwrap();

        let mut new_population = Vec::new();
        new_population.reserve(self.population_size);

        for _ in 0..self.population_size {
            let mut index = distr.sample(&mut self.rand).floor() as usize;
            index = index.clamp(0, self.population_size - 1);

            new_population.push(population[index].clone());
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

    fn reduce(self) -> Self {
        self.populate()
    }

    fn population(&self) -> &Vec<VectorChromosome<'pop>> {
        if let Some(pop) = &self.population {
            pop
        } else {
            unimplemented!()
        }
    }

    fn take_population(self) -> Vec<VectorChromosome<'pop>> {
        self.population.unwrap()
    }
}
