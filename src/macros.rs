
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
        ($e).parse::<>().unwrap()
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