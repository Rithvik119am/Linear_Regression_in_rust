mod linear_regression;
use linear_regression::LinearRegression;
fn main() {
    println!("Hello, world!");
    let mut a= LinearRegression::new();
    let fahrenheit=vec![1.,2.,3.,4.,5.,6.,7.,8.,9.];
    let celsius=vec![-17.222222,-16.6666667,-16.1111111,-15.5555556,-15.0,-14.4444444,-13.8888889,-13.3333333,-12.7777778];
    a.fit(fahrenheit,celsius);
    println!("The slope is {}",a.slope);
    println!("The intercept is {}",a.intercept);
    println!("{}",a.slope*9.+a.intercept);
}
