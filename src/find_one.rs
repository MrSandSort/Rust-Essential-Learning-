fn main()
{
    let numbers= [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];

    let mut max: i32;
    let mut min: i32;
    let mut avg: f64;

    max= find_max(& numbers);
    min= find_min(& numbers);
    avg= find_avg(& numbers);

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(avg, 12.5);

    println!("Test Passed!");


}

fn find_max(number: &[i32]) -> i32
{
    let mut max: i32= number[0];

    for item in number
    {
        if max < *item
        {
            max= *item;
        }   
    }

    return max;
}

fn find_min(num: & [i32]) -> i32 
{
    let mut min: i32= num[0];

    for item in num
    {
       if *item < min
       {
          min= *item;
       }
    }

    return min;
}

fn find_sum(num: &[i32])-> i32
{
    let mut sum: i32= 0;

    for item in num
    {
        sum += *item;
    }

    return sum;

}

fn find_avg(num: &[i32]) -> f64 
{
    let mut avg: f64;
    let mut sum: i32= find_sum(num); 
    let mut _length: usize= num.len();

    avg= sum as f64 / _length as f64;

    return avg;

}