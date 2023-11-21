// find sum of all multiple of 3 or 5 below 1000

fn main() {
    fn sumNaturalNumbers() -> i32 {
        let mut sum = 0;
        let mut n = 0;

        while n < 1000 {
            if n % 3 == 0 || n % 5 == 0 {
                sum += n;
            }
            n += 1; 
        }
        return sum;
    }
    let sum = sumNaturalNumbers();
    println!("Sum is {}", sum);
}
