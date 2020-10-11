pub fn source_merge_sort<T: PartialOrd>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }
    let b = v.split_off(v.len() / 2);
    let a = source_merge_sort(v);
    let b = source_merge_sort(b);
    let mut res = Vec::new();
    let mut b_it = b.into_iter();
    let mut a_it = a.into_iter();
    let mut a_peak = a_it.next();
    let mut b_peak = b_it.next();
    loop {
        match a_peak {
            Some(ref a_val) => match b_peak {
                Some(ref b_val) => {
                    if b_val < a_val {
                        // REDUNDANT: all the take() invocations are useless.
                        //
                        res.push(b_peak.take().unwrap());
                        b_peak = b_it.next();
                    } else {
                        res.push(a_peak.take().unwrap());
                        a_peak = a_it.next();
                    }
                }
                None => {
                    res.push(a_peak.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peak {
                    res.push(b_val)
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}
