use std::fmt::Debug;
use std;

pub fn get_mut_pair<T: Debug>(data: &mut [T], idx1: usize, idx2: usize) -> (&mut T, &mut T) {
    if idx1 < idx2 {
        //idx1 is first
        let (first, second) = data.split_at_mut(idx1 + 1);
        let first_len: usize = first.len();
        (&mut first[first_len - 1], &mut second[idx2 - idx1 - 1])
    } else {
        //idx2 is first
        let (first, second) = data.split_at_mut(idx2 + 1);
        let first_len: usize = first.len();
        (&mut second[idx1 - idx2 - 1], &mut first[first_len - 1])
    }
}
#[test]
fn get_mut_pair_test() {
    let mut x = vec![1, 2, 3, 4];

    let (a, b) = get_mut_pair(&mut x, 1, 2);
    assert_eq!(*a, 2);
    assert_eq!(*b, 3);
}
pub fn get_mut_triplet<T: Debug>(data: &mut Vec<T>, idx1: usize, idx2: usize, idx3: usize) -> (&mut T, &mut T, &mut T) {
    let mut sorted = [(idx1, 0), (idx2, 1), (idx3, 2)];
    sorted.sort();
    let ax = sorted[0];
    let bx = sorted[1];
    let cx = sorted[2];
    let mut resort = [0usize, 0usize, 0usize];

    for (no, item) in sorted.iter().enumerate() {
        resort[item.1] = no;
    }

    let (first, intermediate) = data.split_at_mut(ax.0 + 1);
    let (second, third) = intermediate.split_at_mut(bx.0 - first.len() + 1);

    let firstlen = first.len();
    let secondlen = second.len();
    let mut p1 = &mut first[firstlen - 1];
    let mut p2 = &mut second[secondlen - 1];
    let mut p3 = &mut third[cx.0 - bx.0 - 1];

    if resort[0] != 0 {
        let swapwith = resort[0];
        std::mem::swap(&mut p1, if swapwith == 1 { &mut p2 } else { &mut p3 });
        resort[ax.1] = swapwith;
    }
    if resort[1] != 1 {
        std::mem::swap(&mut p2, &mut p3);
    }

    (p1, p2, p3)
}

#[test]
fn get_mut_triplet_test() {
    let mut rng = thread_rng();

    for x in 0..100 {
        let mut testvec = Vec::new();
        for i in 0..rng.gen_range(3, 20) {
            testvec.push(i);
        }
        for _ in 0..10 {
            let a = rng.gen_range(0, testvec.len());
            let b = rng.gen_range(0, testvec.len());
            let c = rng.gen_range(0, testvec.len());
            if a != b && a != c && b != c {
                let r = get_mut_triplet(&mut testvec, a, b, c);
                println!("I: {} {} {} R: {:?}", a, b, c, r);
                assert!(*r.0 == a);
                assert!(*r.1 == b);
                assert!(*r.2 == c);
            }
        }
    }
    {
        let mut testvec = vec!["a", "b", "c"];
        let trip = get_mut_triplet(&mut testvec, 0, 1, 2);
        assert_eq!(trip.0, &"a");
        assert_eq!(trip.1, &"b");
        assert_eq!(trip.2, &"c");
    }
    {
        let mut testvec = vec!["a", "b", "c"];
        let trip = get_mut_triplet(&mut testvec, 0, 2, 1);
        assert_eq!(trip.0, &"a");
        assert_eq!(trip.1, &"c");
        assert_eq!(trip.2, &"b");
    }
    {
        let mut testvec = vec!["a", "b", "c"];
        let trip = get_mut_triplet(&mut testvec, 2, 1, 0);
        assert_eq!(trip.0, &"c");
        assert_eq!(trip.1, &"b");
        assert_eq!(trip.2, &"a");
    }
    {
        let mut testvec = vec!["a", "x", "b", "c"];
        let trip = get_mut_triplet(&mut testvec, 3, 2, 0);
        assert_eq!(trip.0, &"c");
        assert_eq!(trip.1, &"b");
        assert_eq!(trip.2, &"a");
    }
    {
        let mut testvec = vec!["a", "b", "c"];
        let trip = get_mut_triplet(&mut testvec, 1, 0, 2);
        assert_eq!(trip.0, &"b");
        assert_eq!(trip.1, &"a");
        assert_eq!(trip.2, &"c");
    }
    {
        let mut testvec = vec!["a", "b", "c"];
        let trip = get_mut_triplet(&mut testvec, 1, 2, 0);
        assert_eq!(trip.0, &"b");
        assert_eq!(trip.1, &"c");
        assert_eq!(trip.2, &"a");
    }
    {
        let mut testvec = vec!["a", "b", "c"]; //Correct: c, a, b = 2, 0, 1
        let trip = get_mut_triplet(&mut testvec, 2, 0, 1);
        assert_eq!(trip.0, &"c");
        assert_eq!(trip.1, &"a");
        assert_eq!(trip.2, &"b");
    }
}
