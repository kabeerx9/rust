// run the cargo build command: cargo build
// this will create an executable in the target/debug directory
// to run the executable, we can use ./target/debug/hello_cargo
// we can also directly use cargo run to compile and run the executable
// cargo check - this will check if the code compiles without creating an executable
// so it helps to check for errors without having to wait for the compilation to complete


// We can create a project using cargo new.
// We can build a project using cargo build.
// We can build and run a project in one step using cargo run.
// We can build a project without producing a binary to check for errors using cargo check.
// Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.


// for release build, we can use cargo build --release
// this will create an executable in the target/release directory
// the release build will take longer to compile than the debug build
// but the executable will run faster

fn main() {
    println!("Hello, world! this time from cargo");
}
