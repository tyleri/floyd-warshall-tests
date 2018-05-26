use std::io;

fn read_parse_line() -> Vec<usize> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("could not read line");

    let split_line = input_line.split_whitespace()
                                   .map(|x| x.parse::<usize>().unwrap())
                                   .collect();

    split_line
}

fn main() {
    let mut split_line : Vec<usize>;

    split_line = read_parse_line();
    let (n, e) = (split_line[0], split_line[1]);

    let mut edges : Vec<Vec<i32>> = Vec::with_capacity(n);
    for i in 0..n {
        edges.push(Vec::with_capacity(n));
        for _ in 0..n {
            edges.get_mut(i as usize).unwrap().push(-1);
        }
    }

    for _ in 0..e {
        split_line = read_parse_line();
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

    split_line = read_parse_line();
    let trials = split_line[0];

    for _ in 0..trials {
        split_line = read_parse_line();
        let (start, end) = (split_line[0], split_line[1]);
        println!("{}", edges[start-1][end-1]);
    }
}