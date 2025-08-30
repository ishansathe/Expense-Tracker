
slint::slint!{
    import { Button, LineEdit } from "std-widgets.slint";
    export component Box inherits Window {
        width: 400px;
        height: 300px;
        background: #f0f0f0;
        in-out property <string> amt;
        in-out property <string> name;
        callback clicked <=> submitBtn.clicked;

        public function _resetData() -> bool{
            amt.text = "";
            name.text = "";
            return true;
        }
        VerticalLayout { 
            Rectangle {
                width: parent.width;
                height: 20px;
                background: #ccc;
            }

            Text{
                text: "Hello";
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

            Rectangle {
                width: parent.width;
                height: 20px;
                background: #cccfcc;
            }
        }
    }
}


fn main() {

    // Box::new().unwrap().run().unwrap();

    let obox = Box::new().unwrap();
    let weak_box = obox.as_weak();
    obox.on_clicked(move || {
        println!("Button clicked!");

        let in_box = weak_box.upgrade().unwrap();
        println!("Name: {}, Amount: {}", in_box.get_name(), in_box.get_amt());
        in_box.invoke__resetData();

    });
    obox.run().unwrap();

    println!("Hello, world!");
}
