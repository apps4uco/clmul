#[cfg(test)]
mod tests {
    use crate::clmul;
    use super::*;

    #[test]
    fn test_clmul() {
        //let b=0b10001;
        //println!("b\t{:032b}",b);
        println!();
        for a in 0..16 {
            println!("{}",a);
            println!("a\t{:032b}\t{}",a,a);

            let c=clmul(a,a);
            println!("c\t{:032b}\t{}",c,c);
            let b=(a as u8).reverse_bits() as u64;
            let d=clmul(c as u64,b);
            println!("d\t{:032b}\t{}",d,d);
        }
    }
}