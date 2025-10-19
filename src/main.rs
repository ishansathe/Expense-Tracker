
slint::slint!{
    import { Button, LineEdit } from "std-widgets.slint";
    import { TopCredits, BottomCredits } from "src/slint_files/menu.slint";
    import { CustomButton } from "src/slint_files/custom_button.slint";
    import { MarkPage } from "src/slint_files/Mark_Expenses/layout_text_button.slint";
    import { ViewPage, ItemDetail } from "src/slint_files/View_Expenses/view.slint";
    import { TotalPage } from "src/slint_files/Calculate_Total/total.slint";
    import { Category } from "src/slint_files/View_Expenses/items.slint";

    export component Box inherits Window {
        width: 600px;
        height: 400px;
        background: #e6f5e6;
        in-out property <string> amt;
        in-out property <string> name;

        // This is string because i thought it is quite simple to accept input as string 
        // and have it stored as string within the JSON object.
        in-out property <string> category;

        in-out property <[ItemDetail]> item_details <=> viewpage.itemDetails;

        callback clicked <=> markpage.submit;

        // I want this callback to be made by the backend 
        // when the 'view expenses' button -> 'view_btn_touch'  is clicked.
        callback getExpenses();

        in-out property <int> total <=> totalpage.total;
        callback getTotal();

        // The 'getExpenses' callback will initiate a function at the backend
        // within which, the item details will be read and fed into the function 'callExpenses'
        public function updateExpenses(itemInfo: [ItemDetail]) {
            viewpage.itemDetails = itemInfo;
        }

        public function _resetData(){
            markpage.resetFields();
        }

        SideBarMenu:= Rectangle {
            x: 0;
            y: 0;
            width: 100px;
            height: parent.height;
            background: #86C887;

            TopCredits{}

            mark := CustomButton {
                x: 0;
                y: 50px;
                text: "Mark Expenses";
                TouchArea {
                    clicked => {
                        mark.active = true;
                        view.active = false;
                        total.active = false;
                        markpage.visible = true;
                        viewpage.visible = false;
                        totalpage.visible = false;
                    }
                }
            }

            view := CustomButton {
                x:0;
                y:100px;
                text: "View Expenses";

                // Directly calling this element.
                view_btn_touch:= TouchArea {
                    clicked => {
                        parent.active = true;
                        mark.active = false;
                        total.active = false;
                        markpage.visible = false;
                        viewpage.visible = true;
                        totalpage.visible = false;

                        // I changed the approach. Now the callback is called when the button is clicked
                        // Instead of it being an alias and then being overridden in the backend.
                        root.getExpenses();
                    }
                }
            }

            total:= CustomButton{
                x: 0;
                y: 150px;
                text: "View Total";

                view_total_touch:= TouchArea{
                    clicked => {
                        mark.active = false;
                        view.active = false;
                        parent.active = true;
                        markpage.visible = false;
                        viewpage.visible = false;
                        totalpage.visible = true;

                        // Callback is called when this button is clicked.
                        root.getTotal();
                    }
                }
            }

            BottomCredits {}
        }


        public function changeInfo(newText: string) -> bool {
            markpage.info = newText;
            return true;
        }

        markpage:= MarkPage {
            changed item_name => {
                root.name = self.item_name;
            }
            changed item_cost => {
                root.amt = self.item_cost;
            }
            changed item_category => {
                root.category = self.item_category;
            }
        }

        viewpage:= ViewPage {}

        totalpage := TotalPage {}

    }
}

mod file_mgmt;
mod calculate_total;
mod serdes;

use slint::{VecModel, ModelRc};
use std::rc::Rc;

fn main() {

    // Box::new().unwrap().run().unwrap();

    let obox = Box::new().unwrap();
    let weak_box = obox.as_weak();

    let weak_clicked = weak_box.clone();
    let weak_get_expenses = weak_box.clone();
    let weak_get_total = weak_box.clone();

    obox.on_clicked(move || {
        println!("Expense Added!");

        let in_box = weak_clicked.upgrade().unwrap();

        if in_box.get_name().is_empty() || in_box.get_amt().is_empty() {
            in_box.invoke_changeInfo("Some or all fields are empty. \nNo data added".into());
            println!("One or all fields are empty");
        }

        if in_box.get_name() != "" && in_box.get_amt() != "" {
            
            let info_text = format!(
                "Added {} at {} Rs in Category {}", 
                in_box.get_name(), in_box.get_amt(), in_box.get_category()
            );
            in_box.invoke_changeInfo(info_text.into());

            println!("{}", in_box.get_category());

            file_mgmt::store_in_file(
                in_box.get_name().into(), 
                in_box.get_amt().into(),
                in_box.get_category().into()
            );
        }

        in_box.invoke__resetData();

    });

    
    obox.on_getExpenses({
        let weak_box_clone = weak_get_expenses.clone();
        move || {
            println!("Retrieving Expenses!");

            let in_box = weak_box_clone.upgrade().unwrap();

            let vec_of_vecs = file_mgmt::read_from_file();

            // Converts the vectors into iterators, then converts iterators into reference strings!
            // No need to string.clone() later! (Or even to take references via memory)
            let item_names0: Vec<&String> = vec_of_vecs[0].iter().collect();
            let item_costs1: Vec<&String> = vec_of_vecs[1].iter().collect();
            let item_category2: Vec<&String> = vec_of_vecs[2].iter().collect();

            let mut item_details_: Vec<ItemDetail> = Vec::with_capacity(item_names0.len());

            for index in 0 .. item_names0.len() {

                // A default category of travel is kept, to avoid errors.
                let mut i_category: Category = Category::Travel;
                if item_category2[index] == "Food" {
                    i_category = Category::Food;
                }
                if item_category2[index] == "Travel" {
                    i_category = Category::Travel;
                }
                if item_category2[index] == "Work" {
                    i_category = Category::Work;
                }
                if item_category2[index] == "Utility" {
                    i_category = Category::Utility;
                }
                if item_category2[index] == "Subscriptions" {
                    i_category = Category::Subscriptions;
                }
                if item_category2[index] == "Entertainment" {
                    i_category = Category::Entertainment;
                }

                item_details_.push(
                        ItemDetail{
                        name: item_names0[index].into(),
                        cost: item_costs1[index].into(),
                        category: i_category
                    }
                )
            }

            let model_rc_version = Rc::new(VecModel::from(item_details_));

            in_box.invoke_updateExpenses(ModelRc::from(model_rc_version));
        }
    });


    obox.on_getTotal({
        let get_total_weak_box_clone = weak_get_total.clone();
        move || {
            println!("Retrieving total!");

            let in_box = get_total_weak_box_clone.upgrade().unwrap();

            let total = calculate_total::get_total();

            in_box.set_total(total);
        }
    });

    obox.run().unwrap();

    file_mgmt::read_from_file();


    // serdes::serialize_struct_into_json();

    println!("Hello, world!");
}

