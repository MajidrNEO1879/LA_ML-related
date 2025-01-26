

pub fn simple_reg()
{
    let x:[f32; 5] = [5.0, 3.0, -1., 2., 6. ];
    let y: [f32; 5] = [14.0, 6.0, -5.5, 3.5, 18.0];
    let n = x.len();
    let x_sum:f32 = x.iter().sum();
    let y_sum:f32= y.iter().sum();
    let mut xiyi:f32 = 0.;
    let mut xI_2 = 0.;
     for i in (0..x.len())
    {
        xiyi = xiyi + x[i] * y[i];
        xI_2 = xI_2 + x[i]*x[i];
    }
    
    let W_1 = (n as f32 * xiyi - x_sum * y_sum)/(n as f32 * xI_2 - x_sum * x_sum);
    // print!("{}", W);
    let W_0 = (y_sum - W_1* x_sum)/n as f32;
    print!("{}", W_0);
}