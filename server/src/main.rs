mod models;
mod generators;

fn main() {
    println!("Hello, world!");
    let world = generators::world_generator::generate_world(8, 8);
    for i in world {
        for j in i {
            println!("{}", j);
        }
    }
}
