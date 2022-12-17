use std::fs::read_to_string;

pub(crate) struct OpenFiles {
    pub(crate) cc: Option<String>,
    pub(crate) transcript: Option<String>,
    pub(crate) html: String,
}

pub(crate) fn open_files<'a, I: Iterator<Item = &'a str>>(paths: I) -> OpenFiles {
    let mut transcript = None;
    let mut cc = None;
    let mut html = None;
    for path in paths {
        match &path[path.find('.').expect("Missing extension") + 1..] {
            "cc.vtt" => {
                assert!(
                    cc.replace(read_to_string(path).expect("Read cc")).is_none(),
                    "Duplicate cc"
                );
            }
            "transcript.vtt" => {
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
        html: html.expect("HTML not specified"),
    }
}
