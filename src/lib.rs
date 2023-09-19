mod krates;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use semver::Version;

    use crate::krates::Krate;


    #[test]
    fn exploration() {
        let mut krate = Krate::new("env_logger", Version::new(0,10,0));
        assert!(krate.get_krate_meta().is_some())
    }

    #[test]
    fn owners_name() {
        let mut krate = Krate::new("env_logger", Version::new(0,10,0));
        let owners = krate.get_crate_owners();
        assert!(owners.is_some());
        for owner in owners.unwrap() {
            println!("{:?}", owner);
        }
        assert!(krate.get_krate_meta().is_some())
    }
}