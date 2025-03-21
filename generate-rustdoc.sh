echo '+rustdoc src/main.rs'
rustdoc src/main.rs

echo '+rustdoc worker/src/lib.rs --crate-name server'
rustdoc worker/src/lib.rs --crate-name server

echo '+rustdoc common/src/lib.rs --crate-name shared'
rustdoc common/src/lib.rs --crate-name shared

echo '+rustdoc worker/src/lib.rs --crate-name worker'
rustdoc worker/src/lib.rs --crate-name worker