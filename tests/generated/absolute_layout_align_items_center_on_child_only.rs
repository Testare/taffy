#[test]
fn absolute_layout_align_items_center_on_child_only() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            position_type: stretch::style::PositionType::Absolute,
            align_self: stretch::style::AlignSelf::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(60f32),
                height: stretch::style::Dimension::Points(40f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(110f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 110f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 60f32);
    assert_eq!(stretch.layout(node0).size.height, 40f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 30f32);
}
