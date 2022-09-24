use crate::CategoricalArray;

#[test]
fn create_array() {
    let mut cat_array = CategoricalArray::new();
    cat_array.push("Europe");
    cat_array.push("America");
    cat_array.push("Europe");
    assert_eq!(cat_array.len(), 3);
    assert_eq!(cat_array.number_of_categories(), 2);
}

#[test]
fn simple_iterate() {
    let mut cat_array = CategoricalArray::new();
    cat_array.push("Europe".to_owned());
    cat_array.push("America".to_owned());
    cat_array.push("Europe".to_owned());
    let mut it = cat_array.iter();
    assert_eq!(it.next(), Some(&"Europe".to_owned()));
    assert_eq!(it.next(), Some(&"America".to_owned()));
    assert_eq!(it.next(), Some(&"Europe".to_owned()));
    assert_eq!(it.next(), None);
}
