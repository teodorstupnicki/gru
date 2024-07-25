#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        const DEFAULT_CONFIGFILE_NAME: &str = "dotcc-config.json";
        let x = dotcc::read_config(DEFAULT_CONFIGFILE_NAME);
        let check = if let Ok(_) = x {
            true
        } else {
            false
        };
        assert!(check);
    }
}
