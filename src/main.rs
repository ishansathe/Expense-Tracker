
slint::slint!{
    import { Button, LineEdit } from "std-widgets.slint";
    import { TopCredits, BottomCredits } from "src/slint_files/menu.slint";
    import { CustomButton } from "src/slint_files/custom_button.slint";
    import { MarkPage } from "src/slint_files/Mark_Expenses/layout_text_button.slint";
    import { ViewPage, ItemDetail } from "src/slint_files/View_Expenses/view.slint";

    export component Box inherits Window {
        width: 600px;
        height: 400px;
        background: #e6f5e6;
        in-out property <string> amt;
        in-out property <string> name;
        callback clicked <=> markpage.submit;

        // I want this callback to be made by the backend 
        // when the 'view expenses' button -> 'view_btn_touch'  is clicked.
        callback getExpenses();

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
                        markpage.visible = true;
                        viewpage.visible = false;
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
                        view.active = true;
                        mark.active = false;
                        markpage.visible = false;
                        viewpage.visible = true;

                        // I changed the approach. Now the callback is called when the button is clicked
                        // Instead of it being an alias and then being overridden in the backend.
                        root.getExpenses();
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
        }

        viewpage:= ViewPage {}

    }
}

mod file_mgmt;
mod file_read;

use std::mem;
use slint::{VecModel, ModelRc};
use std::rc::Rc;

fn main() {

    // Box::new().unwrap().run().unwrap();

    let obox = Box::new().unwrap();
    let weak_box = obox.as_weak();

    let weak_clicked = weak_box.clone();
    let weak_get_expenses = weak_box.clone();

    obox.on_clicked(move || {
        println!("Button clicked!");

        let in_box = weak_clicked.upgrade().unwrap();

        if in_box.get_name().is_empty() || in_box.get_amt().is_empty() {
            in_box.invoke_changeInfo("Some or all fields are empty. \nNo data added".into());
            println!("One or all fields are empty");
        }

        if in_box.get_name() != "" && in_box.get_amt() != "" {
            
            let info_text = format!(
                "Added {} at {} Rs", 
                in_box.get_name(), in_box.get_amt()
            );
            in_box.invoke_changeInfo(info_text.into());

            file_mgmt::write_to_files(
                in_box.get_name().into(), 
                in_box.get_amt().into()
            );
        }

        in_box.invoke__resetData();

    });

    
    obox.on_getExpenses({
        let weak_box_clone = weak_get_expenses.clone();
        move || {
            println!("Retrieving Expenses!");

            let in_box = weak_box_clone.upgrade().unwrap();

            let mut item_names: Vec<String> = file_read::read_item_name_entries();
            let mut item_costs: Vec<String> = file_read::read_item_cost_entries();

            let mut item_details: Vec<ItemDetail> = Vec::with_capacity(item_costs.len());

            for i in 0..item_costs.len() {
                item_details.push(
                    ItemDetail { 
                        name: mem::take(&mut item_names[i]).into(), 
                        cost: mem::take(&mut item_costs[i]).into() 
                    }
                );
            }

            let model_rc_version = Rc::new(VecModel::from(item_details));

            in_box.invoke_updateExpenses(ModelRc::from(model_rc_version));
        }
    });


    obox.run().unwrap();

    println!("Hello, world!");
}

