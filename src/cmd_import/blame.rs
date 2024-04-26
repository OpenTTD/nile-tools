use std::path::Path;
use std::process::Output;

pub struct ProcessLineResult {
    pub id: String,
    pub case: String,
    pub translation: String,
    pub commit: String,
}

pub struct Blame {
    output: String,
}

pub struct BlameIterator<'a> {
    iter: std::str::Split<'a, &'a str>,
}

impl<'a> BlameIterator<'a> {
    /**
     * Parse a single STR_ line, splitting it in id/case/translation information.
     * It is augmented with the commit the translation was made in.
     */
    fn parse_line(line: &str, commit: &str) -> Option<ProcessLineResult> {
        /* We only care about STR_ entries in the file. */
        if !line.starts_with("STR_") {
            return None;
        }

        /* The line is in the format: STR_NNN(.case)         :(translation) */

        let mut parts = line.split_whitespace();
        let id = parts.next().unwrap();
        let translation = parts.collect::<Vec<&str>>().join(" ");

        /* Remove the ":" from base, and only use the first 10 characters for commits. */
        let translation = &translation[1..];
        let commit = &commit[..10];

        /* Check if there is a "." in the id; if so, this is an <id>.<case>. */
        let mut parts = id.split(".");
        let id = parts.next().unwrap();
        let case = parts.next().unwrap_or("default");

        Some(ProcessLineResult {
            id: id.to_string(),
            case: case.to_string(),
            translation: translation.to_string(),
            commit: commit.to_string(),
        })
    }
}

impl<'a> Iterator for BlameIterator<'a> {
    type Item = ProcessLineResult;

    /**
     * Find the next valid STR_ line in the output.
     */
    fn next(&mut self) -> Option<Self::Item> {
        /* Read the next chunk. */
        while let Some(line) = self.iter.next() {
            let commit = line.split(" ").nth(0).unwrap();

            while let Some(line) = self.iter.next() {
                /* Skip all metadata of the commit. */
                if !line.starts_with("\t") {
                    continue;
                }

                /* Check if the line is a STR_ line. */
                let line = line.trim_start_matches("\t");
                if let Some(result) = BlameIterator::<'a>::parse_line(line, commit) {
                    return Some(result);
                }

                /* Analyze next commit. */
                break;
            }
        }

        None
    }
}

impl Blame {
    /**
     * Run "git blame" on the file, and return the output.
     */
    pub fn new(path: &Path, language: &String, commit: &String) -> Self {
        eprintln!("Running 'git blame' on {} for commit {}", language, commit);

        let file = Path::new("src/lang").join(language).with_extension("txt");
        let output = Blame::git_blame(path, &file, commit);

        let output = if output.stdout.is_empty() {
            /* Older versions had the language files in another folder. */
            let file = Path::new("lang").join(language).with_extension("txt");
            let output = Blame::git_blame(path, &file, commit);

            /* Older versions sometimes had invalid UTF-8 in them. Ignore them. */
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            String::from_utf8(output.stdout).unwrap()
        };

        Blame { output: output }
    }

    /**
     * Create an iterator that will go over every STR_ in the output.
     */
    pub fn iter(&self) -> BlameIterator {
        BlameIterator {
            iter: self.output.split("\n"),
        }
    }

    /**
     * Internal command to run "git blame" on a file.
     */
    fn git_blame(path: &Path, file: &Path, commit: &String) -> Output {
        /* Executing "git blame" via a Shell is much faster than using libgit. */
        std::process::Command::new("git")
            .arg("blame")
            .arg("--porcelain")
            .arg(commit)
            .arg(file)
            .current_dir(path)
            .output()
            .expect("Failed to execute command")
    }
}
