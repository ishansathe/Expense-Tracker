
slint::slint!{
    import { Button, LineEdit } from "std-widgets.slint";
    import { TopCredits, BottomCredits } from "src/slint_files/menu.slint";
    import { CustomButton } from "src/slint_files/custom_button.slint";
    import { MarkPage } from "src/slint_files/Mark_Expenses/layout_text_button.slint";

    export component Box inherits Window {
        width: 600px;
        height: 400px;
        background: #e6f5e6;
        in-out property <string> amt;
        in-out property <string> name;
        callback clicked <=> markpage.submit;


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
                        markpage.visible = true;
                        view.active = false;
                    }
                }
            }

            view := CustomButton {
                x:0;
                y:100px;
                text: "View Expenses";
                TouchArea {
                    clicked => {
                        view.active = true;
                        markpage.visible = false;
                        mark.active = false;
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

    }
}

mod file_mgmt;

fn main() {

    // Box::new().unwrap().run().unwrap();

    let obox = Box::new().unwrap();
    let weak_box = obox.as_weak();
    obox.on_clicked(move || {
        println!("Button clicked!");

        let in_box = weak_box.upgrade().unwrap();

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
    obox.run().unwrap();

    println!("Hello, world!");
}

