#![allow(
dead_code,
unused_imports,
unused_parens,
unused_variables,
non_snake_case
)]

#[path = "Attributes/attributes.rs"] pub mod attributes;
#[path = "Environment/environment.rs"] pub mod environment;
#[path = "Databases/sql_lite_tests.rs"] pub mod sql_lite_tests;
#[path = "Autotests/autotests.rs"] pub mod autotests;
#[path = "Functions/functions.rs"] pub mod functions;
#[path = "Input_Arguments/read_from_input.rs"] pub mod read_from_input;
#[path = "Compare/compare.rs"] pub mod compare;
#[path = "Dispatching/dynamic_dispatch.rs"] pub mod dynamic_dispatch;
#[path = "Loops/loops.rs"] pub mod loops;
#[path = "Ownership/ownership.rs"] pub mod ownership;
#[path = "Conditions/conditions.rs"] pub mod conditions;
#[path = "Lambdas/lambdas.rs"] pub mod lambdas;
#[path = "Traits/traits.rs"] pub mod traits;
#[path = "Networking/tcp_servers.rs"] pub mod tcp_servers;
#[path = "Networking/udp_servers.rs"] pub mod udp_servers;
#[path = "Networking/ProxyServer.rs"] pub mod ProxyServer;
#[path = "Types/types_tests.rs"] pub mod types;
#[path = "Memory/memory.rs"] pub mod memory;
#[path = "Structures/structs.rs"] pub mod structs;
#[path = "Enums/enums.rs"] pub mod enums;
#[path = "Optional/optional.rs"] pub mod optional;
#[path = "Collections/vector.rs"] pub mod collections_vector;
#[path = "Collections/hash_map.rs"] pub mod collections_hash_map;
#[path = "Collections/arrays.rs"] pub mod collections_arrays;
#[path = "Collections/LinkedList.rs"] pub mod LinkedList;
#[path = "Random/random.rs"] pub mod random;
#[path = "Strings/strings.rs"] pub mod strings;
#[path = "Slices/slices.rs"] pub mod slices;
#[path = "ErrorsHandling/errors_handling.rs"] pub mod errors_handling;
#[path = "Files/files.rs"] pub mod files;
#[path = "Generics_Templates/generics.rs"] pub mod generics;
#[path = "Directories/directories.rs"] pub mod directories;
#[path = "Tuples/tuples.rs"] pub mod tuples;
#[path = "Console_IO/console_io.rs"] pub mod console_io;
#[path = "Pointers_and_References/pointers_and_reference.rs"] pub mod pointers_and_reference;
#[path = "Networking_Tokio/udp_tests.rs"] pub mod Tokio_udp_tests;



fn main()
{
     /** STD: **/
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
     // slices::test_all();
     // pointers_and_reference::test_all();
     // console_io::test_all();
     // strings::test_all();
     // errors_handling::test_all();
     files::test_all();
     // directories::test_all();
     // structs::test_all();
     // enums::test_all();
     // optional::test_all();
     // lambdas::test_all();
     // dynamic_dispatch::test_all();
     // traits::test_all();
     // tuples::test_all();
     // types::test_all();
     // generics::test_all();

     // ** Collections: **/
     // collections_arrays::test_all();
     // collections_vector::test_all();
     // collections_hash_map::test_all();
     // LinkedList::test_all();

     // ** Networking: **/
     // tcp_servers::test_all();
     // udp_servers::test_all();
     // ProxyServer::test_all();
     // Tokio_udp_tests::test_all();

     // sql_lite_tests::test_all();
}
