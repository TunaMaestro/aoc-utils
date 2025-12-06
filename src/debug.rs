pub trait PrintBytes {
    fn print(&self);
    fn display(&self) -> String;
}

impl PrintBytes for &[u8] {
    fn display(&self) -> String {
        String::from_utf8_lossy(&self).to_string()
    }
    fn print(&self) {
        println!("{}", self.display());
    }
}
