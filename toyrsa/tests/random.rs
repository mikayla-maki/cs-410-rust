use toyrsa::*;

///Runs Bart's given test, as a sanity check.
#[test]
fn freebie_test() {
    let private_key = (0xed23e6cd_u32, 0xf050a04d_u32); //(p,q)
    let pub_key = private_key.0 as u64 * private_key.1 as u64;
    let message = 0x12345f_u32;
    let encrypted = encrypt(pub_key, message);
    assert_eq!(encrypted, 0x6418280e0c4d7675);
    assert_eq!(decrypt(private_key, encrypted), message);
}

///Runs the encryption algorithm 100 different ways, ensuring it works.
#[test]
fn randomly_test() {
    //Took RNG code from Bart's library
    use rand::Rng;
    for _ in 0..10 {
        //Generate and test 10 different keys...
        let private = genkey();
        let pub_key = private.0 as u64 * private.1 as u64;
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            //With 10 random values
            let msg: u32 = rng.gen_range(0..=u32::MAX);
            assert_eq!(decrypt(private, encrypt(pub_key, msg)), msg);
        }
    }
}
