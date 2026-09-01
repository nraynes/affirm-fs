/// Checks whether a subslice is present in a slice.
pub fn contains_subslice<T: PartialEq>(data: &[T], needle: &[T]) -> bool {
    data.windows(needle.len()).any(|window| window == needle)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn collection_contains_subslice() {
        let full_collecion_str = "This is a test phrase to test the fancy pizza that is growing in the garden. The pizza is real.";
        let full_collection_arr = [9, 91, 223, 0493, 82939, 98, 34, 99, 99, 99, 7899];
        let slice_str_one = "to test the fancy pizza";
        let slice_arr_one = [82939, 98, 34, 99];
        let slice_str_two = "ncy pizza t";
        let slice_arr_two = [98, 34, 99, 99, 99];
        let slice_str_three = "pizza";
        let slice_arr_three = [9, 91, 223, 0493];
        let slice_str_four = "growing";
        let slice_arr_four = [82939];
        let slice_str_five = "pizza";
        let slice_arr_five = [99];

        assert!(contains_subslice(
            full_collecion_str.as_bytes(),
            slice_str_one.as_bytes()
        ));
        assert!(contains_subslice(
            full_collecion_str.as_bytes(),
            slice_str_two.as_bytes()
        ));
        assert!(contains_subslice(
            full_collecion_str.as_bytes(),
            slice_str_three.as_bytes()
        ));
        assert!(contains_subslice(
            full_collecion_str.as_bytes(),
            slice_str_four.as_bytes()
        ));
        assert!(contains_subslice(
            full_collecion_str.as_bytes(),
            slice_str_five.as_bytes()
        ));
        assert!(contains_subslice(&full_collection_arr, &slice_arr_one));
        assert!(contains_subslice(&full_collection_arr, &slice_arr_two));
        assert!(contains_subslice(&full_collection_arr, &slice_arr_three));
        assert!(contains_subslice(&full_collection_arr, &slice_arr_four));
        assert!(contains_subslice(&full_collection_arr, &slice_arr_five));
    }

    #[test]
    fn collection_not_contains_subslice() {
        let full_collecion_str = "This is a test phrase to test the fancy pizza that is growing in the garden. The pizza is real.";
        let full_collection_arr = [9, 91, 223, 0493, 82939, 98, 34, 99, 99, 99, 7899];
        let slice_str_one = "to test the ffancy pizza";
        let slice_arr_one = [82939, 98, 34, 998];
        let slice_str_two = "test phrase is growing";
        let slice_arr_two = [7809, 9187234, 100, 123];
        let slice_str_three = "hemoculus";
        let slice_arr_three = [9, 91, 0493];
        let slice_str_four = "subtraction";
        let slice_arr_four = [82931];
        let slice_str_five = "x";
        let slice_arr_five = [1];

        assert!(!contains_subslice(
            full_collecion_str.as_bytes(),
            slice_str_one.as_bytes()
        ));
        assert!(!contains_subslice(
            full_collecion_str.as_bytes(),
            slice_str_two.as_bytes()
        ));
        assert!(!contains_subslice(
            full_collecion_str.as_bytes(),
            slice_str_three.as_bytes()
        ));
        assert!(!contains_subslice(
            full_collecion_str.as_bytes(),
            slice_str_four.as_bytes()
        ));
        assert!(!contains_subslice(
            full_collecion_str.as_bytes(),
            slice_str_five.as_bytes()
        ));
        assert!(!contains_subslice(&full_collection_arr, &slice_arr_one));
        assert!(!contains_subslice(&full_collection_arr, &slice_arr_two));
        assert!(!contains_subslice(&full_collection_arr, &slice_arr_three));
        assert!(!contains_subslice(&full_collection_arr, &slice_arr_four));
        assert!(!contains_subslice(&full_collection_arr, &slice_arr_five));
    }
}
