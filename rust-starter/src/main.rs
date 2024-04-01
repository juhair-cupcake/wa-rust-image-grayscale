fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

struct BankAccount {
    balance: i32,
    verified: bool,
}

//& for pointer
fn print_balance(account: &BankAccount) {
    println!("{:?}", account.balance)
}
fn print_verified(account: &BankAccount) {
    println!("{:?}", account.verified)
}

//Reult
fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    match account.verified {
        true => Ok(true),
        false => Err(false),
    }
}

fn main() {
    //variable
    let mut num = add(30, 10);
    let mut free_shipping: bool = false;
    let mut my_text = "text";

    //condition
    if num > 50 {
        my_text = "> 50";
        free_shipping = true;
    } else if num > 20 {
        my_text = "> 20";
    } else {
        my_text = "nope"
    }
    println!("{0} {my_text}", num);

    //match
    num = match free_shipping {
        true => num + 0,
        false => num + 5,
    };
    match num {
        30 => println!("first"),
        40 => println!("second"),
        _ => println!("no match"),
    }

    //array
    let items: [i32; 5] = [1, 1, 1, 1, 1];
    println!("{:?}", items);

    //vectors
    let vector_items = vec![2, 2, 2, 2, 2];
    let mut vector_two = Vec::new();

    vector_two.push(3);
    vector_two.push(3);
    vector_two.push(3);
    vector_two.push(3);
    vector_two.push(3);
    println!("{:?}", vector_items);
    println!("{:?}", vector_two);

    //struc
    let my_account = BankAccount {
        balance: 20,
        verified: false,
    };

    //ownership
    // print_balance(my_account);
    //ownership removed
    // print_verified(my_account)
    //error

    //borrow
    print_balance(&my_account);
    print_verified(&my_account);

    //result
    let verification_status = is_verified(&my_account).expect("unable to unwrap result.");
    println!("{:?}", verification_status);
}
