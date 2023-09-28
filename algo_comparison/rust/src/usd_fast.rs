#![allow(unused)]

use std::{fs::File, io::BufWriter};

use dynamic_weighted_index::DynamicWeightedIndex;
use rand::Rng;

const UNDECIDED:usize = 0;

struct Population {
    dyn_idx:DynamicWeightedIndex<usize>,
    opinions:usize,
    agents:usize,
    finished: bool,
    steps: usize,
    rng: rand::rngs::ThreadRng,
    max:usize,
}

struct Opinion{
    id: usize,
    number: usize,
}

impl Population {
    fn setup(opinion_dist:Vec<usize>
    ) -> Self {

        let opinions = opinion_dist.len();
        let mut dyn_idx:DynamicWeightedIndex<usize>;
        dyn_idx = DynamicWeightedIndex::new(opinions);

        for opinion in 0..opinion_dist.len() {
            let weight = *opinion_dist.get(opinion).unwrap();
            dyn_idx.set_weight(opinion, weight);
        }
        let agents = dyn_idx.total_weight();
        Population { dyn_idx,opinions,agents,
            finished:false,steps:0,rng:rand::thread_rng(),max: 5_000_000_000}
    }



    fn step(&mut self){
        let initiator = self.get_opinion();
        let responder = self.get_opinion();
        self.steps += 1;


        self.basic_usd(initiator, responder);

    }


    fn basic_usd(&mut self, initiator: Opinion, responder: Opinion) {
        let i = initiator.id;
        let r = responder.id;

        if i == UNDECIDED && r != UNDECIDED {
            self.update_opinion(initiator, responder);
        }else if i != r && r != UNDECIDED{
             self.make_undecided(initiator);
        }
    }

    fn update_opinion(&mut self, old_opinion: Opinion, new_opinion:Opinion)->usize{
        self.dyn_idx.set_weight(old_opinion.id, old_opinion.number-1);
        self.dyn_idx.set_weight(new_opinion.id, new_opinion.number+1);
        if (new_opinion.number+1) >= self.agents || self.steps >= self.max{
            self.finished = true;
        }
        new_opinion.number+1
    }

    fn make_undecided(&mut self, old_opinion: Opinion){
        self.update_opinion(old_opinion, self.get_undecided_opinion());
    }

    fn get_undecided_opinion(&self)->Opinion{
        Opinion { id: (UNDECIDED), number: (self.dyn_idx.weight(UNDECIDED)) }
    }

    fn get_opinion(&mut self)->Opinion{
        let result = self.dyn_idx.sample_index_and_weight(&mut self.rng).unwrap();
        Opinion { id: (result.index), number: (result.weight) }
    }

    fn run(&mut self) -> (usize,usize){
    
        while self.finished == false {
            self.step();
        }
        (self.steps,self.get_winner())

    }

    fn get_winner(&self)->usize{
        let mut rng = rand::thread_rng();
        let result = self.dyn_idx.sample_index_and_weight(&mut rng).unwrap();
        result.index
    }
    fn set_max(&mut self, max:usize){
        self.max = max;
    }

}

pub fn run(opinion_dist:Vec<usize>,max:usize)-> (usize,usize){
    let opinions = opinion_dist.len();
    let mut alg = Population::setup(opinion_dist);
    alg.set_max(max);
    let result = alg.run();
    result
}
