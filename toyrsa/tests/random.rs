use toyrsa::*;

#[test]
fn freebie_test() {
    let private_key = (0xed23e6cd_u32, 0xf050a04d_u32); //(p,q)
    let pub_key = private_key.0 as u64 * private_key.1  as u64;
    let message = 0x12345f_u32;
    let encrypted = encrypt(pub_key, message);
    assert_eq!(encrypted, 0x6418280e0c4d7675);
    assert_eq!(decrypt(private_key, encrypted), message);
}

#[test]
fn randomly_test() {
    println!("Testing!");
}