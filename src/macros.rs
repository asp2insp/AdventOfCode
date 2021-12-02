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