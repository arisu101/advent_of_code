use std::collections::{HashMap, HashSet};
use std::io::{self};

fn main() -> io::Result<()> {
    let infile = "inputs.txt";

    let contents = std::fs::read_to_string(infile)?;

    let parts: Vec<&str> = contents.split("\n\n").collect();
    let edges = parts[0];
    let queries = parts[1];

    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in edges.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[1].parse().unwrap();

        graph.entry(y).or_insert_with(HashSet::new).insert(x);
    }
    let mut p1 = 0;
    for query in queries.lines() {
        let vs: Vec<i32> = query.split(',').map(|x| x.parse().unwrap()).collect();
        let mut ok = true;

        for i in 0..vs.len() {
            for j in (i + 1)..vs.len() {
                if graph
                    .get(&vs[i])
                    .unwrap_or(&HashSet::new())
                    .contains(&vs[j])
                {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }

        if ok {
            p1 += vs[vs.len() / 2];
        }
    }

    dbg!(&p1);
    Ok(())
}
