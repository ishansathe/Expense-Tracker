slint::slint!{
    import { Button, LineEdit, VerticalBox } from "std-widgets.slint";
    export component Box inherits Window {
        width: 400px;
        height: 300px;
        background: #f0f0f0;
        property <string> amt;
        callback clicked <=> submit.clicked;

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
                    LineEdit {
                        placeholder-text: "Enter name";
                        width: 101px;
                        horizontal-alignment: center;
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
                        
                    }
                }
                
            }
            spacing: 10px;
            submit:= Button { 
                text: "Click me"; 
                width: 100px;
                height: 40px;
                x: 150px;
                clicked => {
                    root.amt = amt.text;
                }
            }
            // padding: 20px;
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
    // let weak_box = obox.as_weak();
    obox.on_clicked(move || {
        println!("Button clicked!");
    });
    obox.run().unwrap();

    println!("Hello, world!");
}
