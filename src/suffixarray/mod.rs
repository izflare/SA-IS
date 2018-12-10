use std::collections::HashMap;

fn bucket_calc(d: &mut HashMap<usize, (usize, usize)>, v: &Vec<usize>) -> () {
    //{{{
    let mut p = v[0]; 
    let mut q = v[0]; 
    let mut z = 0;

    for c in v.iter() {
        let i = match d.get(c) {Some(x) => (*x).1 + 1, None => 1};
        d.insert(*c, (0, i));
        if *c < p {p = *c};
        if *c > q {q = *c};
    }
    for b in p..q+1 {
        let i = match d.get(&b) {Some(x) => (*x).1, None => 0};
        if i > 0 {d.insert(b, (z, i));};
        z += i;
    }
    //}}}
}

// L/S-type of suffixes. 0: s, 1: l, lms: -1.
fn ls_calc(ls: &mut Vec<i8>, lms: &mut Vec<usize>, v: &Vec<usize>) -> () {
    //{{{
    for i in (0..v.len()-1).rev() {
        if v[i] < v[i+1] {ls[i] = 0;}
        else if v[i] > v[i+1] {ls[i] = 1; if ls[i+1] == 0 {ls[i+1] = -1; lms.push(i+1)};}
        else {ls[i] = ls[i+1];}
    }
    //}}}
}

// SA-IS
pub fn create(v: &Vec<usize>, sa: &mut Vec<Option<usize>>) -> () {
    //{{{
    let mut d = HashMap::new();
    bucket_calc(&mut d, &v);
    // store L/S-type
    let mut ls: Vec<i8> = vec![-1; v.len()];
    // store starting positions of LMS-substrings
    let mut lms: Vec<usize> = Vec::new();
    ls[v.len()-1] = 0;
    ls_calc(&mut ls, &mut lms, &v);

    // induced sorting (LMS-type)
    //{{{
    // insert LMS to bucket
    let mut p = HashMap::new(); // store the positions of suffix inserted
    for (k, v) in &d {p.insert(k, v.0 + v.1 - 1);}
    // insert LMS
    for i in 0..std::cmp::max(1, lms.len()-1) {
        let j = *(p.get(&v[lms[i]]).unwrap());
        sa[j] = Some(lms[i]);
        if (d.get(&v[lms[i]]).unwrap()).0 < j {p.insert(&v[lms[i]], j-1);} else {p.remove(&v[lms[i]]);}
    }
    // induced sorting (L-type)
    for (k, v) in &d {p.insert(k, v.0);}
    for i in 0..v.len() {
        if sa[i] > Some(0) && ls[sa[i].unwrap()-1] == 1 {
            let j = *(p.get(&v[sa[i].unwrap()-1]).unwrap());
            sa[j] = Some(sa[i].unwrap()-1);
            p.insert(&v[sa[i].unwrap()-1], j+1);
        };
        if sa[i] != None && ls[sa[i].unwrap()] == -1 && i > 0 {sa[i] = None;};
    }
    // induced sorting (S-type)
    for (k, v) in &d {p.insert(k, v.0 + v.1 - 1);}
    for i in (0..sa.len()).rev() {
        if sa[i] > Some(0) && ls[sa[i].unwrap()-1] <= 0 {
            let j = *(p.get(&v[sa[i].unwrap()-1]).unwrap());
            sa[j] = Some(sa[i].unwrap()-1);
            p.insert(&v[sa[i].unwrap()-1], j-1);
        }
    }
    //}}}
        
    // check the uniqueness of S-prefixes
    //{{{
    let mut u: Vec<usize> = vec![0; lms.len()];
    let mut z: usize = 0;
    let mut p: HashMap<usize, usize> = HashMap::new();
    for i in 0..lms.len() {p.insert(lms[i], lms.len() - i - 1);};
    let mut unique = true;
    let mut sub: Vec<usize> = vec![0; 0];
    for j in 0..sa.len() {
        if sa[j] != None && ls[(sa[j]).unwrap()] == -1 {
            let i = *(p.get(&((sa[j]).unwrap()))).unwrap();
            if (sa[j]).unwrap() < v.len() - 1 {
                // LMS-substring
                let t: Vec<usize> = v[(sa[j]).unwrap()..lms[lms.len()-i-2]+1].iter().map(|x| *x as usize).collect();
                if sub == t {unique = false} else {z += 1};
                sub = t;
            }
            u[i] = z;
        };
    }

    // if each LMS is not unique, sort LMS recursively
    let mut sa_u: Vec<Option<usize>> = vec![None; u.len()];
    if unique {
        for i in 0..u.len() {
            sa_u[i] = Some(u[i]);
        }
    } else {create(&u, &mut sa_u)};
    let mut sorted_lms: Vec<usize> = vec![0; lms.len()];
    for i in 0..sa_u.len() {
        sorted_lms[i] = lms[lms.len() - sa_u[i].unwrap() - 1];
    }
    // }}}


    // induced sorting
    //{{{
    let mut p = HashMap::new();
    for (k, v) in &d {p.insert(k, v.0 + v.1 - 1);}
    for i in 0..sa.len() {sa[i] = None;}
    for e in sorted_lms.iter().rev() {
        let j = *(p.get(&v[*e]).unwrap());
        sa[j] = Some(*e as usize);
        if j != 0 {p.insert(&v[*e], j-1);}
    }
    // induced sorting (L-type)
    for (k, v) in &d {p.insert(k, v.0);}
    for i in 0..sa.len() {
        if sa[i] > Some(0) && ls[(sa[i].unwrap() as usize)-1] == 1 {
            let j = *(p.get(&v[(sa[i].unwrap() as usize)-1]).unwrap());
            sa[j] = Some(sa[i].unwrap()-1);
            p.insert(&v[(sa[i].unwrap() as usize)-1], j+1);
        }
        if sa[i] >= Some(0) && ls[sa[i].unwrap() as usize] == -1 && i > 0 {sa[i] = None;};
    }
    // induced sorting (S-type)
    for (k, v) in &d {p.insert(k, v.0 + v.1 - 1);}
    for i in (0..sa.len()).rev() {
        if sa[i] > Some(0) && ls[(sa[i].unwrap() as usize)-1] <= 0 {
            let j = *(p.get(&v[(sa[i].unwrap() as usize)-1]).unwrap());
            sa[j] = Some(sa[i].unwrap()-1);
            p.insert(&v[(sa[i].unwrap() as usize)-1], j-1);
        }
        else if sa[i] == Some(0) && ls[v.len()-1] <= 0 {
            let j = *(p.get(&v[v.len()-1]).unwrap());
            sa[j] = Some(v.len()-1);
            if j != 0 {p.insert(&v[v.len()-1], j-1);}
        }
    }
    //}}}

    //}}}
}


