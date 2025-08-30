
slint::slint!{
    import { Button, LineEdit } from "std-widgets.slint";
    import { Buttons } from "C:\\Users\\ACER\\Strongest\\expense_tracker\\src\\ok.slint";
    export component Box inherits Window {
        width: 400px;
        height: 400px;
        background: greenyellow;
        in-out property <string> amt;
        in-out property <string> name;
        callback clicked <=> submitBtn.clicked;

        public function _resetData() -> bool{
            amt.text = "";
            name.text = "";
            return true;
        }

        public function changeInfo(newText: string) -> bool {
            info.text = newText;
            return true;
        }

        VerticalLayout { 
            Rectangle {
                width: parent.width;
                height: 20px;
                background: red;
            }
            
            Text{
                text: "Expense Tracker";
                font-family: "Bernard MT Condensed";
                font-size: 24px;
                color: #333;
                horizontal-alignment: center;
                vertical-alignment: center;
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
                        font-size: 16px;
                        color: #555;
                        horizontal-alignment: center;
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
                        font-size: 16px;
                        color: #555;
                        horizontal-alignment: center;
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
            spacing: 10px;
            submitBtn:= Button { 
                text: "Click me"; 
                width: 100px;
                height: 40px;
                x: 150px;
            }

            info:= Text {
                horizontal-alignment: center;
                font-size: 16px;
                text: "Nothing yet added!";
                font-weight: 500;
            }

            Rectangle {
                width: parent.width;
                height: 20px;
                background: red;
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

