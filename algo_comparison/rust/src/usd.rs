#![allow(unused)]

//use std::vec;
use std::collections::BTreeMap;
use rand::Rng;

struct Agent{   //struct thats holds an agent
    opinion: usize,  //and his opinion
    bias: f32
}

struct Population {     //runtime algorithm storage
    agents: Vec<Agent>,   //vec of all agents
    opinions: BTreeMap<usize,usize>,  //Hasmap that holds how many agents have which opinion
    steps: usize,
    finished: bool,
    max_steps: usize,
    agent_count: usize
}

impl Population {
    fn setup(init: Vec<usize>,max_steps: usize) -> Self {
        let mut agents = Vec::new();
        let mut opinions = BTreeMap::new();
        opinions.insert(0, 0);
        let mut opinion = 1;

        for init_agents in init{
            for _ in 0..init_agents{
                agents.push(Agent {opinion,bias: 1.0});
            }
            opinions.insert(opinion, init_agents);
            opinion += 1;
        }
        
        let agent_count = agents.len();

        Population { agents, opinions, steps:0, finished:false, max_steps, agent_count }
    }

    fn get_opinion(&self,id:usize) -> (usize,f32){
        let x = &self.agents[id];
        return (x.opinion,x.bias);
    }

    fn get_agent(&mut self,id:usize) -> &mut Agent{
        return &mut self.agents[id];
    }

    fn step(&mut self){
        let mut rng = rand::thread_rng();
        let max = self.agents.len();

        let initiator_id = rng.gen_range(0..max);
        let mut responder_id = rng.gen_range(0..max);

        while initiator_id == responder_id {
            responder_id = rng.gen_range(0..max);
        }

        self.transition(initiator_id, responder_id);

        self.steps += 1;
        self.check_finished();
    }

    fn check_finished(&mut self){
        for n in &self.opinions{
            if n.1 >= &self.agent_count {
                self.finished = true;
                return;
            }
        }
        if self.steps >= self.max_steps {
            self.finished = true;
            return;
        }
    }

    fn transition(&mut self,initiator_id:usize,responder_id:usize){
        let (initiator_opinion, initiator_bias) = self.get_opinion(initiator_id);
        let (responder_opinion, responder_bias) = self.get_opinion(responder_id);

        if initiator_opinion == 0 && responder_opinion != 0{
            self.update_opinion(initiator_id, responder_opinion);

        }else if initiator_opinion != responder_opinion && responder_opinion != 0 {
            self.update_opinion(initiator_id, 0);
        }
    }

    fn update_opinion(&mut self, agent_id :usize ,new_opinion :usize){
        let old_opinion = self.get_opinion(agent_id);
        self.get_agent(agent_id).opinion = new_opinion;
        *self.opinions.entry(new_opinion).or_insert(0) += 1;
        *self.opinions.entry(old_opinion.0).or_insert(0) -= 1;
    }
 
    fn print(&self){
        println!("{} {:?}",self.steps, self.opinions);
    }

    fn run(&mut self){
        while !self.finished {
            self.step();
        }

    }
}

pub fn run(init: Vec<usize>,max:usize) -> usize{
    let mut alg = Population::setup(init,max);
    alg.print();
    alg.run();
    alg.print();
    return alg.steps;
}

fn random(bias:f32) -> bool {
    let mut rng = rand::thread_rng();
    let random_value: f32 = rng.gen(); // Generate a random value between 0.0 and 1.0

    random_value < bias // Return true if the random value is less than 0.6
}