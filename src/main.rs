extern crate csv_import_general;
extern crate gnuplot;

use csv_import_general::csv_parse;
use gnuplot::*;

#[derive(Debug)]
pub struct AXIS_RANGE{
    pub x_max :f64,
    pub x_min :f64,
    pub y_max :f64,
    pub y_min :f64,
}


enum MEMBER_STATUS_ENUM {
    height,
    weight
}

#[derive(Debug)]
struct MEMBER_STATUS{
    height : f64,
    weight :f64
}

fn float_sum(nvec : &Vec<MEMBER_STATUS>) -> MEMBER_STATUS{ 
    let mut buff = MEMBER_STATUS{height : 0.0, weight : 0.0};
    for item in nvec{
        buff.height += item.height;
        buff.weight += item.weight;
    }

    buff.height = buff.height/nvec.len() as f64;
    buff.weight = buff.weight/nvec.len() as f64;

    buff
}


fn plot_points(datas : &Vec<MEMBER_STATUS>,fg : &mut Figure , color : String,caption : String, axis_info : &AXIS_RANGE){
    let mut x : Vec<f64> = Vec::new();
    let mut y : Vec<f64> = Vec::new();
    for item in datas {
        x.push(item.height);
        y.push(item.weight);
    }

    fg.axes2d()
        .set_x_range(Fix(axis_info.x_min), Fix(axis_info.x_max))
        .set_y_range(Fix(axis_info.y_min),Fix(axis_info.y_max))
        .points(&x, &y, &[Color(&color)])
        .set_x_axis(true, &[]);

}

fn plot_line(a : f64 , b : f64,fg : &mut Figure , color : String,caption : String, axis_info : &AXIS_RANGE){
    let mut line_x : Vec<f64> = Vec::new();
    let mut line_y : Vec<f64> = Vec::new();
    for n in 0..200{
        line_x.push(n as f64);
        line_y.push(&a*(n as f64)+&b);
    }
    fg.axes2d()
        .set_x_range(Fix(axis_info.x_min), Fix(axis_info.x_max))
        .set_y_range(Fix(axis_info.y_min),Fix(axis_info.y_max))
        .lines(&line_x, &line_y, &[Color(&color)])
        .set_x_axis(true, &[]);
}

fn main(){

    //data import

    let mut datas = csv_parse::read_csv_data("dataset.csv".to_string()).unwrap();
    let mut members : Vec<MEMBER_STATUS> = Vec::new();

    for item in &datas{
        let buff = MEMBER_STATUS{ height : item.get(MEMBER_STATUS_ENUM::height as usize).unwrap().parse::<f64>().unwrap() , 
            weight : item.get(MEMBER_STATUS_ENUM::weight as usize).unwrap().parse::<f64>().unwrap()};
        members.push(buff);
    }

    //ave
    let average = float_sum(&members);
    println!("average = {:?}", float_sum(&members));
    
    //S xy
    let mut Sxy = 0.0;
    let mut Sx = 0.0;
    let mut Sy = 0.0;
    for item in &members{
        Sxy += (item.height - average.height)*(item.weight - average.weight);
        Sx += (item.height - average.height).powf(2.0);
        Sy += (item.weight - average.weight).powf(2.0);
    }
    let r = Sxy/(Sx.sqrt()*Sy.sqrt());
    // y=ax+b
    let a = Sxy/Sx.sqrt().powf(2.0);
    let b = average.weight - a*average.height;

    println!("Correlation coefficient = {:?}",r);
    println!("y = {:?}*x + {:?}",a,b);

    let mut plot_data = Figure::new();
    let range = &AXIS_RANGE { x_min : 150.0 ,x_max : 200.0 ,y_min : 60.0 ,y_max : 100.0};
    plot_points(&members,&mut plot_data,"red".to_string(),"points".to_string(),range);
    plot_line(a, b,&mut plot_data ,"green".to_string(),"line".to_string(), range);
    plot_data.set_title(&format!("y = {:?}*x + ({:?})",a,b));
    plot_data.show();
}