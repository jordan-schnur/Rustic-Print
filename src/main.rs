use crossterm::style::style;
use rustic_print::RusticPrint;

fn main() {
    let rustic_print = RusticPrint::new();

    rustic_print.block("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor. Ut tortor pretium viverra suspendisse. Nam at lectus urna duis convallis convallis tellus id. Mattis aliquam faucibus purus in massa tempor nec feugiat nisl. Condimentum vitae sapien pellentesque habitant. Lorem ipsum dolor sit amet consectetur. Commodo odio aenean sed adipiscing. Venenatis a condimentum vitae sapien pellentesque. Pellentesque elit ullamcorper dignissim cras tincidunt lobortis feugiat. Ut porttitor leo a diam sollicitudin tempor id eu nisl. Platea dictumst quisque sagittis purus sit amet volutpat. Egestas erat imperdiet sed euismod. Erat velit scelerisque in dictum non. Nec dui nunc mattis enim ut tellus elementum sagittis. Elementum facilisis leo vel fringilla.");

    rustic_print.title("Lorem ipsum dolor sit amet.");

    rustic_print.section("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");

    rustic_print.listing(vec![
            "Lorem",
            "Ipsum",
            "Dolor",
            "Sit",
            "Amet"
    ]);

    //     $style->text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    // //
    //     $style->comment("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    // //
    //     $style->success("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    // //
    //     $style->error("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    // //
    //     $style->warning("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    // //
    //     $style->note("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    // //
    //     $style->info("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    // //
    //     $style->caution("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    // //
    //
    // rustic_print.comment("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    //
    // rustic_print.error("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    // rustic_print.warning("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    //
    // rustic_print.info("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    //
    // rustic_print.caution("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Tellus at urna condimentum mattis pellentesque id nibh tortor.");
    //
    // let answer = rustic_print.confirm("Are you sure you want to continue?", true);
    //
    // println!("Answer: {:?}", answer);
    // // $style->table(
    // // 		            ['Header 1aggg1aggg1aggg', 'Header 2', 'Header 3'],
    // // 		            [
    // // 		                ['Cell 1', 'Cell 2', 'Cell 3'],
    // // 		                ['Cell 1', 'Cell 2Cell 2Cell 2Cell 2Cell 2', 'Cell 3'],
    // // 		                ['Cell 1', 'Cell 2', 'Cell 3'],
    // // 		            ]
    // // 		        );
}
