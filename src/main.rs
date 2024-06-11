use std::{fs::File, io::Write};
use voicevox_client::Client;

const ASTR: &str = r##"ずんだもんなのだ！"##;
const SPEAKER_ID: i32 = 1;

#[tokio::main]
async fn main() {
    let mut client = Client::new("http://localhost:50021".to_string(), None);
    let sentences = ASTR.split("。").collect::<Vec<&str>>();
    for (i, s) in sentences.iter().enumerate() {
        let file_name = format!("output/{}.wav", i);
        tts(&mut client, &file_name, s).await;
    }
}

async fn tts(client: &mut Client, file_name: &str, text: &str) {
    let audio_query = client
        .create_audio_query(text, SPEAKER_ID, None)
        .await
        .unwrap();
    let audio = audio_query.synthesis(SPEAKER_ID, true).await.unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(&audio).unwrap();
}
