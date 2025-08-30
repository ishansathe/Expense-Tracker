slint::slint!{

    import { Button, LineEdit } from "std-widgets.slint";
    export component Box inherits Window {
        width: 400px;
        height: 300px;
        background: #f0f0f0;

        in property <string> amt : amt.text;

        callback clicked <=> submitBtn.clicked;
        
        /* 
        callback returnText() -> string;
        returnText() => {
            return amt.text;
        }
        */


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
                        accepted(text) => {
                            name.text = "No";
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
                        text: "asd";
                    }

                    
                }
                
            }

            // submitBstn:= Button { 
            //     text: "Click me"; 
            //     width: 100px;
            //     height: 40px;
            //     // x: 150px;
            //     clicked => {
            //         amt.text = "";
            //         // root.returnText();
            //     }
            // }

            spacing: 10px;
            submitBtn:= Button { 
                text: "Click me"; 
                width: 100px;
                height: 40px;
                x: 150px;
                clicked => {
                    // amt.text = "";
                    // root.returnText();
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
