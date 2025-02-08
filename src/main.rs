use crossterm::{
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    QueueableCommand,
};
use rustic_print::RusticPrint;
use std::io::{stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rustic_print = RusticPrint::new();

    // 1. info: A simple informational message.
    // rustic_print.info("This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    //     rustic_print.info_multiple(vec!["This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "potato"]);

    rustic_print.success("This is an success message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.");

    //rustic_print.success_multiple(vec!["This is a success message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "This is an success message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit."]);

    rustic_print.caution("This is a caution message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    rustic_print.caution(vec!["This is a success message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "This is an success message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit."]);

    rustic_print.error("This is a error message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    rustic_print.error(vec!["This is a error message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "This is an success message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit."]);

    // rustic_print.comment("This is a comment message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    // rustic_print.comment(vec!["This is a comment message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "This is an success message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit."]);

    rustic_print.warning("This is a comment message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    rustic_print.warning(vec!["This is a comment message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "This is an success message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit. This is an info message. Lorem ipsum dolor sit amet, consectetur adipiscing elit."]);

    // 2. block: Uses default block options.
    // rustic_print.block("This is a block message. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.");
    //
    // // 3. title: Displays a title with an underline.
    // rustic_print.title("Title Example");
    //
    // // 4. section: Displays a section header with a dashed underline.
    // rustic_print.section("Section Example: Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    //
    // // 5. listing: Shows a bullet list.
    // rustic_print.listing(vec!["Item 1", "Item 2", "Item 3", "Item 4"]);
    //
    // // 6. text: Prints plain text with extra padding lines.
    // rustic_print.text("This is a plain text message with extra padding.");
    //
    // // 7. comment: Prints a comment-styled block.
    // rustic_print.comment("This is a comment message.");
    //
    // // 8. success: Prints a success-styled block.
    // rustic_print.success("Operation completed successfully!");
    //
    // // 9. error: Prints an error-styled block.
    // rustic_print.error("An error has occurred.");
    //
    // // 10. warning: Prints a warning-styled block.
    // rustic_print.warning("This is a warning message.");
    //
    // // 11. note: Prints a note-styled block.
    // rustic_print.note("This is a note message.");
    //
    // // 12. caution: Prints a caution-styled block.
    // rustic_print.caution("This is a caution message.");
    //
    // // 13. table: Displays a table with headers and multiple rows.
    // rustic_print.table(
    //     vec!["Header 1", "Header 2", "Header 3"],
    //     vec![
    //         vec!["Cell 1", "Cell 2", "Cell 3"],
    //         vec!["Data 1", "Data 2", "Data 3"],
    //         vec!["Value 1", "Value 2", "Value 3"],
    //     ],
    // );
    //
    // // 14. confirm: Prompts the user for a yes/no answer.
    // let user_confirm = rustic_print.confirm("Do you want to continue?", true);
    // println!("Confirm result: {}", user_confirm);
    //
    // // 15. ask (without validation): Asks a question with a default answer.
    // let name = rustic_print.ask("What is your name?", Some("John Doe"), None);
    // println!("Your name is: {}", name);
    //
    // // 16. ask (with validation): Asks for an integer value.
    // let number = rustic_print.ask(
    //     "Enter a valid integer:",
    //     Some("42"),
    //     Some(Box::new(|input| {
    //         input.parse::<i32>()
    //             .map_err(|_| "Please enter a valid number.".to_string())?;
    //         Ok(())
    //     })),
    // );
    // println!("Validated integer: {}", number);
    //
    // // 17. fancy_block: Direct call with custom block options.
    // let custom_options = BlockOptions {
    //     prefix: Some(">>".to_string()),
    //     name: Some("FANCY".to_string()),
    //     line_width: 60,
    //     padding: true,
    //     style: Some(StyleOptions {
    //         foreground: Some(Color::Blue),
    //         background: Some(Color::White),
    //     }),
    //     ..Default::default()  // fills in the missing field `escape`
    // };
    // rustic_print.fancy_block("This is a fancy block message with custom options.", custom_options);

    Ok(())
}
