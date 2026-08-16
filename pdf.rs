use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use crate::data::Workout;

pub fn generate_pdf(workouts: &[Workout], filename: &str) -> Result<(), String> {
    // 1. Aqui desestruturamos a tupla corretamente
    let (doc, page1_idx, layer1_idx) = PdfDocument::new(
        "Workout Plan",
        Mm(210.0),
        Mm(297.0),
        "Layer 1",
    );
    
    // 2. Extraímos a referência da camada usando o doc e os índices gerados
    let mut current_layer = doc.get_page(page1_idx).get_layer(layer1_idx);

    // Carregamos as fontes (agora o doc é do tipo correto, não uma tupla)
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to load font: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to load bold font: {}", e))?;

    let mut y_pos = 270.0;

    // Título principal
    current_layer.use_text(
        "WORKOUT PLAN",
        24.0,
        Mm(30.0),
        Mm(y_pos),
        &font_bold,
    );
    y_pos -= 20.0;

    // Linha separadora
    current_layer.use_text(
        "─────────────────────────────────────",
        12.0,
        Mm(30.0),
        Mm(y_pos),
        &font,
    );
    y_pos -= 15.0;

    // Para cada treino gerado
    for workout in workouts {
        let header = format!("-- {} --", workout.workout_type);
        current_layer.use_text(
            &header,
            18.0,
            Mm(30.0),
            Mm(y_pos),
            &font_bold,
        );
        y_pos -= 15.0;

        if workout.exercises.is_empty() {
            current_layer.use_text(
                "  (No exercises)",
                12.0,
                Mm(35.0),
                Mm(y_pos),
                &font,
            );
            y_pos -= 10.0;
        } else {
            for ex in &workout.exercises {
                // Exibe os dados do exercício
                let line = format!(
                    "  {} | {:?} | {}x{} | {}kg",
                    ex.name, ex.muscular_group, ex.series, ex.reps, ex.weight_kg
                );
                current_layer.use_text(
                    &line,
                    12.0,
                    Mm(35.0),
                    Mm(y_pos),
                    &font,
                );
                y_pos -= 10.0;

                // Se estiver muito perto do fim da página, criamos outra
                if y_pos < 20.0 {
                    let (new_page_idx, new_layer_idx) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                    current_layer = doc.get_page(new_page_idx).get_layer(new_layer_idx);
                    
                    y_pos = 270.0; 
                }
            }
        }
        y_pos -= 5.0;
    }

    // Rodapé com data 
    y_pos = 20.0;
    let now = chrono::Local::now();
    let date_str = now.format("%d/%m/%Y %H:%M").to_string();
    current_layer.use_text(
        &format!("Generated on: {}", date_str),
        10.0,
        Mm(30.0),
        Mm(y_pos),
        &font,
    );

    // Salvar PDF no arquivo
    let file = File::create(filename)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    let mut writer = BufWriter::new(file);
    
    doc.save(&mut writer)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;

    Ok(())
}