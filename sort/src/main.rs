// insert sort

fn main() {
    let mut v1 = vec![0, 1, 4, 3, 7, 5, 7];

    for i in 2..v1.len(){
        dbg!(&v1[i]);

        let t = v1[i];

        let mut j = i-1;

        v1.remove(i);

        while(t < v1[j]){
            j -= 1;
        }

        v1.insert(j+1, t);

    }

    dbg!(&v1);



}
