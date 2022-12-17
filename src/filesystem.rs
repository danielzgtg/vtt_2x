use std::fs::read_to_string;

pub(crate) struct OpenFiles {
    pub(crate) cc: Option<String>,
    pub(crate) transcript: Option<String>,
    pub(crate) html: Option<String>,
}

fn extract_extensions(x: &str) -> &str {
    let space = x.rfind(' ').unwrap_or(0);
    let x = &x[space..];
    let dot = x.find('.').expect("Missing extension");
    &x[dot + 1..]
}

pub(crate) fn open_files(paths: Vec<String>) -> OpenFiles {
    let mut transcript: Option<String> = None;
    let mut cc: Option<String> = None;
    let mut html: Option<String> = None;
    for path in paths {
        match extract_extensions(&path) {
            "cc.vtt" => {
                assert!(
                    cc.replace(read_to_string(path).expect("Read cc")).is_none(),
                    "Duplicate cc"
                );
            }
            "transcript.vtt" | "srt" => {
                assert!(
                    transcript
                        .replace(read_to_string(path).expect("Read transcript"))
                        .is_none(),
                    "Duplicate transcript"
                );
            }
            "txt" | "html" => {
                assert!(
                    html.replace(read_to_string(path).expect("Read html"))
                        .is_none(),
                    "Duplicate html"
                );
            }
            e => {
                if e.ends_with("webm") || e.ends_with("mp4") {
                    continue;
                }
                panic!("Unexpected file extension {}", path)
            }
        }
    }
    OpenFiles {
        cc,
        transcript,
        html,
    }
}
