#![allow(unused)]

mod algorithm_fast;
mod test;
use std::{time::Instant, thread, io::{self, BufWriter}, fs::File};
use incr_stats::incr::Stats;
use polars::export::{rayon::result, num::Pow};




fn fast() {
    let now = Instant::now();

    let agents = 1000000;
    let opinions = 1000;

    let mut initial = even_distributed(agents, opinions);
    let mut bias1 = vec![0.0;opinions+1];  // p1 das u,x -> x,x
    let mut bias2 = vec![0.;opinions+1];   // p2 das x,y -> u,y
    let mut bias3 = vec![0.;opinions+1];   // p3 das x,u -> u,u
    let mut bias4 = vec![0.;opinions+1];
    let mut bias5 = vec![0.;opinions+1];
    bias3[1] = 0.0;
    algorithm_fast::run(initial,Some(bias1),Some(bias2),Some(bias3),Some(bias4),Some(bias5));
    println!("Finished in {} msec", now.elapsed().as_millis());
}



fn run_test_2(){
    let agentslist:Vec<usize> = (8..19).map(|x:u32| 2_usize.pow(x)).collect();
    let opinionslist:Vec<usize> = (1..11).map(|x:u32| 2_usize.pow(x)).collect();
    let agents = 100000;
    let opinions = 5;
    let threads = 20;
    println!("Agents: {:?}, Opinions: {:?}",agentslist,opinionslist);

    let file = File::create(format!("csv2/test2-b5-{opinions}o.csv")).unwrap();
    let mut buffered_file = BufWriter::new(file);
    let mut writer = csv::Writer::from_writer(buffered_file);
    writer.write_record(&["Opinions","lower","upper"]).unwrap();


    for a in opinionslist{  //change
        let mut initial = even_distributed(agents, a+1);  //change
        initial[1] = initial[1]+initial.pop().unwrap();  //change
        println!("Initial: {:?}",initial);

        let mut children = vec![];
        for i in 0..threads{
            children.push(thread::spawn({
                let initial = initial.clone(); 
                move || {
                    return run_until_fail(initial,100); //change
                }
            }));
        }

        let mut s = Stats::new();
        let mut s2 = Stats::new();

        for i in children {
            let result = i.join().unwrap();
            println!("Result: {:?}",result);
            s.update(result.0.into());
            s2.update(result.1.into());
        }
        println!("Opinions/Agents: {}, Bias: {:?}",a,s.mean().unwrap());
        writer.write_record(&[a.to_string(),s.mean().unwrap().to_string(),s2.mean().unwrap().to_string()]);
    }
    writer.flush();
}


fn run_until_fail(initial:Vec<usize>,needed_count: usize)->(f64,f64){
    let mut finished = false;
    let mut bias_v = vec![0.;initial.len()];
    let mut bias = 0.;
    let mut count = 0;
    let mut first = 0.;
    let mut f = false;

    while !finished {
        bias_v[1] = bias/100.0;
        let result = algorithm_fast::run(initial.clone(),None,None,None,None,Some(bias_v.clone())); //change
        //println!("{}",result.1);
        //println!("{},{}",result.1,bias/100.);
        if result.1 != 1 { //change
            count+=1;
            if f == false{
                first = bias;
                f = true;
            }
            if count >= needed_count{
                finished = true;
                break;
            }
        }
        else{
            count = 0;
            bias = bias + 1.;
        }
        if bias > 100.{
            finished = true;
            break;
        }
        
    }
    (first/100.,bias/100.)
}

fn run_until_fail2(initial:Vec<usize>)->f64{
    let mut finished = false;
    let mut bias_v = vec![0.;initial.len()];
    let mut bias = 0.;
    
    while !finished {  
        bias_v[1] = bias/100.;    
        let result = algorithm_fast::run(initial.clone(),None,None,None,None,None);
        println!("{}",result.1);
        //println!("{},{}",result.1,bias/100.);
        if result.1 != 1 || result.0 >= 500000000{
            
            finished = true;
            break;
        }
        //bias = bias + 1.;
    }
    bias/100.
}


fn run_test_1(){
    let agents = 10000;
    let opinions = 100;
    let threads = 100;

    let mut initial = even_distributed(agents, opinions);
    let mut bias1 = vec![0.;opinions+1];  // p1 das u,x -> x,x
    let mut bias2 = vec![0.;opinions+1];   // p2 das x,y -> u,y
    let mut bias3 = vec![0.;opinions+1];   // p3 das x,u -> u,u
    let mut bias4 = vec![0.;opinions+1];
    let mut bias5 = vec![0.;opinions+1];

    //let test = vec![0.,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.81,0.82,0.83,0.84,0.85,0.86,0.87,0.88,0.89,0.9,0.91,0.92,0.93,0.94,0.95,0.96,0.97,0.98,0.99,1.];
    let test = generate_floats(100);

    

    let testname = "b4";
    let file = File::create(format!("csv3/test2-{testname}-{agents}a{opinions}o.csv")).unwrap();
    let buffered_file = BufWriter::new(file);
    let mut writer = csv::Writer::from_writer(buffered_file);
    writer.write_record(["bias","avg","max","min","undecided","opinion1"]);
    let total = Instant::now();

    for v in test {
        
        //bias3 = vec![v;opinions+1];
        match testname{
            "b1" => bias1[1] = v,
            "b2" => bias2[1] = v,
            "b3" => bias3[1] = v,
            "b4" => bias4[1] = v,
            "b5" => bias5[1] = v,
            _ => panic!("Unknown testname")
        };

        //bias5[1] = v;
        let mut s = Stats::new();
        let mut children = vec![];

        let mut undecided_win_counter = 0;
        let mut opinion1_win_counter = 0;

        let now = Instant::now();
        

        for i in 0..threads{
            children.push(thread::spawn( {
                let initial = initial.clone();
                let bias1 = bias1.clone();
                let bias2 = bias2.clone();
                let bias3 = bias3.clone();
                let bias4 = bias4.clone();
                let bias5 = bias5.clone();
                
                move || {
                let result = algorithm_fast::run(initial,Some(bias1),Some(bias2),Some(bias3),Some(bias4),Some(bias5));
                println!("Testrun Nr {}: {} steps. Winner: {}",i+1,result.0,result.1);
                result
        }}));
            
        }

        for i in children {
            let result = i.join().unwrap();
            s.update(result.0 as f64);
            if result.1 == 0 || result.0 >= 50000000{
                undecided_win_counter += 1;
            }
            else if result.1 == 1{
                opinion1_win_counter += 1;
            }
        }
        writer.write_record([v.to_string(),s.mean().unwrap().to_string(),s.max().unwrap().to_string(),s.min().unwrap().to_string(),undecided_win_counter.to_string(),opinion1_win_counter.to_string()]);
        println!("Finished with bias {} in avg: {} steps and opinion1 won {} times.Min:{} Max:{}",v,s.mean().unwrap(),opinion1_win_counter,s.min().unwrap(),s.max().unwrap());
        println!("Finished in {} sec", now.elapsed().as_secs_f32());

    }
    println!("Finished total in {} sec", total.elapsed().as_secs_f32());
    writer.flush();
    
}

fn run_test_3(){
    let agents = usize::pow(2,14);
    let opinions = 5;
    let threads = 100;

    // let mut initial = even_distributed(agents, opinions+1);  //200% bias
    // initial[1] = initial[1]+initial.pop().unwrap();

    // let mut initial = even_distributed(agents, opinions*2-1); //200% bias for alll except op1
    // for i in 2..opinions+1{
    //     initial[i] = initial[i]+initial.pop().unwrap();
    // }

    let mut initial = even_distributed(agents, opinions+1);  //200% bias
    initial[2] = initial[1]+initial.pop().unwrap();

    let mut bias1 = vec![0.;opinions+1];  // p1 das u,x -> x,x
    let mut bias2 = vec![0.;opinions+1];   // p2 das x,y -> u,y
    let mut bias3 = vec![0.;opinions+1];   // p3 das x,u -> u,u
    let mut bias4 = vec![0.;opinions+1];
    let mut bias5 = vec![0.;opinions+1];

    //let test = vec![0.,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.81,0.82,0.83,0.84,0.85,0.86,0.87,0.88,0.89,0.9,0.91,0.92,0.93,0.94,0.95,0.96,0.97,0.98,0.99,1.];
    let test = generate_floats(100);

    

    let testname = "b1";
    let file = File::create(format!("csv/test6-{testname}-{agents}a{opinions}o.csv")).unwrap();
    let buffered_file = BufWriter::new(file);
    let mut writer = csv::Writer::from_writer(buffered_file);
    writer.write_record(["bias","avg","max","min","undecided","opinion1"]);
    let total = Instant::now();

    for v in test {
        
        //bias3 = vec![v;opinions+1];
        match testname{
            "b1" => bias1[1] = v,
            "b2" => bias2[1] = v,
            "b3" => bias3[1] = v,
            "b4" => bias4[1] = v,
            "b5" => bias5[1] = v,
            _ => panic!("Unknown testname")
        };

        //bias5[1] = v;
        let mut s = Stats::new();
        let mut children = vec![];

        let mut undecided_win_counter = 0;
        let mut opinion1_win_counter = 0;

        let now = Instant::now();
        

        for i in 0..threads{
            children.push(thread::spawn( {
                let initial = initial.clone();
                let bias1 = bias1.clone();
                let bias2 = bias2.clone();
                let bias3 = bias3.clone();
                let bias4 = bias4.clone();
                let bias5 = bias5.clone();
                
                move || {
                let result = algorithm_fast::run(initial,Some(bias1),Some(bias2),Some(bias3),Some(bias4),Some(bias5));
                //println!("Testrun Nr {}: {} steps. Winner: {}",i+1,result.0,result.1);
                result
        }}));
            
        }

        for i in children {
            let result = i.join().unwrap();
            s.update(result.0 as f64);
            if result.1 == 0 || result.0 >= 500000000{
                undecided_win_counter += 1;
            }
            else if result.1 == 1{
                opinion1_win_counter += 1;
            }
        }
        writer.write_record([v.to_string(),s.mean().unwrap().to_string(),s.max().unwrap().to_string(),s.min().unwrap().to_string(),undecided_win_counter.to_string(),opinion1_win_counter.to_string()]);
        println!("Finished with bias {} in avg: {} steps and opinion1 won {} times.Min:{} Max:{}",v,s.mean().unwrap(),opinion1_win_counter,s.min().unwrap(),s.max().unwrap());
        println!("Finished in {} sec", now.elapsed().as_secs_f32());

    }
    println!("Finished total in {} sec", total.elapsed().as_secs_f32());
    writer.flush();
    
}

fn main() {
    //run_test_3();
    //run_test_2();
    run_test_1();
    //print!("{:#?}",run_until_fail(vec![0,200,100,100,100,100],100));
    //test::test();
    //fast();
    //slow();
    //io::stdin().read_line(&mut String::new()).unwrap();
    //print!("{}",run_until_fail2(vec![0,200,100,100,100]));
}

fn even_distributed(ammount: usize,opinions: usize) -> Vec<usize>{
    let num_per_opinion = ammount / opinions;
    let mut v = Vec::new();
    v.push(0);
    for _ in 0..opinions {
        v.push(num_per_opinion);
    }
    v
}


fn create_vector(step: f32) -> Vec<f32> {
    let mut vector: Vec<f32> = Vec::new();
    let mut num = 0.;

    while num <= 1. {
        vector.push(num);
        num += step;
    }

    vector
}

fn generate_floats(step: usize) -> Vec<f64> {
    (0..=step)
        .step_by(1)
        .map(|i| i as f64 / step as f64)
        .collect()
}