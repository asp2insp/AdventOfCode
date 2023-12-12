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
        ($e).parse::<$t>()
            .expect(format!("Failed to parse {}", &$e).as_ref())
    };
    ( $e:expr ) => {
        ($e).parse()
            .expect(format!("Failed to parse {}", &$e).as_ref())
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

#[macro_export]
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

#[macro_export]
macro_rules! make_ord {
    ($t:ty, |$this:ident, $other:ident| $body:expr) => {
        impl std::cmp::PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl std::cmp::Ord for $t {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                (|$this: &Self, $other: &Self| $body)(self, other)
            }
        }
    };
}

#[test]
fn test_dict() {
    use fnv::FnvHashMap;
    use itertools::*;
    assert_eq!(
        (0..3)
            .zip(['a', 'b', 'c'])
            .collect::<FnvHashMap<usize, char>>(),
        dict! {0 => 'a', 1 => 'b', 2 => 'c'}
    );
}
