// Referemces amd Borrowing
// Safety and Performace
// Borrowing and references are powerful concepts

// Understandign References
// References enable you to borrow values without taking ownership
// Immutable references
// Mutable references

// Crete Reference by add "&"

/* fn main() {
    let mut _x: i32 = 5;
    //let r = x; <- this completely destroys x
    let _r: &mut i32 = &mut _x;
    *_r += 1;
    *_r -= 3;

    println!("Value of _x : {}", _x);
    println!("Value of _r : {}", _r);
} */

fn main(){
    let mut account: BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money

    account.withdraw(45.5);
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}