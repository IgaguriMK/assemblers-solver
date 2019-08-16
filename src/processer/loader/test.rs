use super::*;

const NO_BEACON: Configulation = Configulation { beacon: 0, io: 8 };
const BEACON4: Configulation = Configulation { beacon: 4, io: 6 };
const BEACON8: Configulation = Configulation { beacon: 8, io: 4 };

#[test]
fn with_assembler() {
    let samples = vec![
        ((0, 0), &NO_BEACON, 1.0, 1.25),
        ((1, 0), &NO_BEACON, 1.1, 1.0625),
        ((3, 1), &NO_BEACON, 1.3, 1.3125),
        ((4, 0), &BEACON4, 1.4, 3.0),
        ((4, 0), &BEACON8, 1.4, 5.5),
    ];

    for (mods, conf, prod_tobe, speed_tobe) in samples {
        let p = build_proc("assembler3", "assembler", 1.25, 3, &mods, conf);

        assert_eq!(
            p.productivity(),
            prod_tobe,
            "mismatch prod with (p, s) = {:?}, beacon = {}",
            &mods,
            conf.beacon
        );
        assert_eq!(
            p.speed(),
            speed_tobe,
            "mismatch speed with (p, s) = {:?}, beacon = {}",
            &mods,
            conf.beacon
        );
    }
}

#[test]
fn with_furnace() {
    let samples = vec![
        ((0, 0), &NO_BEACON, 1.0, 2.0),
        ((1, 0), &NO_BEACON, 1.1, 1.7),
        ((1, 1), &NO_BEACON, 1.1, 2.7),
        ((2, 0), &NO_BEACON, 1.2, 1.4),
        ((2, 0), &BEACON4, 1.2, 5.4),
        ((2, 0), &BEACON8, 1.2, 9.4),
    ];

    for (mods, conf, prod_tobe, speed_tobe) in samples {
        let p = build_proc("furnace", "furnace", 2.0, 0, &mods, conf);

        assert_eq!(
            p.productivity(),
            prod_tobe,
            "mismatch prod with (p, s) = {:?}, beacon = {}",
            &mods,
            conf.beacon
        );
        assert_eq!(
            p.speed(),
            speed_tobe,
            "mismatch speed with (p, s) = {:?}, beacon = {}",
            &mods,
            conf.beacon
        );
    }
}
