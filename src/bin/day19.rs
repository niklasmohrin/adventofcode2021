use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use counter::Counter;
use itertools::Itertools;
use nalgebra::{Matrix3, Point3, Vector3};
use serde_scan::scan;

type Point = Point3<i32>;

fn main() {
    let input = {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        s
    };

    let mut unchecked_scanners: HashMap<usize, Vec<Point>> = input
        .split("\n\n")
        .map(|s| {
            let (header, coords) = s.split_once('\n').unwrap();
            let id = scan!("--- scanner {} ---" <- header).unwrap();
            let points = coords
                .lines()
                .map(|l| {
                    let (x, y, z) = scan!("{},{},{}" <- l).unwrap();
                    Point::from([x, y, z])
                })
                .collect();
            (id, points)
        })
        .collect();

    let all_directions = [
        Vector3::<i32>::x_axis(),
        Vector3::y_axis(),
        Vector3::z_axis(),
    ]
    .into_iter()
    .flat_map(|ax| [ax, -ax])
    .collect_vec();
    let all_orientations = all_directions
        .iter()
        .cartesian_product(all_directions.iter())
        .filter(|(a1, a2)| a1.abs() != a2.abs())
        .map(|(a1, a2)| Matrix3::from_columns(&[a1.into_inner(), a2.into_inner(), a1.cross(a2)]))
        .collect_vec();

    let fingerprint = |points: &[Point]| {
        points
            .iter()
            .cartesian_product(points.iter())
            .filter(|(p1, p2)| p1 != p2)
            .map(|(p1, p2)| p1 - p2)
            .collect_vec()
    };

    let mut correct_scanners = HashMap::new();
    correct_scanners.insert(0, unchecked_scanners.remove(&0).unwrap());
    let mut translation_for_scanner = HashMap::new();
    translation_for_scanner.insert(0, Vector3::identity());

    while !unchecked_scanners.is_empty() {
        let mut correct_rot: Option<Matrix3<i32>> = None;
        let mut matched_points = None;
        let (&correct_id, _) = unchecked_scanners
            .iter()
            .find(|(_, points)| {
                correct_rot = all_orientations
                    .iter()
                    .find(|rot| {
                        let rotated = points.iter().map(|p| *rot * p).collect_vec();
                        let fp = fingerprint(&rotated);
                        matched_points = correct_scanners.values().find(|other_ps| {
                            fingerprint(other_ps)
                                .into_iter()
                                .filter(|v| fp.contains(v))
                                .count()
                                >= 12 * 11
                        });
                        matched_points.is_some()
                    })
                    .cloned();

                correct_rot.is_some()
            })
            .unwrap();

        dbg!(correct_id);
        let correct_rot = correct_rot.unwrap();
        let matched_points = matched_points.unwrap();

        let mut points = unchecked_scanners.remove(&correct_id).unwrap();
        points.iter_mut().for_each(|p| *p = correct_rot * *p);

        // translate the points
        let all_translations: Counter<Vector3<_>> = matched_points
            .iter()
            .cartesian_product(points.iter())
            .map(|(p1, p2)| p1 - p2)
            .collect();
        let translation = all_translations.most_common().first().unwrap().0;

        translation_for_scanner.insert(correct_id, translation);
        points.iter_mut().for_each(|p| *p += translation);

        correct_scanners.insert(correct_id, points);
    }

    let all_beacons: HashSet<Point> = correct_scanners.values().cloned().flatten().collect();
    dbg!(all_beacons.len());

    let max_scanner_dist: i32 = translation_for_scanner
        .values()
        .cartesian_product(translation_for_scanner.values())
        .map(|(v1, v2)| {
            v1.iter()
                .zip(v2.iter())
                .map(|(s1, s2)| (s1 - s2).abs())
                .sum()
        })
        .max()
        .unwrap();
    dbg!(max_scanner_dist);
}
