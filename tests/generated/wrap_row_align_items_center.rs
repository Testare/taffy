#[test]
fn wrap_row_align_items_center() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(30f32),
                height: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(30f32),
                height: stretch::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node2 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(30f32),
                height: stretch::style::Dimension::Points(30f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node3 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(30f32),
                height: stretch::style::Dimension::Points(30f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            flex_wrap: stretch::style::FlexWrap::Wrap,
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        vec![node0, node1, node2, node3],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 100f32);
    assert_eq!(stretch.layout(node).size.height, 60f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 30f32);
    assert_eq!(stretch.layout(node0).size.height, 10f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 10f32);
    assert_eq!(stretch.layout(node1).size.width, 30f32);
    assert_eq!(stretch.layout(node1).size.height, 20f32);
    assert_eq!(stretch.layout(node1).location.x, 30f32);
    assert_eq!(stretch.layout(node1).location.y, 5f32);
    assert_eq!(stretch.layout(node2).size.width, 30f32);
    assert_eq!(stretch.layout(node2).size.height, 30f32);
    assert_eq!(stretch.layout(node2).location.x, 60f32);
    assert_eq!(stretch.layout(node2).location.y, 0f32);
    assert_eq!(stretch.layout(node3).size.width, 30f32);
    assert_eq!(stretch.layout(node3).size.height, 30f32);
    assert_eq!(stretch.layout(node3).location.x, 0f32);
    assert_eq!(stretch.layout(node3).location.y, 30f32);
}
