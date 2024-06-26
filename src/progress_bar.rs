pub fn generate_progress_bar(progress: f64) -> String {
    let progress_bar_capacity = 30;
    let passed_progress_bar_index = (progress * progress_bar_capacity as f64) as usize;
    let progress_bar = "▓".repeat(passed_progress_bar_index) + &"▁".repeat(progress_bar_capacity - passed_progress_bar_index);
    format!("{{ {} }}", progress_bar)
}