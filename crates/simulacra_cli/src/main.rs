use simulacra::*;

#[tokio::main]
async fn main() {

  println!("Welcome to Simulacra. Type 'new' to generate a new world and simulation. Enter 'exit' to quit");
  loop{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    match line.trim(){
      "npc" => {
        let npc = get_new_world_place_npc().await;
        story_message(&npc);
      },
      "place" => {
        let world = get_new_world().await;
        let place = get_new_place(world).await;
        story_message(&place);
      },      
      "world" => {
        let world = get_new_world().await;
        story_message(&world);
      },
      "exit" => {
        status_message("Exiting");
        break;
      },
      _ => {
        status_message("Invalid command");
      }
    }
  }
}