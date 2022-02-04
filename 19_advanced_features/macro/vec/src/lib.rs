#[macro_export]
macro_rules! vec {
    ($($x:expr),*) => {
        let mut vec_tem = Vec::new();
        $(
            vec_tem.push($x);

        )*

        vec_tem
    };
}
