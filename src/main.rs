fn main() {

    let mut seven = " ";
    let mut eight = " ";
    let mut nine = " ";
    let mut four = " ";
    let mut five = " ";
    let mut six = " ";
    let mut one = " ";
    let mut two = " ";
    let mut tree = " ";

    let mut now_x = true;
    let mut game = true;

    
        println!("                 ");
        println!(" {}{}{} | {}{}{} | {}{}{} ",seven,seven,seven,eight,eight,eight,nine,nine,nine);
        println!(" {}{}{} | {}{}{} | {}{}{} ",seven,seven,seven,eight,eight,eight,nine,nine,nine);
        println!("–––––|–––––|–––––");
        println!(" {}{}{} | {}{}{} | {}{}{} ",four,four,four,five,five,five,six,six,six);
        println!(" {}{}{} | {}{}{} | {}{}{} ",four,four,four,five,five,five,six,six,six);
        println!("–––––|–––––|–––––");
        println!(" {}{}{} | {}{}{} | {}{}{} ",one,one,one,two,two,two,tree,tree,tree);
        println!(" {}{}{} | {}{}{} | {}{}{} ",one,one,one,two,two,two,tree,tree,tree);
        println!("                 ");
    
    while game {


        

        use std::io::{stdin,stdout,Write};
        let mut s=String::new();
        if now_x{println!("X turn");}
        else {println!("O turn");}
        print!("Choose field using numpad: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Cos zepsules");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        if s == "7" {
            if seven == " "{
                if now_x{
                    seven = "X";
                    now_x = false;
                }
                else{
                    seven = "O";
                    now_x = true;
                }
            } else if seven == "O" || seven == "X" {
                println!("This field is already occupied – please try again");
            } 
        } else if s == "8" {
            if eight == " "{
                if now_x{
                    eight = "X";
                    now_x = false;
                }
                else{
                    eight = "O";
                    now_x = true;
                }
            } else if eight == "O" || eight == "X" {
                println!("This field is already occupied – please try again");
            }
        } else if s == "9" {
            if nine == " "{
                if now_x{
                    nine = "X";
                    now_x = false;
                }
                else{
                    nine = "O";
                    now_x = true;
                }
            } else if nine == "O" || nine == "X" {
                println!("This field is already occupied – please try again");
            }
        } else if s == "4" {
            if four == " "{
                if now_x{
                    four = "X";
                    now_x = false;
                }
                else{
                    four = "O";
                    now_x = true;
                }
            } else if four == "O" || four == "X" {
                println!("This field is already occupied – please try again");
            }
        } else if s == "5" {
            if five == " "{
                if now_x{
                    five = "X";
                    now_x = false;
                }
                else{
                    five = "O";
                    now_x = true;
                }
            } else if five == "O" || five == "X" {
                println!("This field is already occupied – please try again");
            }
        } else if s == "6" {
            if six == " "{
                if now_x{
                    six = "X";
                    now_x = false;
                }
                else{
                    six = "O";
                    now_x = true;
                }
            } else if six == "O" || six == "X" {
                println!("This field is already occupied – please try again");
            }
        } else if s == "1" {
            if one == " "{
                if now_x{
                    one = "X";
                    now_x = false;
                }
                else{
                    one = "O";
                    now_x = true;
                }
            } else if one == "O" || one == "X" {
                println!("This field is already occupied – please try again");
            }
        } else if s == "2" {
            if two == " "{
                if now_x{
                    two = "X";
                    now_x = false;
                }
                else{
                    two = "O";
                    now_x = true;
                }
            } else if two == "O" || two == "X" {
                println!("This field is already occupied – please try again");
            }
        } else if s == "3" {
            if tree == " "{
                if now_x{
                    tree = "X";
                    now_x = false;
                }
                else{
                    tree = "O";
                    now_x = true;
                }
            } else if tree == "O" || tree == "X" {
                println!("This field is already occupied – please try again");
            }
        }


        if 
        ((one == two) && (two == tree) && (tree == "X")) || ((four == five) && (five == six) && (six == "X")) || ((seven == eight) && (eight == nine) && (nine == "X")) ||
        ((one == four) && (four == seven) && (seven == "X")) || ((two == five) && (five == eight) && (eight == "X")) || ((tree == six) && (six == nine) && (nine == "X")) ||
        ((one == five) && (five == nine) && (nine == "X")) || ((tree == five) && (five == seven) && (seven == "X")) 
        {
            game = false;
            println!("X won!");
        } else if
        ((one == two) && (two == tree) && (tree == "O")) || ((four == five) && (five == six) && (six == "O")) || ((seven == eight) && (eight == nine) && (nine == "O")) ||
        ((one == four) && (four == seven) && (seven == "O")) || ((two == five) && (five == eight) && (eight == "O")) || ((tree == six) && (six == nine) && (nine == "O")) ||
        ((one == five) && (five == nine) && (nine == "O")) || ((tree == five) && (five == seven) && (seven == "O")) 
        {
            game = false;
            println!("O won!");
        } {
            
        }

            println!("                 ");
            println!(" {}{}{} | {}{}{} | {}{}{} ",seven,seven,seven,eight,eight,eight,nine,nine,nine);
            println!(" {}{}{} | {}{}{} | {}{}{} ",seven,seven,seven,eight,eight,eight,nine,nine,nine);
            println!("–––––|–––––|–––––");
            println!(" {}{}{} | {}{}{} | {}{}{} ",four,four,four,five,five,five,six,six,six);
            println!(" {}{}{} | {}{}{} | {}{}{} ",four,four,four,five,five,five,six,six,six);
            println!("–––––|–––––|–––––");
            println!(" {}{}{} | {}{}{} | {}{}{} ",one,one,one,two,two,two,tree,tree,tree);
            println!(" {}{}{} | {}{}{} | {}{}{} ",one,one,one,two,two,two,tree,tree,tree);
            println!("                 ");
    }
    
    
}