type int32 = i32;

struct ComplexRefs<'a>
{
    int_ref: &'a i32,
    unsigned_ref: &'a u32,
    str_ref: &'a str,
}

fn main() {
    let a: int32 = -423;
    let b: int32 = 9018239;
    let c: int32 = 2321;

    let d: u32 = 5;

    let str_ref: &str = "Hahaha";
    let cr: ComplexRefs = ComplexRefs {
        int_ref: &b,
        unsigned_ref: &d,
        str_ref,
    };


    let mut e;
    let g;
    {
        let r = cr;
        e = four_refs(&a, &b, &c, &r);
        g = four_refs_move(&a, &b, &c, &r);
    }
    println!("{}", e);
    println!("{}", b);
}

fn four_refs<'a, 'b>(a: &'a int32, b: &int32, c: &int32, cr: &'b ComplexRefs<'a>) -> &'a int32 {
    if *a < 5 {
        cr.int_ref
    } else {
        a
    }
}

fn four_refs_move(a: &int32, b: &int32, c: &int32, cr: &ComplexRefs) -> int32 {
    if *a < 5 {
        *cr.int_ref
    } else {
        *a
    }
}
