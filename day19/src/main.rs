use std::collections::{BTreeSet, HashSet};
use std::ops::Add;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    pub fn under_all_orientations(self) -> [Coord; 24] {
        let Coord { x, y, z} = self;
        [
            (x, y, z).into(),
            (x, z, -y).into(),
            (x, -y, -z).into(),
            (x, -z, y).into(),
            (-x, y, -z).into(),
            (-x, -z, -y).into(),
            (-x, -y, z).into(),
            (-x, z, y).into(),
            (y, -x, z).into(),
            (y, z, x).into(),
            (y, x, -z).into(),
            (y, -z, -x).into(),
            (-y, -x, -z).into(),
            (-y, -z, x).into(),
            (-y, x, z).into(),
            (-y, z, -x).into(),
            (z, x, y).into(),
            (z, y, -x).into(),
            (z, -x, -y).into(),
            (z, -y, x).into(),
            (-z, y, x).into(),
            (-z, x, -y).into(),
            (-z, -y, -x).into(),
            (-z, -x, y).into(),
        ]
    }
}

impl From<(i64, i64, i64)> for Coord {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        Self { x, y, z }
    }
}

impl Add<(i64, i64, i64)> for Coord {
    type Output = Coord;

    fn add(self, (x, y, z): (i64, i64, i64)) -> Self::Output {
        Coord { x: self.x + x, y: self.y + y, z: self.z + z }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScannerReport {
    beacons: HashSet<Coord>
}

impl ScannerReport {
    pub fn overlap_with(&self, other: &ScannerReport) -> Option<(ScannerReport, ScannerReport)> {
        let orientations = other.beacons.iter().map(|c| c.under_all_orientations()).collect::<Vec<_>>();
        let our_xs = self.beacons.iter().map(|c| c.x).collect::<BTreeSet<_>>();
        let x_min = our_xs.first().unwrap();
        let x_max = our_xs.last().unwrap();

        for i in 0..24 {
            let beacons_in_orientation = orientations.iter().map(|o| o[i]).collect::<HashSet<_>>();
            let beacon_xs = beacons_in_orientation.iter().map(|c| c.x).collect::<BTreeSet<_>>();
            let other_x_min = *beacon_xs.first().unwrap();
            let other_x_max = *beacon_xs.last().unwrap();

            for potential_dx in other_x_min - x_max ..= other_x_max - x_min {
                let translated_xs = beacon_xs.iter().map(|x| x - potential_dx).collect::<BTreeSet<_>>();
                let x_intersection = translated_xs.intersection(&our_xs).collect::<BTreeSet<_>>();
                if x_intersection.len() < 12 {
                    continue; 
                }

                // we might have found an overlap
                // println!("With offset of {} we get x overlap", potential_dx);
                let other_translated_beacons_in_x_overlap = beacons_in_orientation.iter().map(|b| *b + (-potential_dx, 0, 0)).filter(|c| x_intersection.contains(&c.x)).collect::<HashSet<_>>();
                let our_beacons_in_x_overlap = self.beacons.iter().filter(|c| x_intersection.contains(&c.x)).collect::<HashSet<_>>();
                let our_ys = our_beacons_in_x_overlap.iter().map(|c| c.y).collect::<BTreeSet<_>>();
                // println!("Our beacons are {:?} with ys {:?}", our_beacons_in_x_overlap, our_ys);
                let y_min = our_ys.first().unwrap();
                let y_max = our_ys.last().unwrap();
                let beacon_ys = other_translated_beacons_in_x_overlap.iter().map(|c| c.y).collect::<BTreeSet<_>>();
                // println!("Other beacons are {:?} with ys {:?}", other_translated_beacons_in_x_overlap, beacon_ys);
                let other_y_min = *beacon_ys.first().unwrap();
                let other_y_max = *beacon_ys.last().unwrap();

                for potential_dy in other_y_min - y_max ..= other_y_max - y_min {
                    let translated_ys = beacon_ys.iter().map(|y| y - potential_dy).collect::<BTreeSet<_>>();
                    let y_intersection = translated_ys.intersection(&our_ys).collect::<BTreeSet<_>>();
                    if y_intersection.len() < 12 {
                        // the x worked but this y didn't, alas
                        continue; 
                    }

                    // we STILL might have found an overlap
                    let other_translated_beacons_in_y_overlap = beacons_in_orientation.iter().map(|b| *b + (-potential_dx, -potential_dy, 0)).filter(|c| y_intersection.contains(&c.y) && x_intersection.contains(&c.x)).collect::<HashSet<_>>();
                    let our_beacons_in_y_overlap = self.beacons.iter().filter(|c| y_intersection.contains(&c.y) && x_intersection.contains(&c.x)).collect::<HashSet<_>>();
                    let our_zs = our_beacons_in_y_overlap.iter().map(|c| c.z).collect::<BTreeSet<_>>();
                    let z_min = our_zs.first().unwrap();
                    let z_max = our_zs.last().unwrap();
                    let beacon_zs = other_translated_beacons_in_y_overlap.iter().map(|c| c.z).collect::<BTreeSet<_>>();
                    let other_z_min = *beacon_zs.first().unwrap();
                    let other_z_max = *beacon_zs.last().unwrap();

                    for potential_dz in other_z_min - z_max ..= other_z_max - z_min {
                        let translated_zs = beacon_zs.iter().map(|z| z - potential_dz).collect::<BTreeSet<_>>();
                        let z_intersection = translated_zs.intersection(&our_zs).collect::<BTreeSet<_>>();
                        if z_intersection.len() < 12 {
                            // tx and y worked but z didn't, super alas
                            continue; 
                        }

                        // println!("Found overlap of {:?}, {:?}, {:?} at {}, {}, {}", x_intersection, y_intersection, z_intersection, potential_dx, potential_dy, potential_dz);

                        let mut other_beacons_translated = beacons_in_orientation.into_iter().map(|c| c + (- potential_dx, - potential_dy, - potential_dz)).collect::<HashSet<_>>();
                        other_beacons_translated.extend(self.beacons.clone());
                        let combined_report = ScannerReport { beacons: other_beacons_translated };
                        let overlap = self.beacons.iter().copied().filter(|c| y_intersection.contains(&c.y) && x_intersection.contains(&c.x) && z_intersection.contains(&c.z)).collect::<HashSet<_>>();
                        return Some((ScannerReport { beacons: overlap }, combined_report));
                    }
                }
            }
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    reports: Vec<ScannerReport>,
}

impl Input {
    pub fn full_overlap(&self) -> ScannerReport {
        let mut combined = self.reports.clone();
        while combined.len() > 1 {
            let mut new = combined.clone();
            for (first, second) in combined.iter().enumerate().tuple_combinations() {
                if let Some((_, union)) = first.1.overlap_with(second.1) {
                    new[first.0] = union;
                    new.remove(second.0);
                    break;
                }
            }

            combined = new;
        }

        ScannerReport { beacons: combined[0].beacons.clone() }
    }
}

pub fn parse_input(input: &str) -> Input {
    Input { reports: input.split("\n\n").map(|scanner| {
            let mut lines = scanner.lines();
            _ = lines.next();
            ScannerReport { beacons: lines.map(|line| {
                let mut nums = line.split(",").map(|n| n.parse().unwrap());
                let x = nums.next().unwrap();
                let y = nums.next().unwrap();
                let z = nums.next().unwrap();
                (x, y, z).into()
            }).collect() }
        }).collect()
    }
}


fn main() {
    let input = include_str!("../input.txt");
    let input = parse_input(input.trim());
    // println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
let input = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390"#;

    let overlap_relative_to_0 = r#"--- overlap 0 ---
-618,-824,-621
-537,-823,-458
-447,-329,318
404,-588,-901
544,-627,-890
528,-643,409
-661,-816,-575
390,-675,-793
423,-701,434
-345,-311,381
459,-707,401
-485,-357,347
"#;

    let overlap_relative_to_1 = r#"--- overlap 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
-476,619,847
-460,603,-452
729,430,532
-322,571,750
-355,545,-477
413,935,-424
-391,539,-444
553,889,-390
"#;

    let input = parse_input(input);
    let overlap_relative_to_0 = &parse_input(overlap_relative_to_0).reports[0];
    let overlap_relative_to_1 = &parse_input(overlap_relative_to_1).reports[0];
    let scanner_0 = &input.reports[0];
    let scanner_1 = &input.reports[1];
    let get_overlap_0 = scanner_0.overlap_with(scanner_1).unwrap();
    let get_overlap_1 = scanner_1.overlap_with(scanner_0).unwrap();
    assert_eq!(&get_overlap_0.0, overlap_relative_to_0);
    assert_eq!(&get_overlap_1.0, overlap_relative_to_1);

}


#[test]
pub fn test_full() {
let input = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
"#;

    let expected_overlap = r#"--- overlap 0 ---
-892,524,684
-876,649,763
-838,591,734
-789,900,-551
-739,-1745,668
-706,-3180,-659
-697,-3072,-689
-689,845,-530
-687,-1600,576
-661,-816,-575
-654,-3158,-753
-635,-1737,486
-631,-672,1502
-624,-1620,1868
-620,-3212,371
-618,-824,-621
-612,-1695,1788
-601,-1648,-643
-584,868,-557
-537,-823,-458
-532,-1715,1894
-518,-1681,-600
-499,-1607,-770
-485,-357,347
-470,-3283,303
-456,-621,1527
-447,-329,318
-430,-3130,366
-413,-627,1469
-345,-311,381
-36,-1284,1171
-27,-1108,-65
7,-33,-71
12,-2351,-103
26,-1119,1091
346,-2985,342
366,-3059,397
377,-2827,367
390,-675,-793
396,-1931,-563
404,-588,-901
408,-1815,803
423,-701,434
432,-2009,850
443,580,662
455,729,728
456,-540,1869
459,-707,401
465,-695,1988
474,580,667
496,-1584,1900
497,-1838,-617
527,-524,1933
528,-643,409
534,-1912,768
544,-627,-890
553,345,-567
564,392,-477
568,-2007,-577
605,-1665,1952
612,-1593,1893
630,319,-379
686,-3108,-505
776,-3184,-501
846,-3110,-434
1135,-1161,1235
1243,-1093,1063
1660,-552,429
1693,-557,386
1735,-437,1738
1749,-1800,1813
1772,-405,1572
1776,-675,371
1779,-442,1789
1780,-1548,337
1786,-1538,337
1847,-1591,415
1889,-1729,1762
1994,-1805,1792
"#;


    let input = parse_input(input);
    let expected_overlap = &parse_input(expected_overlap).reports[0];

    let overlap = input.full_overlap();
    println!("{:?}", overlap);
    assert_eq!(&overlap, expected_overlap);

}