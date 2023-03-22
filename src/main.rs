use anyhow::Result;
mod mnist_linear;

fn main() -> Result<()> {
    println!("Hello, world!");

    mnist_linear::run()
}
