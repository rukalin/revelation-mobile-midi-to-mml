use crate::utils::{self, MMLNote};

#[test]
fn test_get_list_of_mml_notes() {
    assert_eq!(
        vec![
            MMLNote { duration_in_smallest_unit: 64, mml_value: 1 },
            MMLNote { duration_in_smallest_unit: 32, mml_value: 2 },
            MMLNote { duration_in_smallest_unit: 16, mml_value: 4 },
            MMLNote { duration_in_smallest_unit: 8, mml_value: 8 },
            MMLNote { duration_in_smallest_unit: 4, mml_value: 16 },
            MMLNote { duration_in_smallest_unit: 2, mml_value: 32 },
            MMLNote { duration_in_smallest_unit: 1, mml_value: 64 },
        ],
        utils::get_list_of_mml_notes(64),
    );

    assert_eq!(
        vec![
            MMLNote { duration_in_smallest_unit: 128, mml_value: 1 },
            MMLNote { duration_in_smallest_unit: 64, mml_value: 2 },
            MMLNote { duration_in_smallest_unit: 32, mml_value: 4 },
            MMLNote { duration_in_smallest_unit: 16, mml_value: 8 },
            MMLNote { duration_in_smallest_unit: 8, mml_value: 16 },
            MMLNote { duration_in_smallest_unit: 4, mml_value: 32 },
            MMLNote { duration_in_smallest_unit: 2, mml_value: 64 },
            MMLNote { duration_in_smallest_unit: 1, mml_value: 128 },
        ],
        utils::get_list_of_mml_notes(128),
    );
}

#[test]
fn test_get_display_mml() {
    let note_class = &"r".to_string();

    assert_eq!(
        String::from("r64"),
        utils::get_display_mml(1, note_class),
    );

    assert_eq!(
        String::from("r32"),
        utils::get_display_mml(2, note_class),
    );

    assert_eq!(
        String::from("r8&r64"),
        utils::get_display_mml(9, note_class),
    );

    assert_eq!(
        String::from("r1.&r16"),
        utils::get_display_mml(100, note_class),
    );

    assert_eq!(
        String::from("r1&r4."),
        utils::get_display_mml(88, note_class),
    );

    assert_eq!(
        String::from("r2&r8."),
        utils::get_display_mml(44, note_class),
    );
}
