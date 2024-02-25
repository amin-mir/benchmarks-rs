pub fn pieces_float(total_len: usize, piece_len: usize) -> usize {
    let pieces = total_len as f64 / piece_len as f64;
    pieces.ceil() as usize
}

pub fn pieces_usize(total_len: usize, piece_len: usize) -> usize {
    (total_len + piece_len - 1) / piece_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_pieces() {
        let total_len = 3001;
        let piece_len = 1000;
        assert_eq!(4, pieces_float(total_len, piece_len));
        assert_eq!(4, pieces_usize(total_len, piece_len));
    }

    #[test]
    fn three_pieces() {
        let total_len = 80000;
        let piece_len = 32768;
        assert_eq!(3, pieces_float(total_len, piece_len));
        assert_eq!(3, pieces_usize(total_len, piece_len));
    }
}
