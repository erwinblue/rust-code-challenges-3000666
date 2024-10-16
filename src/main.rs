use std::path;
use std::fs;
use tempfile;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        //todo!();
        let metadata = match fs::metadata(self) {
            Ok(m) => m,
            Err(e) => panic!("ERROR: Cannot read attributes of file {:?}\n{:?}", self.file_name(), e)
        };
        metadata.permissions().readonly()
    }

    fn is_writeable(&self) -> bool {
        //todo!();
        let metadata = match fs::metadata(self) {
            Ok(m) => m,
            Err(e) => panic!("ERROR: Cannot read attributes of file {:?}\n{:?}", self.file_name(), e)
        };
        !metadata.permissions().readonly()
    }

    fn exists(&self) -> bool {
        //todo!();
        let response = match self.try_exists() {
            Ok(r) => r,
            Err(e) => panic!("ERROR: Cannot find file {:?}\n{:?}", self.file_name(), e)
        };
        response
    }
}

fn main() {

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    if f.path().is_readable() {
        println!("File {:?} is readable!", f);
    }

    //fs::remove_file(f.path()).unwrap();
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}
