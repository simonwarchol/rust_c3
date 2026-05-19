use ndarray::Array2;
use rust_c3::C3;

fn main() {
    let c_3 = C3::new();
    let ce_test = c_3.color_entropy(7271);
    println!("Color Entropy Test: {}", ce_test);

    let related_terms = c_3.color_related_terms(7271, Some(10), None, None);
    println!("Related Terms Test: {:?}", related_terms);

    let cosine_test = c_3.color_cosine(2173, 7271);
    println!("Cosine Test: {}", cosine_test);

    let color_index_test = c_3.color_index([60.3, 98.2, -60.8]);
    println!("Color Index Test: {:?}", color_index_test);

    let palette = Array2::from_shape_vec(
        (3, 3),
        vec![
            60.32273214,
            98.2353325,
            -60.84232404,
            79.42618245,
            -1.22650957,
            -19.14108948,
            66.88027726,
            43.42296322,
            71.85391542,
        ],
    )
    .unwrap();

    let analyzed_palette = c_3.analyze_palette(palette.clone());
    println!("Analyzed Palette: {:?}", analyzed_palette);

    let palette_terms = c_3.get_palette_terms(palette.clone(), 10);
    println!("Analyzed Palette Terms: {:?}", palette_terms);

    let cosine_matrix = c_3.compute_color_name_distance_matrix(&analyzed_palette);
    println!("Cosine Matrix: {:?}", cosine_matrix[[2, 1]]);
    println!("Cosine Matrix: {:?}", cosine_matrix.shape());
}
