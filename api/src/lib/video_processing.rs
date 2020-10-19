use std::error::Error;
use std::io::Write;
use subprocess::{Popen, PopenConfig, Redirection};
use youtube_dl::YoutubeDl;

pub fn extract_url(url: &str) -> Result<String, Box<dyn Error>> {
    let output = YoutubeDl::new(url)
        .socket_timeout("15")
        .format("best")
        .run()?;

    let video = match output {
        youtube_dl::YoutubeDlOutput::Playlist(_) => return Err("Playlist are not supported".into()),
        youtube_dl::YoutubeDlOutput::SingleVideo(video) => video,
    };

    match video.url {
        Some(url) => Ok(url),
        None => Err("Something is fucked".into()),
    }
}

pub fn stream(url: &str, volume: i32) -> Result<Popen, Box<dyn Error>> {
    let url = extract_url(url)?;

    let p = Popen::create(
        &[
            "omxplayer",
            "-b",
            "-r",
            "-o",
            "both",
            &url,
            "--vol",
            &volume.to_string(),
        ],
        PopenConfig {
            stdin: Redirection::Pipe,
            stdout: Redirection::None,
            stderr: Redirection::None,
            ..Default::default()
        },
    )?;

    Ok(p)
}

pub fn write_to_stdin(process: &mut Popen, msg: &str) -> Result<(), Box<dyn Error>> {
    process.stdin.as_mut().unwrap().write(msg.as_bytes())?;
    Ok(())
}