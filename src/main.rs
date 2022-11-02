use grammers_client::{Client, Config};
use grammers_session::Session;
use grammers_tl_types as tl;
use tokio::runtime;
use std::io::{self, BufRead as _, Write as _};

fn prompt(message: &str) -> anyhow::Result<String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    Ok(line)
}

async fn async_main() -> anyhow::Result<()> {
    let tg_api_id = 2134545;
    let tg_api_hash = "17b514a71050f7eba50c23e79fe05e1e";
    const SESSION_FILE: &str = "bot.session";
    let client = Client::connect(Config {
        session: Session::load_file_or_create(SESSION_FILE)?,
        api_id: tg_api_id,
        api_hash: tg_api_hash.clone().to_string(),
        params: Default::default(),
    }).await?;
    if !client.is_authorized().await? {
        let phone = prompt("Enter your phone number (international format): ")?;
        let token = client.request_login_code(&phone, tg_api_id, tg_api_hash).await?;
        let code = prompt("Enter the code you received: ")?;
        client.sign_in(&token, &code).await?;
        client.session().save_to_file(SESSION_FILE)?;
    }
    // loop {
    //     if let Err(e) = client.invoke(&tl::functions::account::UpdateStatus { offline: false }).await {
    //         println!("Error while invoking a raw API call: {}", e);
    //         std::thread::sleep(std::time::Duration::from_secs(2));
    //         break
    //     } else {
    //         std::thread::sleep(std::time::Duration::from_secs(15));
    //     }
    // }
    // Ok(())
    loop {
        client.invoke(&tl::functions::account::UpdateStatus { offline: false }).await?;
        std::thread::sleep(std::time::Duration::from_secs(15));
    }
}

fn main() -> anyhow::Result<()> {
    loop {
        runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async_main())?;
        println!("restarting...");
    }
}