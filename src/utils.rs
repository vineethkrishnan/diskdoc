use human_bytes::human_bytes;

pub fn format_size(size: u64) -> String {
    human_bytes(size as f64)
}
