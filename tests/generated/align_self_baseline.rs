#[test]
fn align_self_baseline() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            align_self: stretch::style::AlignSelf::Baseline,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node10 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            align_self: stretch::style::AlignSelf::Baseline,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node10],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0, node1],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 100f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 50f32);
    assert_eq!(stretch.layout(node0).size.height, 50f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node1).size.width, 50f32);
    assert_eq!(stretch.layout(node1).size.height, 20f32);
    assert_eq!(stretch.layout(node1).location.x, 50f32);
    assert_eq!(stretch.layout(node1).location.y, 40f32);
    assert_eq!(stretch.layout(node10).size.width, 50f32);
    assert_eq!(stretch.layout(node10).size.height, 10f32);
    assert_eq!(stretch.layout(node10).location.x, 0f32);
    assert_eq!(stretch.layout(node10).location.y, 0f32);
}
