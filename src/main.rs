use serde::Deserialize;
use std::error::Error;
use csv::ReaderBuilder;
use plotters::prelude::*;


//Struct: Wine, the wine is seperated into 3 different types(sort), and 13 characteristic
#[derive(Debug, Deserialize, Clone)]
struct Wine {
    sort: f32,
    alcohol: f32,
    malic: f32,
    ash: f32,
    alc: f32,  
    mag: f32,
    totphe: f32,
    flaphe: f32,
    xflaphe: f32,
    pro: f32,
    col: f32,
    hue: f32,
    od: f32,
    prl: f32,
}

//average function, takes in a wine struct and a feature name， return the average value of that feature's value of that type of wine
fn mean_feature<F>(wines: &[Wine], feature_extractor: F) -> f32
where
    F: Fn(&Wine) -> f32,
{
    let sum: f32 = wines.iter().map(feature_extractor).sum();
    sum / wines.len() as f32
}


//std function, takes in a wie struct and a feature name, return the standard deviation of that feature's value of that type of wine
fn std_feature<F>(wines: &[Wine], feature_extractor: F) -> f32
where
    F: Fn(&Wine) -> f32,
{
    let mean = mean_feature(wines, &feature_extractor);  // reuse the mean function
    let sum_squared_diffs: f32 = wines.iter()
        .map(feature_extractor)
        .map(|value| (value - mean).powi(2))
        .sum();

    let n = wines.len() as f32;
    (sum_squared_diffs / (n - 1.0)).sqrt()
}



//start running
fn main() -> Result<(), Box<dyn Error>> {
    //read from given csv
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path("wine.csv")?;

    //create a vector out of different wines
    let mut wines:Vec<Wine> = Vec::new();
    
    //push each line one by one from rdr
    for result in rdr.deserialize() {
        let person: Wine = result?;
        wines.push(person);
    }

    //check the data is at vec
    // println!("Vector optained {:#?}", wines);

    //start grouping based on type of wine(sort)
    let mut wines_1: Vec<Wine> = Vec::new();
    let mut wines_2: Vec<Wine> = Vec::new();
    let mut wines_3: Vec<Wine> = Vec::new();


    //looping over the new vector of wine and push it into vectors accordingly
    for wine in &wines {
        match wine.sort as f32 {
            1.0 => wines_1.push(wine.clone()),
            2.0 => wines_2.push(wine.clone()),
            3.0 => wines_3.push(wine.clone()),
            _ => println!("Warning: unknown sort value {}", wine.sort),
        }
    }

    //check the number of lines for each
    // println!("Class 1 count: {}", wines_1.len());//59
    // println!("Class 2 count: {}", wines_2.len());//71
    // println!("Class 3 count: {}", wines_3.len());//48

    //record scores for graphing
    let mut feature_scores: Vec<(&str, f32)> = Vec::new();


    //looping through all to find average

    //feature list for function to function well
    let features: Vec<(&str, fn(&Wine) -> f32)> = vec![
        ("alcohol", |w: &Wine| w.alcohol),
        ("malicacid", |w: &Wine| w.malic),
        ("ash", |w: &Wine| w.ash),
        ("alccalinity_of_ash", |w: &Wine| w.alc),
        ("magnesium", |w: &Wine| w.mag),
        ("total_phenol", |w: &Wine| w.totphe),
        ("flavanoids", |w: &Wine| w.flaphe),
        ("nonflavanoid_phenols", |w: &Wine| w.xflaphe),
        ("proanthocyanins", |w: &Wine| w.pro),
        ("color_intensity", |w: &Wine| w.col),
        ("hue", |w: &Wine| w.hue),
        ("0D280_0D315_of_diluted_wines", |w: &Wine| w.od),
        ("proline", |w: &Wine| w.prl),
    ];

    //begin looping through it manually
    for (target, fture) in &features {

        //find the means
        let mean_class1 = mean_feature(&wines_1, *fture);
        let mean_class2 = mean_feature(&wines_2, *fture);
        let mean_class3 = mean_feature(&wines_3, *fture);
        
        //print mean
        println!("________ {} average ________", target);
        println!("  Class 1: {:.3}", mean_class1);
        println!("  Class 2: {:.3}", mean_class2);
        println!("  Class 3: {:.3}", mean_class3);

        //find std
        let std_class1 = std_feature(&wines_1, *fture);
        let std_class2 = std_feature(&wines_2, *fture);
        let std_class3 = std_feature(&wines_3, *fture);


        //print std
        println!("________ {} standard deviation ________", target);
        println!("  Class 1: {:.3}", std_class1);
        println!("  Class 2: {:.3}", std_class2);
        println!("  Class 3: {:.3}", std_class3);

        //use (max mean - min mean)/mean std to find the importance constant, higher the constant, higher the function for that to distinguish the type of wine
        let mean_diff = (mean_class1.max(mean_class2).max(mean_class3)) - (mean_class1.min(mean_class2).min(mean_class3));
        let mean_std = (std_class1 + std_class2 + std_class3) / 3.0;
        let importance_score = mean_diff / mean_std;

        //push the importance score into the vector for graphing
        feature_scores.push((target, importance_score));
        //print the scores accordingly
        println!("{} Score: {}", target ,importance_score);
        
        
    }
    //print the scores totally for clearance
    println!("{:#?}",feature_scores);

    //plotting 
    let features: Vec<&str> = feature_scores.iter().map(|(name, _)| *name).collect();
    let scores: Vec<f64> = feature_scores.iter().map(|(_, score)| *score as f64).collect();
    let root = BitMapBackend::new("feature_importance.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    //upper limit for sizing the graph
    let max_score = scores.iter().cloned().fold(f64::NAN, f64::max);

    // Draw graph
    let mut chart = ChartBuilder::on(&root)
        .caption("Feature Importance", ("Arial", 25).into_font())
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0..features.len(), 0f64..(max_score * 1.1))?;


    //label the grid line and axis
    chart
        .configure_mesh()
        .x_labels(features.len())
        .x_label_formatter(&|idx| features.get(*idx).unwrap_or(&"").to_string())
        .x_desc("Features")
        .y_desc("Importance Score")
        .axis_desc_style(("Arial", 15))
        .draw()?;

    //design the bar color
    chart.draw_series(
        scores.iter().enumerate().map(|(idx, &score)| {
            Rectangle::new(
                [(idx, 0.0), (idx + 1, score)],
                RED.filled(),
            )
        }),
    )?;


    Ok(())
}
