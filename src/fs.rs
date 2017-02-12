use std::io;
use std::fs::File;
use std::path::Path;
use serde_json;
use spec::Spec;
use runtime::RuntimeSpec;

pub fn load<P: AsRef<Path>>(p: P) -> io::Result<(Spec, RuntimeSpec)> {
    fn err(e: serde_json::Error) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, e)
    }
    let p = p.as_ref();
    let config = try!(File::open(p.join("config.json")));
    let runtime = try!(File::open(p.join("runtime.json")));
    Ok((try!(Spec::from_read(config).map_err(err)),
        try!(RuntimeSpec::from_read(runtime).map_err(err))))
}
