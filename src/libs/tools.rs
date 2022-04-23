pub mod pretty {

    pub fn bytes(size: u64) -> String {
        let units = vec!["Byte", "K", "M", "G", "T", "P", "E", "Z", "Y", "B"];
        let usize = 1000_f64;
        let mut index = 0;
        let mut k = size as f64;

        while k >= usize {
            k = k / usize;
            index += 1;
        }

        return format!("{:.2} {}", k, units[index]);
    }
}

#[cfg(test)]
mod tests {

    use super::pretty;

    #[test]
    fn it_works() {
        assert_eq!("0.00 Byte", pretty::bytes(0u64));
        assert_eq!("100.00 Byte", pretty::bytes(100u64));
        assert_eq!("1000.00 Byte", pretty::bytes(1000u64));
        assert_eq!("1.95 K", pretty::bytes(2000u64));
        assert_eq!("976.56 K", pretty::bytes(1000000u64));
        assert_eq!("9.54 M", pretty::bytes(10000000u64));
        assert_eq!("888.18 P", pretty::bytes(1000000000000000000u64));
    }
}
