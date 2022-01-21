pub mod pretty {

    /// byte size for 1 byte
    const B: f64 = 1.00;
    /// bytes size for 1 kilobyte
    const KB: f64 = 1_000.00;
    /// bytes size for 1 megabyte
    const MB: f64 = 1_000_000.00;
    /// bytes size for 1 gigabyte
    const GB: f64 = 1_000_000_000.00;
    /// bytes size for 1 terabyte
    const TB: f64 = 1_000_000_000_000.00;
    /// bytes size for 1 petabyte
    const PB: f64 = 1_000_000_000_000_000.00;

    pub fn bytes(size: f64) -> String {
        if size <= B {
            return format!("{:.2} Bytes", size);
        }
        if size > B && size <= KB {
            return format!("{:.2} KB", size / B);
        }
        if size > KB && size <= MB {
            return format!("{:.2} MB", size / KB);
        }
        if size > MB && size <= GB {
            return format!("{:.2} GB", size / MB);
        }
        if size > GB && size <= TB {
            return format!("{:.2} TB", size / GB);
        }
        if size > TB && size <= PB {
            return format!("{:.2} PB", size / TB);
        }
        return format!("{:.2} Bytes", size);
    }
}
