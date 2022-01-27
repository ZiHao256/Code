use encapsulation::*;

fn main() {
    let mut a = AveragedCollection::new();
    a.add(1);

    println!("{}", a.get_average());

}
