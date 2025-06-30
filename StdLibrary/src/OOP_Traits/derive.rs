
/** The following is a list of derivable traits:
    Comparison traits: Eq, PartialEq, Ord, PartialOrd.
    Clone, to create T from &T via a copy.
    Copy, to give a type 'copy semantics' instead of 'move semantics'.
    Hash, to compute a hash from &T.
    Default, to create an empty instance of a data type.
    Debug, to format a value using the {:?} formatter.
**/



/// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

/// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches 
{
    fn to_centimeters(&self) -> Centimeters 
    {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}



pub fn test_all()
{
    let foot = Inches(12);
    let meter = Centimeters(100.0);
    let compare_result: &str = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    
    
    println!("One foot equals {:?}", foot);
    println!("One foot is '{}' than one meter.", compare_result);
}

// One foot equals Inches(12)
// One foot is 'smaller' than one meter.