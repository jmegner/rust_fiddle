use itertools;

fn main() {
    let origs = [6, 3, 5, 8, 6, 34, 1, 5, 7];

    for sublen in 0..origs.len() {
        let unsorted_vals = &origs[..sublen];
        let mut sorted_vals = unsorted_vals.to_vec();
        quick_sort(&mut sorted_vals);

        println!("{:?} => {:?}", unsorted_vals, sorted_vals);

        let oracle_sorted_vals =
            itertools::sorted(unsorted_vals.iter().cloned()).collect::<Vec<_>>();
        assert_eq!(oracle_sorted_vals, sorted_vals);
    }
}

fn quick_sort<T: PartialOrd>(vals: &mut [T]) {
    if vals.len() > 1 {
        let boundary_idx = partition(vals);
        quick_sort(&mut vals[..boundary_idx]);
        quick_sort(&mut vals[boundary_idx + 1..]);
    }
}

fn partition<T: PartialOrd>(vals: &mut [T]) -> usize {
    let pivot_idx = vals.len() - 1;
    let mut boundary_idx = 0;

    for i in 0..pivot_idx {
        if vals[i] < vals[pivot_idx] {
            vals.swap(i, boundary_idx);
            boundary_idx += 1;
        }
    }

    vals.swap(boundary_idx, pivot_idx);
    boundary_idx
}
