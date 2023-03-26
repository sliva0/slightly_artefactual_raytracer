use std::borrow::Cow;

use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref STYLE: ProgressStyle =
        ProgressStyle::with_template("{msg:14} {elapsed:>3} {wide_bar} {pos}/{len} ETA {eta:>3}",)
            .unwrap();
}

pub fn progress_bar(pixel_count: usize, message: impl Into<Cow<'static, str>>) -> ProgressBar {
    ProgressBar::new(pixel_count as u64)
        .with_style(STYLE.clone())
        .with_finish(ProgressFinish::AndLeave)
        .with_message(message)
}
