slint::slint! {
    import { ListView , CheckBox} from "std-widgets.slint";
export component MainWindow {
        width: 800px;
        height: 400px;
        Rectangle {
             
        }
     
    }
}
fn main() {
    MainWindow::new().unwrap().run().unwrap();
}
