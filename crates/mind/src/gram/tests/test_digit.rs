use crate::gram::Digit;


#[test]
fn digit_try_from_f32() {
    assert_eq!(format!("{:?}", Digit::try_from_f32(0.0, 16)), "Med('0')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(1.0, 16)), "Med('1')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(2.0, 16)), "Med('2')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(3.0, 16)), "Med('3')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(4.0, 16)), "Med('4')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(5.0, 16)), "Med('5')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(6.0, 16)), "Med('6')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(7.0, 16)), "Med('7')");

    assert_eq!(format!("{:?}", Digit::try_from_f32(-0.0, 16)), "Med('0')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(-1.0, 16)), "Med('f')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(-2.0, 16)), "Med('e')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(-3.0, 16)), "Med('d')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(-4.0, 16)), "Med('c')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(-5.0, 16)), "Med('b')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(-6.0, 16)), "Med('a')");
    assert_eq!(format!("{:?}", Digit::try_from_f32(-7.0, 16)), "Med('9')");
}
