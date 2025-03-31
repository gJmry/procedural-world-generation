mod models;
mod generators;

fn main() {
    println!("Hello, world!");
    let world = generators::world_generator::generate_world(16, 16);

    for i in world.iter().clone() {
        for j in i {
            println!("{}", j);
        }
    }

    println!("\n\n\n");
    println!("Affichage en tableau \n \n \n");

    for i in world.iter().clone() {
        for j in i {
            print!(" {} ", j.biome);
        }
        println!();
    }

    println!("\n\n\n");
    println!("Affichage en json \n \n \n");
    println!(" {} ", generators::world_generator::serialize_to_json(world));
}
