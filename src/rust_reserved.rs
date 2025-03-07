// You can find all the types included in the Rust std lib below
// .rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/std/all.html
// However, we're particularly interested in avoiding collisions with the prelude
// So we only disallow types that can be found in the prelude
// https://doc.rust-lang.org/std/prelude/index.html

pub const STD_TYPES: [&str; 40] = [
    "Copy",
    "Send",
    "Sized",
    "Sync",
    "Unpin",
    "Drop",
    "Fn",
    "FnMut",
    "FnOnce",
    "drop",
    "Box",
    "ToOwned",
    "Clone",
    "PartialEq",
    "PartialOrd",
    "Eq",
    "Ord",
    "AsRef",
    "AsMut",
    "Into",
    "From",
    "Default",
    "Iterator",
    "Extend",
    "IntoIterator",
    "DoubleEndedIterator",
    "ExactSizeIterator",
    "Self",
    "Option",
    "Some",
    "None",
    "Result",
    "Ok",
    "Err",
    "String",
    "ToString",
    "Vec",
    "TryFrom",
    "TryInto",
    "FromIterator"
];