use std::io;

fn main() {
    let mut input_line : String;
    let mut split_line : Vec<usize>;

    input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("could not read line");

    split_line = input_line.split_whitespace()
                           .map(|x| x.parse::<usize>().unwrap())
                           .collect();

    let (n, e) = (split_line[0], split_line[1]);

    let mut edges : Vec<Vec<i32>> = Vec::with_capacity(n);
    for i in 0..n {
        edges.push(Vec::with_capacity(n));
        for _ in 0..n {
            edges.get_mut(i as usize).unwrap().push(-1);
        }
    }

    for _ in 0..e {
        input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("could not read line");
        split_line = input_line.split_whitespace()
                               .map(|x| x.parse::<usize>().unwrap())
                               .collect();

        let (n1, n2, w) = (split_line[0], split_line[1], split_line[2]);
        edges[n1-1][n2-1] = w as i32;
    }

    for k in 0..n {
        for i in 0..n {
            if edges[i][k] == -1 {
                continue;
            }
            for j in 0..n {
                if edges[k][j] != -1 && (edges[i][j] == -1 || edges[i][k] + edges[k][j] < edges[i][j]) {
                    edges[i][j] = edges[i][k] + edges[k][j];
                }
            }
        }
    }

    input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("could not read line");
    
    let trials : i32 = input_line.trim().parse().unwrap();
    // println!("test");

    for _ in 0..trials {
        input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("could not read line");
        split_line = input_line.split_whitespace()
                               .map(|x| x.parse::<usize>().unwrap())
                               .collect();
        
        let (start, end) = (split_line[0], split_line[1]);
        println!("{}", edges[start-1][end-1]);
    }
}