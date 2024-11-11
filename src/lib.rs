use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn download_files(url: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let pb = ProgressBar::new(0);
    let mut downloaded = 0;
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let file_name = url.split("/").last().unwrap_or("Unknown");
    match std::fs::exists("./downloads") {
        Ok(exists) => {
            if !exists {
                let created_dir = std::fs::create_dir("./downloads");
                if let Err(err) = created_dir {
                    panic!("Failed to create downloads directory {:#?}", err);
                }
            }
        }
        Err(err) => {
            panic!("Couldn't resolve downloads folder {:#?}", err);
        }
    }
    let mut file = File::create(format!("./downloads/{}", file_name)).await?;
    println!("Downloading {}...", url);
    let mut stream = reqwest::get(url).await?.bytes_stream();
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        downloaded = downloaded + chunk.len() as u64;
        pb.set_position(downloaded + chunk.len() as u64);
        file.write_all(&chunk).await?;
    }
    pb.set_length(downloaded);
    file.flush().await?;
    pb.finish_with_message("Successfully downloaded file");
    println!("Successfully downloaded {url}");
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download() {
        let test_dl = download_files("https://ash-speed.hetzner.com/100MB.bin").await;
        match test_dl {
            Ok(res) => assert_eq!(res, true),
            Err(err) => panic!("Something went wrong {:#?}", err),
        }
    }
}
