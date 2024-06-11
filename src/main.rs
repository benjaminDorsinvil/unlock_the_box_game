use std::io::stdin;

enum State {
    Locked, 
    Unlocked, 
    Failed
}

fn main() {
    let code = String::from("123");
    let mut entry = String::new();
    let mut state = State::Locked;

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                println!("Enter your code: ");
                match stdin().read_line(&mut input){
                    Ok(_) =>{
                        entry.push_str(&input.trim_end());
                        if entry == code {
                            state = State::Unlocked;
                        }
                        if !entry.starts_with(&code){
                            state = State::Failed;
                        }
                    },
                    Err(_) => continue

                };
            }
            State::Unlocked => {
                println!("CONGRATULATIONS!! You've unlocked the box!");
                return;

            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;

            }

            
        }
    }

}
