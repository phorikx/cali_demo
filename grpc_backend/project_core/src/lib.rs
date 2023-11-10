use tokio::task;

pub struct Cable {
    end_1: String,
    end_2: String,
}
pub async fn spawn_cable() -> task::JoinHandle<Cable> {
    let task = tokio::task::spawn(async {
        println!("Spawning cable");
        Cable { end_1: String::from("empty"), end_2: String::from("empty") }
    });
    task
}
