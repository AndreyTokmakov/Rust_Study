#![allow(
dead_code,
unused_imports,
unused_parens,
unused_variables,
non_snake_case
)]

#[path = "Attributes/attributes.rs"] pub mod attributes;
#[path = "Environment/environment.rs"] pub mod environment;
#[path = "Databases/databases.rs"] pub mod databases;
#[path = "Autotests/autotests.rs"] pub mod autotests;
#[path = "Collections/collections.rs"] pub mod collections;
#[path = "Functions/functions.rs"] pub mod functions;
#[path = "Input_Arguments/read_from_input.rs"] pub mod read_from_input;
#[path = "Compare/compare.rs"] pub mod compare;
#[path = "Time_Duration/tests.rs"] pub mod duration;
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
#[path = "ErrorsHandling/errors_handling.rs"] pub mod errors_handling;
#[path = "Files/files.rs"] pub mod files;
#[path = "Folders_Directories/folders_directories.rs"] pub mod folders_directories;
#[path = "Generics_Templates/generics.rs"] pub mod generics;
#[path = "Directories/directories.rs"] pub mod directories;
#[path = "Tuples/tuples.rs"] pub mod tuples;
#[path = "Console_IO/console_io.rs"] pub mod console_io;
#[path = "Pointers_and_References/pointers_and_reference.rs"] pub mod pointers_and_reference;
#[path = "Json/Json.rs"] pub mod Json;
#[path = "ScopingRules_LifeTIme_RAII/scoping_rules.rs"] pub mod scoping_rules;
#[path = "Networking/main.rs"] pub mod networking;
#[path = "Networking_Tokio/web_sockets_tungstenite.rs"] pub mod Tokio_web_sockets;
#[path = "Web_Framework/web_framework.rs"] pub mod web_framework;
#[path = "Multithreading/threads_main.rs"] pub mod multithreading;


fn main()
{ 
    // attributes::test_all();
    // environment::test_all();
    // autotests::test_all();
    // compare::test_all();
    // functions::test_all();
    // conditions::test_all();
    // memory::test_all();
    // read_from_input::test_all();
    // random::test_all();
    // ownership::test_all();
    // loops::test_all();
    // pointers_and_reference::test_all();
    // console_io::test_all();
    // errors_handling::test_all();
    // files::test_all();
    // folders_directories::test_all();
    // Json::test_all();
    // directories::test_all();
    // enums::test_all();
    // optional::test_all();
    // lambdas::test_all();
    // dynamic_dispatch::test_all();
    // strings::test_all();
    // scoping_rules::test_all();

    // ** Time - Duration: **/
    // duration::test_all();

    // tuples::test_all();
    // types::test_all();

    // ** Generics - Templates: **/
    // generics::test_all();

    // structs::test_all();
    // traits::test_all();

    // slices::test_all();
    // collections::test_all();

    // ** Networking: **/
    networking::test_all();
    // Tokio_udp_tests::test_all();
    // Tokio_web_sockets::test_all();

    // web_framework::test_all();
    
    // * * * Multithreading * * *
    // multithreading::test_all();

    // databases::test_all();
}
