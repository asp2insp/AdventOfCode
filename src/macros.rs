#[macro_export]
macro_rules! veci {
    ( $( $e:expr, if $b:expr),*$(,)* ) => ({
        vec![
            $(
                if $b {Some($e)} else {None}
            ),*
        ].into_iter()
        .flat_map(|a| a)
        .collect()
    })
}

#[macro_export]
macro_rules! parse {
    ( $e:expr , $t:ty) => {
        ($e).parse::<$t>().unwrap()
    };
    ( $e:expr ) => {
        ($e).parse().unwrap()
    };
}

#[macro_export]
macro_rules! ppush {
    ($v:expr, $arg:expr) => {
        ($v).last_mut().unwrap().push($arg)
    };
}

#[macro_export]
macro_rules! makeset {
    ( $( $e:expr),*$(,)* ) => {
        {
            let mut set = std::collections::HashSet::new();
            $(
                set.insert($e);
            )*
            set
        }
    };
}

macro_rules! dict {
    ($($i:expr => $e:expr),* $(,)?) => {
        {
            use std::collections::HashMap;
            let mut m = HashMap::new();
            $(
                m.insert($i, $e);
            )*
            m
        }
    };
}

#[macro_export]
macro_rules! gimme_nums {
    ( $e:expr ) => {
        {
            use regex::*;
            let re = Regex::new(r"([-\d]+)([^-\d]*)").unwrap();
            ($e).lines().map(|l| {
                re.captures_iter(l.trim()).map(|c| parse!(c[1], isize)).collect::<Vec<isize>>()
            }).collect::<Vec<Vec<isize>>>()
        }
    };
}

#[test]
fn test_nums() {
    assert_eq!(vec![vec![1, 2, -3, 4]], gimme_nums!("  1,2  , -3 | 4"));
}

#[test]
fn test_dict() {
    use itertools::*;
    use std::collections::HashMap;
    assert_eq!((0..3).zip(['a', 'b', 'c']).collect::<HashMap<usize, char>>(), dict!{0 => 'a', 1 => 'b', 2 => 'c'});
}