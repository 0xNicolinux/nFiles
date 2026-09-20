use anyhow::Result;
use clap::Parser;
use file_organizer::categories::Category;
use file_organizer::cli::{CliArgs, CliUserInterface};
use file_organizer::filesystem::FilesystemHandler;
use file_organizer::organizer::{Organizer, OrganizerOptions};

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let target_path_str = if let Some(ref p) = args.path {
        p.to_string_lossy().to_string()
    } else {
        CliUserInterface::prompt_directory()?
    };

    if target_path_str.is_empty() {
        println!("No path provided. Exiting.");
        return Ok(());
    }

    let resolved_path = match FilesystemHandler::resolve_path(&target_path_str) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error resolving directory path '{}': {}", target_path_str, err);
            std::process::exit(1);
        }
    };

    let options = OrganizerOptions {
        root_dir: resolved_path.clone(),
        recursive: args.recursive,
        dry_run: args.dry_run,
        verbose: args.verbose,
    };

    let organizer = Organizer::default();
    let operations = organizer.scan(&options)?;

    if operations.is_empty() {
        println!("No files found to organize in {}.", resolved_path.display());
        return Ok(());
    }

    // In interactive mode (or when no path argument was passed), prompt user before proceeding
    let is_interactive = args.interactive || args.path.is_none();
    if is_interactive && !args.dry_run {
        let summary = organizer.build_summary(&operations);
        println!("\nFiles found: {}\n", summary.total_found);

        for category in Category::all_categories() {
            if let Some(count) = summary.category_counts.get(&category) {
                if *count > 0 {
                    println!("{:<14} {:>5}", format!("{}:", category), count);
                }
            }
        }
        println!();

        let proceed = CliUserInterface::prompt_confirm("Proceed?", false)?;
        if !proceed {
            println!("Operation cancelled by user.");
            return Ok(());
        }
    }

    let summary = organizer.execute(&options, &operations);

    println!("\nOrganization completed.\n");
    if args.dry_run {
        println!("Files found: {}", summary.total_found);
    } else {
        println!("{:<12} {:>5}", "Moved:", summary.moved);
        println!("{:<12} {:>5}", "Skipped:", summary.skipped);
        println!("{:<12} {:>5}", "Failed:", summary.failed);
        println!("{:<12} {:>5}", "Unrecognized:", summary.unrecognized);
    }

    Ok(())
}
