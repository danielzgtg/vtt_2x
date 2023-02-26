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
    &x[dot..]
}

pub(crate) fn open_files(paths: Vec<String>) -> OpenFiles {
    let mut transcript: Option<String> = None;
    let mut cc: Option<String> = None;
    let mut html: Option<String> = None;
    for path in paths {
        let ext = extract_extensions(&path);
        if ext.ends_with(".cc.vtt") {
            assert!(
                cc.replace(read_to_string(path).expect("Read cc")).is_none(),
                "Duplicate cc"
            );
        } else if ext.ends_with(".transcript.vtt") || ext.ends_with(".srt") {
            assert!(
                transcript
                    .replace(read_to_string(path).expect("Read transcript"))
                    .is_none(),
                "Duplicate transcript"
            );
        } else if ext.ends_with(".txt") || ext.ends_with(".html") {
            assert!(
                html.replace(read_to_string(path).expect("Read html"))
                    .is_none(),
                "Duplicate html"
            );
        } else if ext.ends_with(".webm") || ext.ends_with(".mp4") {
            continue;
        } else {
            panic!("Unexpected file extension {}", ext);
        }
    }
    OpenFiles {
        cc,
        transcript,
        html,
    }
}
