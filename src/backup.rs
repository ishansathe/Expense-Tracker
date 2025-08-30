slint::slint!{
    import { Button, LineEdit } from "std-widgets.slint";
    export component Box inherits Window {
        width: 400px;
        height: 300px;
        background: #f0f0f0;
        in-out property <string> amt;
        in-out property <string> name;
        callback clicked <=> submitBtn.clicked;

        public function submit() {
            name.setName();
            amt.setCost();
            txt.text = "Name: " + root.name + ", Cost: " + root.amt;
        }
        txt := Text {
            text: "Name: " + root.name + ", Cost: " + root.amt;
            font-size: 16px;
            color: #000;
            horizontal-alignment: center;
            vertical-alignment: center;
            y: 250px;
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
                        accepted(text) => {
                            name.text = "No";
                        }
                        public function setName() {
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
                        // text: root.amt;
                        public function setCost() {
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
                clicked => {
                    root.submit();
                    amt.text = "";
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
