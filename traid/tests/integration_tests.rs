use std::sync::Mutex;
use traid::raid::{cmd_write, read_bytes,cmd_init};


static  LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_write_and_read() {
    let _guard = LOCK.lock().unwrap();
    cmd_init(3, 4, 16).unwrap();
    cmd_write("deadbeef".to_string()).unwrap();
    let result = read_bytes(0, 4).unwrap();
    assert_eq!(result, vec![0xde, 0xad, 0xbe, 0xef]);
}

#[test]
fn test_rebuild() {
    let _guard = LOCK.lock().unwrap();
    cmd_init(3, 4, 16).unwrap();
    cmd_write("deadbeef".to_string()).unwrap();
    traid::raid::cmd_fail(0).unwrap();
    traid::raid::cmd_rebuild(0).unwrap();
    let result = read_bytes(0, 4).unwrap();
    assert_eq!(result, vec![0xde, 0xad, 0xbe, 0xef]);
}

#[test]
fn test_without_rebuild() {
    let _guard = LOCK.lock().unwrap();
    cmd_init(3, 4, 16).unwrap();
    cmd_write("deadbeef".to_string()).unwrap();
    traid::raid::cmd_fail(0).unwrap();
    let result = read_bytes(0, 4).unwrap();
    assert_eq!(result, vec![0xde, 0xad, 0xbe, 0xef]);
}
