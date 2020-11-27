use crate::Url;
use crate::Volume;
use cached::proc_macro::cached;
use std::error::Error;
use std::io::Write;
use subprocess::{Popen, PopenConfig, Redirection};
use youtube_dl::YoutubeDl;

#[derive(Debug, Clone)]
pub enum CustomError {
    FeatureError(String),
}

// #[cached(size = 100)]
pub fn extract_url(url: Url) -> Result<Url, CustomError> {
    if url.is_ip() {
        return Ok(url.clone());
    }

    let output = YoutubeDl::new(&url.url)
        .socket_timeout("15")
        .format("best")
        .run();

    if let Err(_) = output {
        return Err(CustomError::FeatureError("youtube_dl error".into()));
    }

    let video = match output.unwrap() {
        youtube_dl::YoutubeDlOutput::Playlist(_) => {
            return Err(CustomError::FeatureError(
                "Playlist are not supported".into(),
            ))
        }
        youtube_dl::YoutubeDlOutput::SingleVideo(video) => video,
    };

    match video.url {
        Some(extracted_url) => Ok(Url::new(url.url.clone(), Some(extracted_url))),
        None => Err(CustomError::FeatureError("Something is fucked".into())),
    }
}

#[cfg(target_arch = "arm")]
pub fn stream(url: &Url, volume: Volume) -> Result<Popen, Box<dyn Error>> {
    let url = extract_url(url)?;

    let p = Popen::create(
        &[
            "omxplayer",
            "-b",
            "-r",
            "-o",
            "both",
            &url.extracted_url.unwrap(),
            "--vol",
            &volume.as_milibells().to_string(),
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

#[cfg(target_arch = "x86_64")]
pub fn stream(_url: &Url, _volume: Volume) -> Result<Popen, Box<dyn Error>> {
    let argv = &["ping", "-c", "10", "8.8.8.8"];

    let p = Popen::create(
        argv,
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
    process.stdin.as_mut().unwrap().write_all(msg.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use subprocess::{Popen, PopenConfig, Redirection};

    use crate::write_to_stdin;

    fn init_popen() -> Popen {
        Popen::create(
            &["cat"],
            PopenConfig {
                stdin: Redirection::Pipe,
                stdout: Redirection::Pipe,
                stderr: Redirection::None,
                ..Default::default()
            },
        )
        .unwrap()
    }

    #[test]
    fn test_write_to_stdin() {
        let mut process = init_popen();
        let msg = "any message";
        write_to_stdin(&mut process, msg).unwrap();
        let (stdout, _) = process.communicate(Some("")).unwrap();
        assert!(stdout.unwrap().as_str() == msg)
    }
}
