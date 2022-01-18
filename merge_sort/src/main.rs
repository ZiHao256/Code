fn main() {

    let mut v = vec![-1, 5, 2, 4, 7, 1, 3, 2, 6];

    dbg!(&v);

    merge_sort(&mut v, 1, 8);

    dbg!(&v);


}

fn merge_sort(v: &mut Vec<i32>, first: usize, last: usize){
    if(first<last){
        let mid = (first+last)/2;

        merge_sort(v, first, mid);

        merge_sort(v, mid+1, last);

        merge(v, first, mid, last);

    }
}

fn merge(v: &mut Vec<i32>, first:usize, mid:usize, last: usize){
    let n1 = mid-first+1;
    let n2 = last-mid;

    let v2 = v.clone();
    let left = &v2[first..mid+1];
    let right = &v2[mid+1..last+1];

    let mut i:usize = 0;
    let mut j:usize = 0;

    for k in first..last+1{
        if(left[i] < right[j]){
            v[k] = left[i];
            i += 1;
            if i==n1{
                for x in k..last+1{
                    v[x] = right[j];
                    j += 1;
                }
            }
        }else{
            v[k] = right[j];
            j += 1;
            if j==n2{
                for x in k..last+1{
                    v[x] = left[i];
                    i += 1;
                }
            }
        }

    }

}
