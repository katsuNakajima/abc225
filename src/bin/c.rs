#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let (n, m) = parse_line!(usize, usize);
    let mut b = Vec::new();
    let mut flg = true;
    for _ in 0..n {
        let x = parse_vec!(i64);
        b.push(x);
    }
    for i in 0..n {
        for j in 0..m {
            if i > 0 {
                flg &= b[i - 1][j] + 7 == b[i][j];
            }
            if j > 0 {
                flg &= b[i][j - 1] + 1 == b[i][j];
                flg &= b[i][j] % 7 != 1;
            }
        }
    }
    if flg {
        println!("Yes");
    } else {
        println!("No");
    }
}
