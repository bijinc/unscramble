use std::path::Path;
use std::fs;
use rand::Rng;

pub fn populate_dir(path: &Path) {
    println!("Populating dir: {}", path.display());

    let mut rng = rand::thread_rng();
    
    // meeting notes - high similarity
    for i in 1..=5 {
        let filename = format!("meeting_notes_{}.txt", i);
        fs::write(path.join(&filename), format!("Meeting notes {}", i)).unwrap();
    }
    
    // reports - high similarity
    for i in 1..=4 {
        let filename = format!("project_report_{}.md", i);
        fs::write(path.join(&filename), format!("Project report {}", i)).unwrap();
    }
    
    // invoices - high similarity
    for i in 1..=3 {
        let filename = format!("invoice_document_{}.pdf", i);
        fs::write(path.join(&filename), format!("Invoice {}", i)).unwrap();
    }
    
    // photos - high similarity
    for i in 1..=4 {
        let filename = format!("team_photo_{}.jpg", i);
        fs::write(path.join(&filename), format!("Photo {}", i)).unwrap();
    }

    // budgets - medium similarity (different years)
    let years = ["2023", "2024", "2025"];
    for year in years {
        let filename = format!("budget_spreadsheet_{}.xlsx", year);
        fs::write(path.join(&filename), format!("Budget {}", year)).unwrap();
    }
    
    // client presentations - medium similarity
    let clients = ["acme", "globex", "initech"];
    for client in clients {
        let filename = format!("client_presentation_{}.pptx", client);
        fs::write(path.join(&filename), format!("Presentation for {}", client)).unwrap();
    }

    // class presentations - medium similarity
    let classes = ["math", "cs", "stats"];
    for class in classes {
        let filename = format!("class_presentation_{}.pptx", class);
        fs::write(path.join(&filename), format!("Presentation for {}", class)).unwrap();
    }
    
    // Edge cases: Files with partial similarity (should test threshold)
    fs::write(path.join("meeting_agenda.txt"), "Agenda").unwrap();
    fs::write(path.join("notes_summary.txt"), "Summary").unwrap();
    fs::write(path.join("project_overview.md"), "Overview").unwrap();
    fs::write(path.join("team_roster.csv"), "Roster").unwrap();
    
    // Files with numbers that should be ignored in feature extraction
    fs::write(path.join("report_2024_final_v2.doc"), "Final report").unwrap();
    fs::write(path.join("report_2023_draft_v1.doc"), "Draft report").unwrap();
    
    // Files with different delimiters
    fs::write(path.join("data-analysis-jan.csv"), "January data").unwrap();
    fs::write(path.join("data-analysis-feb.csv"), "February data").unwrap();
    fs::write(path.join("data_backup_jan.zip"), "January backup").unwrap();
    fs::write(path.join("data_backup_feb.zip"), "February backup").unwrap();
    
    // Singleton files (should not be grouped)
    fs::write(path.join("readme.txt"), "Readme content").unwrap();
    fs::write(path.join("config.json"), "Config content").unwrap();
    fs::write(path.join("license.md"), "License content").unwrap();
    
    // Files with no clear features (edge case)
    fs::write(path.join("123.txt"), "Numbers only").unwrap();
    fs::write(path.join("456.log"), "More numbers").unwrap();

    // Create subdirectories for recursive testing
    fs::create_dir_all(path.join("subdir1")).unwrap();
    fs::create_dir_all(path.join("subdir2")).unwrap();
    fs::write(path.join("subdir1/nested_file_1.txt"), "Nested file 1").unwrap();
    fs::write(path.join("subdir2/nested_file_2.txt"), "Nested file 2").unwrap();
}

pub fn clear_dir(path: &Path) {
    println!("Clearing dir: {}", path.display());

    match fs::read_dir(path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            if path.is_ok() {
                let p = path.unwrap().path();
                if p.is_dir() {
                    fs::remove_dir_all(p).unwrap();
                } else {
                    fs::remove_file(p).unwrap();
                }
            }
        },
    }
}
