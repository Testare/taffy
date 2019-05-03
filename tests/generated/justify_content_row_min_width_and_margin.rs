#[test]
fn justify_content_row_min_width_and_margin() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(20f32),
                height: stretch::style::Dimension::Points(20f32),
                ..Default::default()
            },
            margin: stretch::geometry::Rect { start: stretch::style::Dimension::Points(10f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            justify_content: stretch::style::JustifyContent::Center,
            min_size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50f32), ..Default::default() },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 50f32);
    assert_eq!(stretch.layout(node).size.height, 20f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 20f32);
    assert_eq!(stretch.layout(node0).size.height, 20f32);
    assert_eq!(stretch.layout(node0).location.x, 20f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
}
