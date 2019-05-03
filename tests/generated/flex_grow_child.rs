#[test]
fn flex_grow_child() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            flex_grow: 1f32,
            flex_basis: stretch::style::Dimension::Points(0f32),
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(stretch::style::Style { ..Default::default() }, vec![node0]);
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 0f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 0f32);
    assert_eq!(stretch.layout(node0).size.height, 100f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
}
