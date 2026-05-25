fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let v_iter = v.iter_mut();
    for num in v_iter {
        *num += 1; // modifying the vector.
        print!("{}  ", num);
    }

    for num in v.iter() {
        print!(" --- {} --- ", *num);
    }

    println!("{:?}", v);
}
