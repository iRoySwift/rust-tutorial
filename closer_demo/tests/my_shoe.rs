use closer_demo::my_shoe;

#[test]
fn filter_by_size() {
    let shoes = vec![
        my_shoe::Shoe {
            size: 10,
            style: String::from("snakers"),
        },
        my_shoe::Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        my_shoe::Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = my_shoe::shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            my_shoe::Shoe {
                size: 10,
                style: String::from("snakers"),
            },
            my_shoe::Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    )
}
