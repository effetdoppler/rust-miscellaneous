use structopt::StructOpt;

// You are free to define some constants.

#[derive(Debug, StructOpt)]
#[structopt(name = "Numbers to Words Converter",
    about = "Convert an integer into letters.")]
struct Arg
{
    number: u64,
}

fn main()
{
    let arg = Arg::from_args();
    println!("{}", to_letters(arg.number));
}

fn to_letters(n: u64) -> String
{
    if n == 0
    {
        return String::from("zero");
    }
    
    let mut res = String::from("");
    let mut unit: u32 = 7;
    let mut number = n;
    let mut prev_num;

    while unit > 0
    {
        unit -= 1;
        prev_num = number;
        number /= 1000u64.pow(unit);
        if number != 0
        {
            if res.chars().count() != 0
            { 
                res.push_str(" ");
            }
            res.push_str(&get_number(number));
            res.push_str(&get_unity(unit));
        }
        number = prev_num%(1000u64.pow(unit));
    }
    
    res
}

fn get_unity(n: u32) -> String
{
    let mut res = String::from("");
    match n
    {
        1 => res.push_str(" thousand"),
        2 => res.push_str(" million"),
        3 => res.push_str(" billion"),
        4 => res.push_str(" trillion"),
        5 => res.push_str(" quadrillion"),
        6 => res.push_str(" quintillion"),
        _ => res.push_str(""),
    }
    
    res
}

fn get_number(n: u64) -> String
{
    let res;
    match n
    {
        1 => res = String::from("one"),
        2 => res = String::from("two"),
        3 => res = String::from("three"),
        4 => res = String::from("four"),
        5 => res = String::from("five"),
        6 => res = String::from("six"),
        7 => res = String::from("seven"),
        8 => res = String::from("eight"),
        9 => res = String::from("nine"),
        10 => res = String::from("ten"),
        11 => res = String::from("eleven"),
        12 => res = String::from("twelve"),
        13 => res = String::from("thirteen"),
        14 => res = String::from("fourteen"),
        15 => res = String::from("fifteen"),
        16 => res = String::from("sixteen"),
        17 => res = String::from("seventeen"),
        18 => res = String::from("eighteen"),
        19 => res = String::from("nineteen"),
        20 => res = String::from("twenty"),
        30 => res = String::from("thirty"),
        40 => res = String::from("forty"),
        50 => res = String::from("fifty"),
        60 => res = String::from("sixty"),
        70 => res = String::from("seventy"),
        80 => res = String::from("eighty"),
        90 => res = String::from("ninety"),
        _ => res = special_number(n),
    }
    res
}

fn special_number(n: u64) -> String
{
    let mut res;
    if n > 99
    {
        res = get_number(n/100) + " hundred";
        if n%100 != 0
        {
            res.push_str(" and ");
            res.push_str(&get_number(n%100));
        }
    }
    else
    {
        res = get_number((n/10)*10);
        if n%10 != 0
        {
            res.push_str("-");
            res.push_str(&get_number(n%10));
        }
    }
    
    res
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_quintillions()
    {
        assert_eq!(to_letters(0), "zero");
        assert_eq!(to_letters(1), "one");
        assert_eq!(to_letters(2), "two");
        assert_eq!(to_letters(3), "three");
        assert_eq!(to_letters(4), "four");
        assert_eq!(to_letters(5), "five");
        assert_eq!(to_letters(6), "six");
        assert_eq!(to_letters(7), "seven");
        assert_eq!(to_letters(8), "eight");
        assert_eq!(to_letters(9), "nine");

        assert_eq!(to_letters(10), "ten");
        assert_eq!(to_letters(11), "eleven");
        assert_eq!(to_letters(12), "twelve");
        assert_eq!(to_letters(13), "thirteen");
        assert_eq!(to_letters(14), "fourteen");
        assert_eq!(to_letters(15), "fifteen");
        assert_eq!(to_letters(16), "sixteen");
        assert_eq!(to_letters(17), "seventeen");
        assert_eq!(to_letters(18), "eighteen");
        assert_eq!(to_letters(19), "nineteen");
        assert_eq!(to_letters(20), "twenty");
        assert_eq!(to_letters(21), "twenty-one");
        assert_eq!(to_letters(30), "thirty");
        assert_eq!(to_letters(32), "thirty-two");
        assert_eq!(to_letters(40), "forty");
        assert_eq!(to_letters(43), "forty-three");
        assert_eq!(to_letters(50), "fifty");
        assert_eq!(to_letters(54), "fifty-four");
        assert_eq!(to_letters(60), "sixty");
        assert_eq!(to_letters(65), "sixty-five");
        assert_eq!(to_letters(70), "seventy");
        assert_eq!(to_letters(76), "seventy-six");
        assert_eq!(to_letters(80), "eighty");
        assert_eq!(to_letters(87), "eighty-seven");
        assert_eq!(to_letters(90), "ninety");
        assert_eq!(to_letters(98), "ninety-eight");

        assert_eq!(to_letters(100), "one hundred");
        assert_eq!(to_letters(101), "one hundred and one");
        assert_eq!(to_letters(115), "one hundred and fifteen");
        assert_eq!(to_letters(165), "one hundred and sixty-five");
        assert_eq!(to_letters(200), "two hundred");
        assert_eq!(to_letters(277), "two hundred and seventy-seven");
        assert_eq!(to_letters(580), "five hundred and eighty");
        assert_eq!(to_letters(999), "nine hundred and ninety-nine");

        assert_eq!(to_letters(1_000), "one thousand");
        assert_eq!(to_letters(5_454), "five thousand four hundred and fifty-four");
        assert_eq!(to_letters(9_999), "nine thousand nine hundred and ninety-nine");

        assert_eq!(to_letters(100_002), "one hundred thousand two");
        assert_eq!(to_letters(200_100_003), "two hundred million one hundred thousand three");

        assert_eq!(to_letters(18_446_744_073_709_551_615),
            "eighteen quintillion four hundred and forty-six \
             quadrillion seven hundred and forty-four trillion \
             seventy-three billion \
             seven hundred and nine million \
             five hundred and fifty-one thousand \
             six hundred and fifteen");
    }
}
