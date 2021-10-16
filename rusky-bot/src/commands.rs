macro include_commands($($cat:ident$(,)?)*) {
    $(
        pub mod $cat;
        pub use $cat::*;
    )*
}
include_commands!(information);
