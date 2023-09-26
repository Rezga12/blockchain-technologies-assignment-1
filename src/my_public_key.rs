pub struct MyPublicKey;

impl From<&str> for MyPublicKey {
    fn from(value: &str) -> Self {
        unimplemented!("convert base58 encoded string slice to your MyPublicKey struct")
    }
}

impl From<String> for MyPublicKey {
    fn from(value: String) -> Self {
        unimplemented!("convert base58 encoded string type to your MyPublicKey struct")
    }
}

impl From<[u8; 32]> for MyPublicKey {
    fn from(bytes: [u8; 32]) -> Self {
        unimplemented!("convert byte array to our MyPublic key struct")
    }
}

impl PartialEq for MyPublicKey {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!("implement partial equality operator for our struct to be able to compare by `==` operator")
    }
}