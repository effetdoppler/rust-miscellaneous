const SEPARATORS: &str = " ,;:!?./%*$=+)@_-('\"&1234567890\r\n";

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2
    {
        let text = &args[1];
        println!("{}\n{}", text, mix(text));
    }
    else
    {
        let defaut_text = "This is the default text.";
        println!("{}\n{}", defaut_text, mix(defaut_text));
    }
}

pub fn mix(s: &str) -> String
{
    let mut a: Vec<char> = s.chars().collect();
    for group in a.split_mut(|&c| SEPARATORS.contains(c))
    {
        if group.len() > 3
        {
            let len = group.len() - 1;
            &group[1..len].chunks_exact_mut(2).for_each(|x| x.swap(0, 1));
        }
    }
    a.iter().collect()
}

// You are free to write other functions.

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_mix()
    {
        assert_eq!(mix("I am not a number! I'm a free man!", "I am not a nmuebr! I'm a fere man!"));
        assert_eq!(mix("I was born ready!", "I was bron raedy!"));
    }
}