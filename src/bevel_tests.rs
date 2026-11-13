use super::*;

#[test]
fn control_outline_cuts_all_four_corners_by_four_points() {
    let points = outline(100.0, 24.0, Bevel::Control, 0.0);
    let want = [
        [0.0, 4.0],
        [4.0, 0.0],
        [96.0, 0.0],
        [100.0, 4.0],
        [100.0, 20.0],
        [96.0, 24.0],
        [4.0, 24.0],
        [0.0, 20.0],
    ];
    assert_eq!(points, want);
}

#[test]
fn panel_outline_cuts_only_top_left_and_bottom_right_by_ten_points() {
    let points = outline(360.0, 200.0, Bevel::Panel, 0.0);
    let want = [
        [0.0, 10.0],
        [10.0, 0.0],
        [360.0, 0.0],
        [360.0, 190.0],
        [350.0, 200.0],
        [0.0, 200.0],
    ];
    assert_eq!(points, want);
}

#[test]
fn card_cuts_the_same_corners_as_the_panel_by_eight_points() {
    let cut: Vec<_> = Corner::ALL
        .into_iter()
        .filter(|c| Bevel::Card.cuts(*c))
        .collect();
    assert_eq!(
        (cut, Bevel::Card.size()),
        (vec![Corner::TopLeft, Corner::BottomRight], 8.0)
    );
}

#[test]
fn nested_bevel_is_two_points_on_every_corner() {
    assert!(
        (Bevel::Nested.size() - 2.0).abs() < f32::EPSILON
            && Corner::ALL.into_iter().all(|c| Bevel::Nested.cuts(c))
    );
}

#[test]
fn inset_outline_keeps_the_chamfer_parallel() {
    let points = outline(100.0, 24.0, Bevel::Control, 1.0);
    let [a, b] = [points[0], points[1]];
    // Both ends of the inset top-left diagonal lie on x + y = 4 + sqrt 2.
    let on_line = |[x, y]: [f32; 2]| (x + y - (4.0 + std::f32::consts::SQRT_2)).abs() < 1e-5;
    assert!(on_line(a) && on_line(b) && (a[0] - 1.0).abs() < 1e-6 && (b[1] - 1.0).abs() < 1e-6);
}

#[test]
fn top_left_cap_covers_exactly_the_cut_triangle() {
    let (triangle, band) = cap_geometry(Corner::TopLeft, 4.0, 0.0, 4.0);
    assert_eq!(
        (triangle, band),
        ([[0.0, 0.0], [4.0, 0.0], [0.0, 4.0]], None)
    );
}

#[test]
fn bottom_right_cap_is_the_top_left_cap_mirrored_in_its_box() {
    let (triangle, _) = cap_geometry(Corner::BottomRight, 4.0, 0.0, 4.0);
    assert_eq!(triangle, [[4.0, 4.0], [0.0, 4.0], [4.0, 0.0]]);
}

#[test]
fn rim_band_meets_both_edges_one_rim_width_in() {
    let (_, band) = cap_geometry(Corner::TopLeft, 4.0, 1.0, 5.0);
    let band = band.unwrap();
    assert!((band[1][1] - 1.0).abs() < 1e-6 && (band[2][0] - 1.0).abs() < 1e-6);
}
