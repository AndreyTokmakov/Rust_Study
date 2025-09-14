#![allow(
dead_code,
unused_imports,
unused_parens,
unused_variables,
non_snake_case
)]


#[path = "Attributes/attributes.rs"] pub mod attributes;
#[path = "Bytes_Bits/main.rs"] pub mod bytes_bits;
#[path = "Environment/environment.rs"] pub mod environment;
#[path = "Autotests/autotests.rs"] pub mod autotests;
#[path = "Cryptography/main.rs"] pub mod cryptography;
#[path = "Functions/functions.rs"] pub mod functions;
#[path = "Command_Line_Arguments/main_command_line.rs"] pub mod command_line;
#[path = "Input_Arguments/read_from_input.rs"] pub mod read_from_input;
#[path = "Compare/compare.rs"] pub mod compare;
#[path = "Time_Duration/tests.rs"] pub mod time_and_duration;
#[path = "Dispatching/dynamic_dispatch.rs"] pub mod dynamic_dispatch;
#[path = "Loops/loops.rs"] pub mod loops;
#[path = "Ownership/ownership.rs"] pub mod ownership;
#[path = "Conditions/conditions.rs"] pub mod conditions;
#[path = "Lambdas/lambdas.rs"] pub mod lambdas;
#[path = "Types/types_tests.rs"] pub mod types;
#[path = "Memory/memory.rs"] pub mod memory;
#[path = "OOP_Structures/structs.rs"] pub mod structs;
#[path = "OOP_Traits/traits.rs"] pub mod traits;
#[path = "Enums/enums.rs"] pub mod enums;
#[path = "Optional/optional.rs"] pub mod optional;
#[path = "Random/random.rs"] pub mod random;
#[path = "Strings/strings.rs"] pub mod strings;
#[path = "Slices/slices.rs"] pub mod slices;
#[path = "SmartPointers/main.rs"] pub mod smart_pointers;
#[path = "ErrorsHandling/main.rs"] pub mod errors_handling;
#[path = "Files_Folders_Directories/main.rs"] pub mod files_folders_directories;
#[path = "Generics_Templates/generics.rs"] pub mod generics;
#[path = "Directories/directories.rs"] pub mod directories;
#[path = "Tuples/tuples.rs"] pub mod tuples;
#[path = "Console_IO/console_io.rs"] pub mod console_io;
#[path = "Pointers_and_References/pointers_and_reference.rs"] pub mod pointers_and_reference;
#[path = "Json/Json.rs"] pub mod Json;
#[path = "ScopingRules_LifeTIme_RAII/scoping_rules.rs"] pub mod scoping_rules;
#[path = "Operators_Overload/main.rs"] pub mod operators_overload;
#[path = "Unsafe_Features/unsafe_features.rs"] pub mod unsafe_features;
#[path = "Operators/main.rs"] pub mod operators;
#[path = "Test_Experiments/tests_and_experiments.rs"] pub mod tests_and_experiments;
#[path = "Display/main.rs"] pub mod display;
#[path = "Documentation/main.rs"] pub mod documentation;
#[path = "Pattern_Matching/main.rs"] pub mod pattern_matching;
#[path = "Lifetime_Borrowing/main.rs"] pub mod lifetime_borrowing;
#[path = "Result/result.rs"] pub mod result;


fn main()
{
    // tests_and_experiments::test_all();
    // loops::test_all();
    // lifetime_borrowing::test_all();
    
    // attributes::test_all();
    // autotests::test_all();
    // bytes_bits::test_all();
    // environment::test_all();
    // compare::test_all();
    // command_line::test_all();
    // conditions::test_all();
    // cryptography::test_all();
    // functions::test_all();
    memory::test_all();
    // read_from_input::test_all();
    // random::test_all();
    // ownership::test_all();
    // pointers_and_reference::test_all();
    // console_io::test_all();
    // errors_handling::test_all();
    // files_folders_directories::test_all();
    // folders_directories::test_all();
    // directories::test_all();
    // display::test_all();
    // enums::test_all();
    // lambdas::test_all();
    // dynamic_dispatch::test_all();
    // strings::test_all();
    // scoping_rules::test_all();
    // unsafe_features::test_all();

    // optional::test_all();
    // result::test_all();


    // pattern_matching::test_all();

    // NOTE: Json
    // Json::test_all();
    
    // NOTE: Documentation
    // documentation::test_all();

    // NOTE: Smart pointers
    // smart_pointers::test_all();

    // operators::test_all();
    // operators_overload::test_all();

    // NOTE: Time - Duration:
    // time_and_duration::test_all();
    
    // tuples::test_all();
    // types::test_all();

    // NOTE: Generics - Templates
    // generics::test_all();

    // structs::test_all();
    // traits::test_all();
    
}
