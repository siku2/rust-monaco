use std::io::Write;
use swc_common::comments::{Comment, CommentKind};

pub fn insert_comments(w: &mut dyn Write, comments: Option<Vec<Comment>>) {
    if let Some(comments) = comments {
        for comment in comments {
            match comment.kind {
                CommentKind::Line => {
                    writeln!(w, "    /// {}", escape(comment.text.as_ref())).unwrap();
                }
                CommentKind::Block => {
                    writeln!(w, "    /*{}*/ ", escape(comment.text.as_ref())).unwrap();
                }
            }
        }
    }
}

fn escape(s: &str) -> String {
    s.replace("/*", "\\/\\*").replace("*/", "\\*\\/")
}
