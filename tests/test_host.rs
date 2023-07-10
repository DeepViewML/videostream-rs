use std::path::PathBuf;
use videostream::host::Host;

#[test]
fn test_host() {
    let path = PathBuf::from("/tmp/test.vsl");
    let host = Host::new(&path).unwrap();
    assert_eq!(path, host.path().unwrap());
    assert!(path.exists());
    // Rust doesn't provide an is_socket but we at least confirm some things it is not.
    assert!(!path.is_file());
    assert!(!path.is_dir());
    assert!(!path.is_symlink());

    // FIXME: currently the library will unlink old sockets, this should be
    // corrected along with adding proper cleanup and error handling when a
    // socket is already present.
    //
    // Creating a second host at the same path should raise an error.
    // let host2 = Host::new(&path);
    // assert!(host2.is_err());
}
