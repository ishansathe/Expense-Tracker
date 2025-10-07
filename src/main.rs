
slint::slint!{
    import { Button, LineEdit } from "std-widgets.slint";
    import { Total } from "C:/Users/ACER/Strongest/expense_tracker/src/slint_files/view_total.slint";
    import { SideBarMenu } from "C:/Users/ACER/Strongest/expense_tracker/src/slint_files/menu.slint";
    // import {SideBarMenu} from "./slint_files/menu.slint";
    export component Box inherits Window {
        width: 600px;
        height: 400px;
        background: #e6f5e6;
        in-out property <string> amt;
        in-out property <string> name;
        callback clicked <=> submitBtn.clicked;



        public function _resetData() -> bool{
            amt.text = "";
            name.text = "";
            return true;
        }

        SideBarMenu{
            x: parent.x;
            y: parent.y;
            height: parent.height;
        }
        
        public function changeInfo(newText: string) -> bool {
            info.text = newText;
            return true;
        }

        home:= VerticalLayout { 
            x: parent.x + 100 * 1px;
            y: parent.y;

            // For some reason, this is giving parent.width - 100 as error by reading it as parent.width-100
            // So I'll change it another way.
            width: 500px;
            spacing: 10px;

            Rectangle {
                width: parent.width;
                height: 20px;
                background: #023602;
            }

            Text{
                text: "Expense Tracker";
                font-family: "Gill Sans Ultra Bold Condensed";
                font-size: 28px;
                color: #548054;
                horizontal-alignment: center;
                vertical-alignment: center;
                height: 72px;
            }
            Text {
                text: "Enter expense information below:";
                font-size: 20px;
                vertical-alignment: center;
                horizontal-alignment: center;
            }
            HorizontalLayout {
                alignment: center;
                spacing: 66px;
                VerticalLayout {
                    Text{
                        text: "Name";
                        font-size: 18px;
                        color: #436b41;
                        horizontal-alignment: center;
                        font-family: "Century";
                    }
                    name:= LineEdit {
                        placeholder-text: "Enter name";
                        width: 101px;
                        horizontal-alignment: center;

                        changed text => {
                            root.name = name.text;
                        }
                    }
                }
                VerticalLayout {
                    Text{
                        text: "Cost";
                        font-size: 18px;
                        color: #436b41;
                        horizontal-alignment: center;
                        font-family: "Century";
                    }
                    amt:= LineEdit {
                        placeholder-text: "Enter cost";
                        width: 101px;
                        horizontal-alignment: center;
                        input-type: number;

                        changed text => {
                            root.amt = amt.text;
                        }
                    }
                }
                
            }
            submitBtn:= Button { 
                text: "Click me"; 
                width: 100px;
                height: 40px;
                x: parent.x + 100px;
            }

            info:= Text {
                horizontal-alignment: center;
                font-family: "Bahnschrift Condensed";
                font-weight: 500;
                font-size: 16px;
                vertical-alignment: center;
                text: "Nothing yet added!";
                height: 48px;
                color: #0f9407;
            }

            Rectangle {
                width: parent.width;
                height: 20px;
                background: #023602;
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

