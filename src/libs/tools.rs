pub mod pretty {

    pub fn bytes(size: f64) -> String {
        let units = vec![
            "Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB", "BB",
        ];
        let usize = 1024_f64;
        let mut index = 0;
        let mut k = size;

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
        assert_eq!("-1.00 Bytes", pretty::bytes(-1f64));
        assert_eq!("0.00 Bytes", pretty::bytes(0f64));
        assert_eq!("100.00 Bytes", pretty::bytes(100f64));
        assert_eq!("1000.00 Bytes", pretty::bytes(1000f64));
        assert_eq!("1.95 KB", pretty::bytes(2000f64));
        assert_eq!("976.56 KB", pretty::bytes(1000000f64));
        assert_eq!("9.54 MB", pretty::bytes(10000000f64));
        assert_eq!("888.18 PB", pretty::bytes(1000000000000000000f64));
    }
}
