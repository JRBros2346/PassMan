use std::process::{Child, Command, Stdio};

mod db;

struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        self.0.kill().expect("Nah, I'd Win..");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    let _server = Server(Command::new("surreal")
        .args(["start"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()?);
    std::thread::sleep(std::time::Duration::from_secs(3));
    db::config().await?;
    Ok(())
}
