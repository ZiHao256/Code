use std::slice;

fn main() {
    let mut slice = vec![1,2,3,4];

    let (a,b) = slice.split_at_mut(2);

    println!("{:?} {:?}",a,b);

    let (a,b) = split_at_mut(&mut slice, 2);

    dbg!(a,b);

}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let l = slice.len();

    let s = slice.as_mut_ptr();

    assert!(mid <= l);

    unsafe{
        (
            slice::from_raw_parts_mut(s, mid),
            slice::from_raw_parts_mut(s.add(mid), l-mid)
        )
    }

}
