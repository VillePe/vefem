use crate::structure::element::Element;

pub fn get_element_release_count(elements: &Vec<Element>) -> usize {
    let mut count = 0;
    for e in elements {
        if e.releases.s_tx { count += 1; }
        if e.releases.s_tz { count += 1; }
        if e.releases.s_ry { count += 1; }
        if e.releases.e_tx { count += 1; }
        if e.releases.e_tz { count += 1; }
        if e.releases.e_ry { count += 1; }
    }
    count
}