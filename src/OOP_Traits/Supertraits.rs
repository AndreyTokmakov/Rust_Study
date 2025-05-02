trait Person {
    fn name(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

pub fn test_all()
{

}