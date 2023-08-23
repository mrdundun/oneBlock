pub fn bubble<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
    for _i in 0..list.len() {
        for j in 0..list.len() - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
    list
}