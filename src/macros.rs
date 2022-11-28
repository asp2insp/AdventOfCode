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
            let mut set = fnv::FnvHashSet::default();
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
            use fnv::FnvHashMap;
            let mut m = FnvHashMap::default();
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
    use fnv::FnvHashMap;
    assert_eq!((0..3).zip(['a', 'b', 'c']).collect::<FnvHashMap<usize, char>>(), dict!{0 => 'a', 1 => 'b', 2 => 'c'});
}