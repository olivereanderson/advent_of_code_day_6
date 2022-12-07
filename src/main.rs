// Find the position of the first byte following the first occurrence of a sequence consisting of `SEQUENCE_LENGTH` distinct bytes.
fn find_marker<const SEQUENCE_LENGTH: usize>(input: &[u8]) -> Option<usize> {
    input
        .windows(SEQUENCE_LENGTH)
        .enumerate()
        .find(|(_idx, window)| {
            <[u8; SEQUENCE_LENGTH]>::try_from(&window[..])
                .ok()
                .map(is_distinct::<SEQUENCE_LENGTH>)
                .unwrap_or_default()
        })
        .map(|(idx, _)| idx + SEQUENCE_LENGTH)
}

// Returns true exactly when the array does not contain any duplicate entries.
fn is_distinct<const SIZE: usize>(mut chunk: [u8; SIZE]) -> bool {
    // Reduce the problem to the case where the array is sorted.
    chunk.sort_unstable();
    // Check whether the mapping i -> chunk[i] is strictly monotone.
    chunk
        .into_iter()
        .fold((true, None), |acc, value| {
            ((acc.0 && (acc.1 < Some(value))), Some(value))
        })
        .0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read("input.txt")?;
    // Part 1:
    const NUM_DIFFERENT_CHARS_PACKET_MARKER: usize = 4;
    let first_packet_after = find_marker::<NUM_DIFFERENT_CHARS_PACKET_MARKER>(&input).unwrap();
    println!("Part 1: {first_packet_after}");

    // Part 2:
    const NUM_DIFFERENT_CHARS_MESSAGE_MARKER: usize = 14;
    // Take advantage of the already completed search from Part 1.
    let latest_message_marker_start_position = first_packet_after
        .checked_sub(2 * NUM_DIFFERENT_CHARS_PACKET_MARKER)
        .unwrap_or_default();
    // The position of the message marker relative to where we started the search.
    let first_message_after_rel = find_marker::<NUM_DIFFERENT_CHARS_MESSAGE_MARKER>(
        &input[latest_message_marker_start_position..],
    )
    .unwrap();
    let first_message_after = latest_message_marker_start_position + first_message_after_rel;
    println!("Part 2: {first_message_after}");
    Ok(())
}
