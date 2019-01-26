pub fn duration_to_s(duration: std::time::Duration) -> u64 {
    let nanos = duration.subsec_nanos() as u64;
    let s = (1000 * 1000 * 1000 * duration.as_secs() + nanos) / (1000 * 1000 * 1000);
    s
}
