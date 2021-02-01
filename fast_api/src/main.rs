use std::cmp::Ordering;

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct G<'a, T: Ord> { m: &'a T }

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Value { v: i32 }

fn longer<'a, T: Ord>(s1: &'a T, s2: &'a T) -> &'a T {
    if s1 > s2 { s1 } else { s2 }
}

fn main() {
    let v0 = Value { v: 12 };
    let v1 = Value { v: 15 };
    let res_v = longer(&v0, &v1);
    println!("{}", res_v.v);//15

    let g0 = G { m: &v0 };
    let g1 = G { m: &v1 };
    let res_g = longer(&g0, &g1);//15
    println!("{}", res_g.m.v);
}
