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

#[test]
fn test_dict() {
    use itertools::*;
    use std::collections::HashMap;
    assert_eq!((0..3).zip(['a', 'b', 'c']).collect::<HashMap<usize, char>>(), dict!{0 => 'a', 1 => 'b', 2 => 'c'});
}