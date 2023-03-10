use crate::gram::{gram_map::GramMap, gram};

#[test]
fn test_map_base() {
    let mut map = GramMap::<i64>::new();

    assert!(map.get(&gram("a")).is_none());

    assert!(map.insert(gram("a"), 10).is_none());
    assert!(! map.get(&gram("a")).is_none());

    assert!(map.get(&gram("a")).expect("missing value") == &10);
    assert!(map.get(&gram("?a")).expect("missing value") == &10);
    assert!(map.get(&gram("+a")).expect("missing value") == &10);
    assert!(map.get(&gram("!a")).expect("missing value") == &10);

    assert!(map.get(&gram("0")).is_none());

    assert!(map.insert(gram("!b?c+d"), 20).is_none());
    assert!(map.get(&gram("bcd")).expect("missing value") == &20);
    assert!(map.get(&gram("?b?c?d")).expect("missing value") == &20);
    assert!(map.get(&gram("!b!c!d")).expect("missing value") == &20);
    assert!(map.get(&gram("+b+c+d")).expect("missing value") == &20);
}