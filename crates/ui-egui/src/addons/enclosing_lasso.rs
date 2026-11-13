use pluma_tools::addons::enclosing_lasso;
use serde_json::json;

use crate::menus::Item;

pub fn menu() -> Vec<Item> {
    vec![Item::Cmd("Enclosing Lasso Tool", "tool.select", json!({"tool": enclosing_lasso::ID}))]
}
