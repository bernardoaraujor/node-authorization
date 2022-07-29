use sp_keyring::AccountKeyring;

fn main() {
    println!("{:?}", bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2")
                        .into_vec()
                        .unwrap());
    println!("{:?}", bs58::decode("12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust")
        .into_vec()
        .unwrap());

    let a = AccountKeyring::Alice.to_account_id();
    let b = AccountKeyring::Bob.to_account_id();
    println!("{:?}", a);
    println!("{:?}", b);

    println!("{:?}", hex_literal::hex!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").to_vec());
    println!("{:?}", hex_literal::hex!("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48").to_vec());
}
