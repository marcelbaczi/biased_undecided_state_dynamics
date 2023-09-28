#![allow(unused)]

use std::{fs::File, io::BufWriter};

use csv::Writer;
use dynamic_weighted_index::DynamicWeightedIndex;
use rand::Rng;

const UNDECIDED:usize = 0;

struct Population {
    dyn_idx:DynamicWeightedIndex<usize>,
    opinions:usize,
    agents:usize,
    finished: bool,
    steps: usize,
    writer: Option<Writer<BufWriter<File>>>,
    bias1: Vec<f64>,
    bias2: Vec<f64>,
    bias3: Vec<f64>,
    bias4: Vec<f64>,
    bias5: Vec<f64>,
    rng: rand::rngs::ThreadRng,
}

struct Opinion{
    id: usize,
    number: usize,
}

impl Population {
    fn setup(opinion_dist:Vec<usize>,
        bias1:Vec<f64>,
        bias2:Vec<f64>,
        bias3:Vec<f64>,
        bias4:Vec<f64>,
        bias5:Vec<f64>
    ) -> Self {

        let opinions = opinion_dist.len();
        let mut dyn_idx:DynamicWeightedIndex<usize>;
        dyn_idx = DynamicWeightedIndex::new(opinions);

        for opinion in 0..opinion_dist.len() {
            let weight = *opinion_dist.get(opinion).unwrap();
            dyn_idx.set_weight(opinion, weight);
        }
        let agents = dyn_idx.total_weight();
        //println!("Agents: {},Opinions {},opinion_dist:{:?}",agents,opinions,opinion_dist);
        Population { dyn_idx,opinions,agents,
            finished:false,steps:0,writer:None,
            bias1,bias2,bias3,bias4,bias5,rng:rand::thread_rng()}
    }



    fn step(&mut self){
        let initiator = self.get_opinion();
        let responder = self.get_opinion();
        self.steps += 1;

        self.bias_usd(initiator, responder);
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




    
    fn bias_usd(&mut self, initiator: Opinion, responder: Opinion) {
        let i = initiator.id;
        let r = responder.id;
        let bs = i;

        if(i != r && i != UNDECIDED && r != UNDECIDED){  //bias 1 und bias 2
            if random(self.bias1.get(bs).unwrap()) {
                self.update_opinion(initiator, responder);
            }else if random(self.bias2.get(bs).unwrap()) {
                // Do Nothing
            }else{
                self.make_undecided(initiator);
            }
        }else if i == UNDECIDED && r != UNDECIDED {   //bias 3
            if random(self.bias3.get(bs).unwrap()) {
                // Do Nothing
            }else{
                self.update_opinion(initiator, responder);
            }
        }else if i == r && i != UNDECIDED { //bias 4
            if random(self.bias4.get(bs).unwrap()) {
                self.make_undecided(initiator);
            }  
        }else if i != UNDECIDED && r == UNDECIDED { //bias 5
            if random(self.bias5.get(bs).unwrap()) {
                self.make_undecided(initiator);
            }
        }
    }

    fn update_opinion(&mut self, old_opinion: Opinion, new_opinion:Opinion)->usize{
        self.dyn_idx.set_weight(old_opinion.id, old_opinion.number-1);
        self.dyn_idx.set_weight(new_opinion.id, new_opinion.number+1);
        if (new_opinion.number+1) >= self.agents || self.steps >= 50000000{
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
        self.csv_write_header();
        self.csv_write();
        while self.finished == false {
            self.step();
            if self.steps%(self.agents/10) == 0{
                //self.print();
                self.csv_write();
            }
        }
        self.csv_write();
        if let Some(mut writer) = self.writer.as_mut() {
            let _ = writer.flush();
        }
        //println!("Agents: {},Opinions {},opinion_dist:{:?}",self.agents,self.opinions,self.dyn_idx.total_weight());
        (self.steps,self.get_winner())

    }

    pub fn enable_csv(&mut self, filename:&str){
        let file = File::create(format!("{}.csv",filename)).unwrap();
        let buffered_file = BufWriter::new(file);
        let writer = csv::Writer::from_writer(buffered_file);
        self.writer = Some(writer);
    }

    fn get_winner(&self)->usize{
        let mut rng = rand::thread_rng();
        let result = self.dyn_idx.sample_index_and_weight(&mut rng).unwrap();
        result.index
    }

    fn csv_write_header(&mut self){
        match self.writer {
            Some(ref mut writer) => {
                let _ = writer.write_field("usd/Steps");
                let _ = writer.write_field("usd/opinions/Undecided");
                for i in 1..self.opinions {
                    let _ = writer.write_field(format!("usd/opinions/Opinion {}",i));
                }
                let _ = writer.write_record(None::<&[u8]>);
            },
            None => (),
        }
    }

    fn csv_write(&mut self){
        if let Some(mut writer) = self.writer.take() {
                let rec = self.to_array();
                let _ = writer.serialize(rec);
                self.writer = Some(writer);
        }
    }

    fn to_array(& self)->Vec<usize>{
        let mut vec:Vec<usize> = Vec::new();
        vec.push(self.steps);
        for i in 0..self.opinions {
            let s = self.dyn_idx.weight(i);
            vec.push(s);
        }
        vec
    }

    fn print(&mut self){
        for o in 0..self.opinions {
            print!("{:>3}: {:>7}  ",o,self.dyn_idx.weight(o));
        }
        println!(" ");
    }

}

pub fn run(opinion_dist:Vec<usize>,bias1:Option<Vec<f64>>,bias2:Option<Vec<f64>>,bias3:Option<Vec<f64>>,bias4:Option<Vec<f64>>,bias5:Option<Vec<f64>>)-> (usize,usize){
    let opinions = opinion_dist.len();
    let bias1 = bias1.unwrap_or(vec![0.;opinions]);
    let bias2 = bias2.unwrap_or(vec![0.;opinions]);
    let bias3 = bias3.unwrap_or(vec![0.;opinions]);
    let bias4 = bias4.unwrap_or(vec![0.;opinions]);
    let bias5 = bias5.unwrap_or(vec![0.;opinions]);

    let mut alg = Population::setup(opinion_dist,bias1,bias2,bias3,bias4,bias5);
    alg.enable_csv("test");
    //alg.print();
    let result = alg.run();
    //alg.print();
    //println!("{}",alg.steps);
    result
}

fn random(bias:&f64) -> bool {
    let mut rng = rand::thread_rng();
    let random_value: f64 = rng.gen(); // Generate a random value between 0.0 and 1.0

    random_value < *bias // Return true if the random value is less than bias
}
