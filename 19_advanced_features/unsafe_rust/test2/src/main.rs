fn main() {
    let mut s = vec![1,2,3];

    let x = &mut s;

    let (z,y) = s.split_at_mut(2);
    // let (a,b) = x.split_at_mut(2);

    assert_eq!(z, &mut vec![1,2]);
}
