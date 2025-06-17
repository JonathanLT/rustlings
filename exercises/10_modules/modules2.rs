// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    pub use fruits::PEAR as PEAR;
    pub use veggies::CARROT as CARROT;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {

    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::PEAR,
        delicious_snacks::CARROT,
    );
}
