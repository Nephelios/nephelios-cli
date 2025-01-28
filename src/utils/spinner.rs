use indicatif::{ProgressBar, ProgressStyle};

/// Creates a new spinner with custom styling for loading animations.
///
/// # Arguments
///
/// * `message` - The message to display next to the spinner
///
/// # Returns
///
/// * `ProgressBar` - A configured progress bar instance with spinner animation
///
/// # Examples
///
/// ```
/// let spinner = create_spinner("Loading...");
/// // Do some work
/// spinner.finish_and_clear();
/// ```
pub fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(std::time::Duration::from_millis(120));
    spinner
}
