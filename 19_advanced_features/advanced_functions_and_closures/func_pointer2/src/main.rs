#[derive(Debug)]
enum Status{
    Value(u32)
}

fn main() {
    let x:Vec<Status> = (0u32..10).map(Status::Value).collect();

    dbg!(x);
}
