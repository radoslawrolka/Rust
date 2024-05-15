// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 scoops left. At 10PM, someone eats it
// all, so there'll be no more left :(
    fn maybe_icecream(time_of_day: u16) -> Option<u16> {
        match time_of_day {
            0..=21 => Some(5),
            22 | 23 => Some(0),
            _ => None,
        }
    }
     
    #[cfg(test)]
    mod tests {
        use super::*;
     
        #[test]
        fn check_icecream() {
            assert_eq!(maybe_icecream(0), Some(5));
            assert_eq!(maybe_icecream(9), Some(5));
            assert_eq!(maybe_icecream(18), Some(5));
            assert_eq!(maybe_icecream(22), Some(0));
            assert_eq!(maybe_icecream(23), Some(0));
            assert_eq!(maybe_icecream(25), None);
        }
     
        #[test]
        fn raw_value() {
            let icecreams = maybe_icecream(12).unwrap();
            assert_eq!(icecreams, 5);
        }
    }