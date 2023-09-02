#[cfg(test)]
use crate::find_n_0_hash;
#[test]
fn hash_find_n3_f6() {
    assert_eq!(
        find_n_0_hash(3, 6),
        vec![
            (
                4163,
                String::from("95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000")
            ),
            (
                11848,
                String::from("cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000")
            ),
            (
                12843,
                String::from("bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000")
            ),
            (
                13467,
                String::from("42254207576dd1cfb7d0e4ceb1afded40b5a46c501e738159d8ac10b36039000")
            ),
            (
                20215,
                String::from("1f463eb31d6fa7f3a7b37a80f9808814fc05bf10f01a3f653bf369d7603c8000")
            ),
            (
                28892,
                String::from("dab12874ecae90c0f05d7d87ed09921b051a586c7321850f6bb5e110bc6e2000")
            )
        ]
    );
}

#[test]
fn hash_find_n5_f3() {
    assert_eq!(
        find_n_0_hash(5, 3),
        vec![
            (
                828028,
                String::from("d95f19b5269418c0d4479fa61b8e7696aa8df197082b431a65ff37595c100000")
            ),
            (
                2513638,
                String::from("862d4525b0b60779d257be2b3920b90e3dbcd60825b86cfc6cffa49a63c00000")
            ),
            (
                3063274,
                String::from("277430daee71c67b356dbb81bb0a39b6d53efd19d14177a173f2e05358a00000")
            )
        ]
    )
}

#[test]
fn hash_find_n1_f0() {
    assert_eq!(find_n_0_hash(1, 0), Vec::new())
}

#[test]
#[should_panic]
fn hash_find_n0_f1() {
    find_n_0_hash(0, 1);
}
