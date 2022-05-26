fn main() {
    println!("{:?}", is_prime(8));
    println!("{:?}", is_prime(7));
}

fn is_prime(x: i64) -> bool {
    if x == 2 || x == 3 {
        return true;
    }
    
    if x <= 1 || x % 2 == 0 || x % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    let mut result = true;
    
    while i * i <= x {
        if(x % i == 0) || (x % (i + 2) == 0) {
            result = false;
            break;
        }
            
        i += 6;
    }
    
    result
}
