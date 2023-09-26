pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
}
impl LinearRegression{
    pub fn new() -> LinearRegression {
        LinearRegression {
            slope: 0.0,
            intercept: 0.0,
        }
    }
    pub fn fit (&mut self,input:Vec<f64>,output:Vec<f64>) {
        if input.len()!=output.len(){
            panic!("The number of input and output values is different");
        }
        let size_of_input=input.len();
        let sum_of_input:f64=input.iter().sum::<f64>();
        let sum_of_output:f64=output.iter().sum::<f64>();
        let sum_of_input_output=input.iter().zip(output.iter()).map(|(&x, &y)| x * y).sum::<f64>();
        let square_sum_input=input.iter().map(|&x|x*x).sum::<f64>();
        println!("sum_of_inputs: {}",sum_of_input);
        println!("sum_of_outputs: {}",sum_of_output);
        println!("sum_of_input_output: {}",sum_of_input_output);
        println!("square_sum_inputs: {}",square_sum_input);
        //let square_sum_output=output.iter().map(|&x|x*x).sum::<f64>();
        self.slope=(size_of_input as f64*sum_of_input_output-sum_of_input*sum_of_output)/(size_of_input as f64*square_sum_input-sum_of_input*sum_of_input);
        self.intercept=(sum_of_output*square_sum_input-sum_of_input*sum_of_input_output)/(size_of_input as f64*square_sum_input-sum_of_input*sum_of_input);
    }
}