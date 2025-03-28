mod models;
mod generators;

fn main() {
    println!("Hello, world!");
    generators::world_generator::generate_world(4, 4);
}
