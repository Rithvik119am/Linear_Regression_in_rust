# Linear_Regresstion_in_rust
This a Linear Regression implementation from scratch in rust
I have used the following formula:
![image](https://github.com/Rithvik119am/Linear_Regresstion_in_rust/assets/120341869/272d63c6-596d-4644-b982-42655d11a79b)


There is a example code on how to use it:
```mod linear_regression;
use linear_regression::LinearRegression;
fn main() {
    let mut a= LinearRegression::new();
    let fahrenheit=vec![1.,2.,3.,4.,5.,6.,7.,8.,9.];
    let celsius=vec![-17.22,-16.67,-16.11,-15.56,-15.0,-14.44,-13.89,-13.33,-12.78];
    a.fit(fahrenheit,celsius);
    println!("The slope is {}",a.slope);
    println!("The intercept is {}",a.intercept);
    println!("Celsius at Fahrenheit{}",a.slope*9.+a.intercept);
}
```
----------------------------------------------------------------
Any Feedback is appreciated
