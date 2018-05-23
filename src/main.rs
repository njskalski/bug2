extern crate cursive;
extern crate cursive_tree_view;


use cursive::Cursive;
use cursive::traits::*;
use cursive::views::*;
use cursive::direction::Orientation;
use cursive_tree_view::*;


fn main() {
    let mut siv = Cursive::new();

    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.

    let mut ll1 = LinearLayout::new(Orientation::Horizontal);

    let mut dir_tree : TreeView<String> = TreeView::new();
    dir_tree.insert_container_item("root".to_string(), Placement::LastChild, 0);
    dir_tree.set_collapsed(0, true);
    ll1.add_child(BoxView::with_fixed_size((30, 30), dir_tree));

    let mut ll2 = LinearLayout::new(Orientation::Vertical);

    let mut sel_view = SelectView::new();
    sel_view.add_item_str("a1");
    sel_view.add_item_str("a2");
    sel_view.add_item_str("a3");

    ll2.add_child(BoxView::with_fixed_size((30, 30), sel_view));


    ll2.add_child(BoxView::with_fixed_size((30, 30), EditView::new()
        .on_submit(show_popup)
        .with_id("name3")
        .fixed_width(20)));


    ll1.add_child(ll2);

    siv.add_layer(ll1);

    siv.run();
}

// This will replace the current layer with a new popup.
// If the name is empty, we'll show an error message instead.
fn show_popup(s: &mut Cursive, name: &str) {
    if name.is_empty() {
        s.add_layer(Dialog::info("Please enter a name!"));
    } else {
        let content = format!("Hello {}!", name);
        s.pop_layer();
        s.add_layer(
            Dialog::around(TextView::new(content))
                .button("Quit", |s| s.quit()),
        );
    }
}
