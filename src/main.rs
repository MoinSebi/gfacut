use gfaR::{Gfa, readGFA, Node};
use std::env;
use std::collections::{HashSet, HashMap};


pub fn checkUnique(graph: &Gfa, node: &String) -> bool{
    for x in graph.paths.iter(){
        let mut u = 0;
        for y in x.nodes.iter(){
            if y == node{
                u += 1;
            }

        }
        if u > 1{
            false;
        }
    }
    true
}

pub fn cutout(graph: &Gfa, node: &String, lens: usize) -> (HashSet<String>, HashMap<String, (usize, usize)>){
    let mut h: HashSet<String> = HashSet::new();
    let mut h2: HashMap<String, (usize, usize)>  =  HashMap::new();
    for x in graph.paths.iter(){
        let mut ind = usize::MAX;
        for (i, y) in  x.nodes.iter().enumerate(){
            if y == node{
                ind = i;
            }
        }
        if ind != usize::MAX {
            let mut min = 0;
            let mut maxx = 0;
            //println!("{}", "\n");
            //println!("{}", ind);
            if (ind < lens) & ((ind + lens) > x.nodes.iter().len()){
                println!("{}", 1);
                maxx = x.nodes.iter().len();
                min = 0;
            } else if ind < lens{
                //println!("{}", 2);
                min = 0;
                maxx = ind + lens;
            } else if (ind + lens) > x.nodes.iter().len() {
                //println!("{}", 3);
                min = ind - lens;
                maxx = x.nodes.iter().len();
            } else{
                min = ind - lens;
                maxx = ind + lens;
            }

            for ii in min..maxx{
                h.insert(x.nodes[ii].to_owned());
            }

            h2.insert(x.name.to_owned(), (min, maxx));


        }
        //println!("{}", h.len());
        //println!("{:?}", h)
    }
    (h, h2)

}

pub fn sortout(graph: &Gfa, nodes: &HashSet<String>, fromto: &HashMap<String, (usize, usize)>){
    let mut n: HashMap<&String, &Node> = HashMap::new();
    for x in graph.nodes.iter(){
        if nodes.contains(x.0){
            println!("{}\t{}\t{}", "S", x.0, x.1.seq);
        }
    }
    for x in graph.edges.iter(){
        if nodes.contains(&x.to) & nodes.contains(&x.from){
            println!("{}\t{}\t{}\t{}\t{}\t{}", "L", x.from, if x.from_dir {"+"} else {"-"} , x.to, if x.to_dir {"+"} else {"-"}, "0M")
        }
    }

    for x in graph.paths.iter(){
        //println!("{}", x.name);
        //println!("{:?}", fromto.keys());
        if fromto.contains_key(&x.name){
            print!("{}\t{}\t", "P", x.name);
            for y in fromto[&x.name].0..fromto[&x.name].1{;
                print!("{}{}", x.nodes[y], if x.dir[y] {"+"} else {"-"}, );
                if y != fromto[&x.name].1 -1{
                    print!("{}", ",")
                }
            }
            print!("{}", "\t");
            for y in fromto[&x.name].0..fromto[&x.name].1-1{;
                print!("{}","*");
                if y != fromto[&x.name].1 -2{
                    print!("{}", ",")
                }
            }
            print!("{}", "\n")
        }
    }

}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        eprintln!("{}", "ERROR! Not enogh parameters!");
        eprintln!("{}", "Usage: gfacut path/to/graph.gfa node_id")
    } else {
        eprintln!("Graph: {:?}", args[1]);
        eprintln!("Node: {}", args[2]);
        let graph = readGFA(&args[1]);
        if checkUnique(&graph, &args[2]){
            let (u, u2) = cutout(&graph, &"40".to_owned(), 100);
            println!("{}\t{}", "H","VN:Z:1.0");
            sortout(&graph, &u, &u2);
        } else{
            eprintln!("{}", "The selected not is not unique.")
        }
    }



}
