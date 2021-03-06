#[macro_use]
extern crate dymod;


dymod!
{
    #[path = "../subcrate/src/lib.rs"]
    pub mod subcrate
    {
        fn count_sheep(sheep: u32) -> &'static str;
    }
}


fn main()
{
    println!();

    if dymod::is_hotswapping_enabled() {
        println!("You are running in hotswapping mode.");
        println!("Make changes to subcrate/src/lib.rs");
        println!("Then run `cargo build` in the subcrate directory.");
        println!("You should see your changes apply while this code runs:");
    } else {
        println!("You are running in static mode.");
        println!("The `subcrate` module has been statically linked.");
        println!("Any changes made to subcrate/src/lib.rs will not apply until this program is recompiled.");
    }

    println!("\nPress Ctrl+C to quit.\n");

    loop
    {
        let num_sheep = 3;
        let message = subcrate::count_sheep(num_sheep);
        println!("There are '{}' sheep.", message);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
