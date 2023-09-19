use krates::Krate;
use semver::Version;

mod krates;

fn main() {
    let mut krate = Krate::new("env_logger", Version::new(0,10,0));
    let owners = krate.get_crate_owners().unwrap();
    let metadata = krate.get_krate_meta().unwrap();
}
