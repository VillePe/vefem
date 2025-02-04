use std::str::Split;
use crate::loads;
use crate::loads::load::Load;
use crate::structure::Element;

/// Gets the element numbers that are linked to given load. Different elements are separated with , (comma).
/// 
/// For multiple elements a 'S..E' (double dots with numbers before and after it) can be used.
///
/// If all load should be linked to all elements, -1 can be used (e.g. for self weight load) 
pub fn get_linked_element_numbers(load: &Load) -> Vec<i32> {
    let split = load.element_numbers.split(",");
    let mut result : Vec<i32> = Vec::new();

    for s in split {
        if s.is_empty() { continue; }
        if s.contains("..") {
            // Split the S..E into S and E and collect it into a vector
            let range_split : Vec<&str> = s.split("..").collect();
            // If there are not exactly two objects, invalidate the value and continue.
            if range_split.len() != 2 { continue; }
            let num_begin : i32 = vputilslib::vputils::s_to_int(range_split[0]).unwrap() as i32;
            let num_end : i32 = vputilslib::vputils::s_to_int(range_split[1]).unwrap() as i32;
            for i in num_begin..=num_end {
                result.push(i);
            }
        } else {
            // Parse the numbers if there are no '..' modifier in the given string
            let num : i32 = vputilslib::vputils::s_to_int(s).unwrap() as i32;
            result.push(num);
        }
    }
    result
}

/// Checks if the given load is linked to given element by comparing the elements number to 'element_numbers' in [`Load`]
pub fn load_is_linked(elem: &Element, load: &Load) -> bool {
    let linked_elements = get_linked_element_numbers(&load);
    linked_elements.contains(&-1) || linked_elements.contains(&elem.number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_get_element_list() {
        let load1 = Load{element_numbers: "1,2,3".to_string(), ..Load::default()};
        let result1 = get_linked_element_numbers(&load1);
        assert_eq!(vec![1,2,3], result1);

        let load2 = Load{element_numbers: "1,2,6,8".to_string(), ..Load::default()};
        let result2 = get_linked_element_numbers(&load2);
        assert_eq!(vec![1,2,6,8], result2);

        let load3 = Load{element_numbers: "1,3..6,8".to_string(), ..Load::default()};
        let result3 = get_linked_element_numbers(&load3);
        assert_eq!(vec![1,3,4,5,6,8], result3);

        let load4 = Load{element_numbers: "-1".to_string(), ..Load::default()};
        let result4 = get_linked_element_numbers(&load4);
        assert_eq!(vec![-1], result4);
    }
}