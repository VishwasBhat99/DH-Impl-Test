pub fn get_bucket_no(tenor: i64) -> i64 {
    if tenor >= 0 && tenor < 7 {
        1
    } else if tenor >= 7 && tenor <= 14 {
        2
    } else if tenor >= 15 && tenor <= 28 {
        3
    } else if tenor >= 29 && tenor <= 45 {
        4
    } else if tenor >= 46 && tenor <= 60 {
        5
    } else if tenor >= 61 && tenor <= 90 {
        6
    } else if tenor >= 91 && tenor <= 180 {
        7
    } else if tenor >= 181 && tenor <= 183 {
        8
    } else if tenor == 184 {
        9
    } else if tenor >= 185 && tenor <= 270 {
        10
    } else if tenor >= 271 && tenor <= 273 {
        11
    } else if tenor == 274 {
        12
    } else if tenor >= 275 && tenor <= 285 {
        13
    } else if tenor == 286 {
        14
    } else if tenor >= 287 && tenor <= 364 {
        15
    } else if tenor == 365 {
        16
    } else if tenor >= 366 && tenor <= 368 {
        17
    } else if tenor == 369 {
        18
    } else if tenor >= 370 && tenor <= 380 {
        19
    } else if tenor == 381 {
        20
    } else if tenor >= 382 && tenor <= 730 {
        21
    } else if tenor >= 731 && tenor <= 745 {
        22
    } else if tenor == 746 {
        23
    } else if tenor <= 747 && tenor <= 1095 {
        24
    } else if tenor >= 1096 && tenor <= 1825 {
        25
    } else if tenor >= 1826 && tenor <= 2920 {
        26
    } else if tenor >= 2921 && tenor <= 3650 {
        27
    } else if tenor > 3650 {
        28
    } else {
        0
    }
}
