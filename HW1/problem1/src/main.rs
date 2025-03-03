const FREEZE_POINT_F: f64 = 32.0;

//conversion fahrenheit to celsius
fn tempF_to_tempC(f:f64) -> f64{
    (f - FREEZE_POINT_F) * 5.0 / 9.0
}

//conversion celsius to fahrenheit
fn tempC_to_tempF(c: f64) -> f64{
    (c * 9.0 / 5.0) + FREEZE_POINT_F
}

fn main(){
    let mut temp_f: f64 = 32.0;

    let temp_c = tempF_to_tempC(temp_f);
    println!("{:.1}F us {:.1}C",temp_f,temp_c);

    for i in 1..=5{
        let next_temp_f = temp_f + i as f64;
        let next_temp_c = tempF_to_tempC(next_temp_f);
        println!("{:.1}F is {:.1}C",next_temp_f,next_temp_c)
    }
}