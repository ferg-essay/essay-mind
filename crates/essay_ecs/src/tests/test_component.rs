use std::any::TypeId;

use essay_ecs_macros::Component;

use crate::component::{ComponentInfos};

#[test]
fn empty_info_set() {
    let info = ComponentInfos::new();

    assert_eq!(info.len(), 0);
}

#[test]
fn add_info() {
    let mut info = ComponentInfos::new();

    assert_eq!(info.len(), 0);

    let id = info.add_info::<TestA>();
    assert_eq!(id.index(), 0);
    assert_eq!(info.len(), 1);

    assert_eq!(info.get_name(id).expect("id"), "essay_ecs::tests::test_component::TestA");
    assert_eq!(info.get_id_by_type(TypeId::of::<TestA>()).expect("id"), id);

    let id_b = info.add_info::<TestB>();
    assert_eq!(id_b.index(), 1);
    assert_eq!(info.len(), 2);
    assert_eq!(info.get_id_by_type(TypeId::of::<TestB>()).expect("id"), id_b);

    let id_a2 = info.add_info::<TestA>();
    assert_eq!(id.index(), 0);
    assert_eq!(info.len(), 2);
    assert_eq!(id, id_a2);
}

#[derive(Component)]
struct TestA {
}

#[derive(Component)]
struct TestB {
}
