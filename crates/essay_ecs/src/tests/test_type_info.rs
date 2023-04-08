use std::any::TypeId;

use crate::table::prelude::{TypeMetas};

#[test]
fn empty_info_set() {
    let info = TypeMetas::new();

    assert_eq!(info.len(), 0);
}

#[test]
fn add_info() {
    let mut info = TypeMetas::new();

    assert_eq!(info.len(), 0);

    let id = info.add_type::<TestA>();
    assert_eq!(id.index(), 0);
    assert_eq!(info.len(), 1);

    assert_eq!(info.get_name(id).expect("id"), "essay_ecs::tests::test_type_info::TestA");
    assert_eq!(info.get_id_by_type(TypeId::of::<TestA>()).expect("id"), id);

    let id_b = info.add_type::<TestB>();
    assert_eq!(id_b.index(), 1);
    assert_eq!(info.len(), 2);
    assert_eq!(info.get_id_by_type(TypeId::of::<TestB>()).expect("id"), id_b);

    let id_a2 = info.add_type::<TestA>();
    assert_eq!(id.index(), 0);
    assert_eq!(info.len(), 2);
    assert_eq!(id, id_a2);
}

struct TestA {
}

struct TestB {
}
