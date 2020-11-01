pub fn sort<T: Ord>(slice: &mut [T]) {
    let mut i: usize = 0;
    let size = slice.len();
    while i < size {
        up_heap(slice, i);
        i += 1;
    }
    while i > 1 {
        i -= 1;
        slice.swap(0, i);
        down_heap(slice, i);
    }
}

fn up_heap<T: Ord>(slice: &mut [T], n: usize) {
    let mut n = n;
    while n > 0 {
        let parent = (n + 1) / 2 - 1;
        if slice[parent] < slice[n] {
            slice.swap(parent, n);
        } else {
            break;
        }
        n = parent;
    }
}

fn down_heap<T: Ord>(slice: &mut [T], n: usize) {
    let mut m = 0;
    let mut tmp = 0;
    loop {
        let left = (m + 1) * 2 - 1;
        let right = (m + 1) * 2;
        if left >= n {
            break;
        }
        if slice[left] > slice[tmp] {
            tmp = left;
        }
        if right < n && slice[right] > slice[tmp] {
            tmp = right;
        }
        if tmp == m {
            break;
        }
        slice.swap(tmp, m);
        m = tmp;
    }
}
