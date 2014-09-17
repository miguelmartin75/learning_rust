use std::io::{BufferedReader, File};
use std::io::stdio::StdReader;

struct Config
{
    showLineNumbers : bool,
    showLineNumbersNonBlank : bool,
    displayTabs : bool,
    squeezeEmptyLines : bool,
    displayDollarAtEOL : bool
}

enum MultiReader 
{
    StdinReader(BufferedReader<StdReader>),
    FileReader(BufferedReader<File>)
}

fn process_args(config : &mut Config, filenames : &mut Vec<String>, args: &Vec<String>)
{
    for arg in args.slice_from(1).iter()
    {
        if arg.len() == 0
        {
            continue;
        }
        else if arg.len() >= 1 && arg.as_slice().char_at(0) == '-'
        {
            // change the config...
            match arg.as_slice().char_at(1)
            {
                // number non-blank lines
                'b' => 
                {
                    config.showLineNumbersNonBlank = true;
                },
                // number output lines
                'n' => 
                {
                    config.showLineNumbers = true
                },
                // display non-printing chars and $ for EOLF
                'e' => 
                {
                    config.displayDollarAtEOL = true;
                },
                // squeeze empty lines
                's' => 
                {
                    config.squeezeEmptyLines = true;
                },
                // display non printing chars and tabs as ^I
                't' => 
                {
                    config.displayTabs = true;
                },

                _ =>
                {
                    println!("invalid argument {}. please read man page!", arg);
                    break;
                }
            }
            continue;
        }

        filenames.push(arg.clone());
    }
}

fn print_lines<T : Reader>(b : &mut BufferedReader<T>, config : &Config)
{
    let mut lineNumber = 1u;
    for line in b.lines()
    {
        match line
        {
            Ok(l) => 
            { 
                if config.showLineNumbersNonBlank && !l.is_empty()
                {
                    print!("{}", lineNumber);
                    lineNumber += 1;
                }
                else if config.showLineNumbers
                {
                    print!("{}", lineNumber);
                    lineNumber += 1;
                }

                if config.displayTabs
                {
                    // display tabs as ^I
                    l.replace("\t", "^I");
                }

                print!("{}", l.as_slice().trim_chars('\n')); 
                if config.displayDollarAtEOL
                {
                    println!("$");
                }
                else
                {
                    println!("");
                }
            }
            _ => {}
        }
    }
}

fn main() 
{
    let args = std::os::args();
    let mut config = Config 
    { 
        showLineNumbers: false,
        showLineNumbersNonBlank: false,
        displayTabs: false,
        squeezeEmptyLines: false,
        displayDollarAtEOL: false,
    };
    
    let mut filenames = Vec::new();

    process_args(&mut config, &mut filenames, &args);

    let mut readers = 
        if filenames.len() == 0 
        {
            vec!(StdinReader(std::io::stdin()))
        }
        else 
        {
            let mut temp = Vec::new();

            for filename in filenames.iter()
            {
                let file = match File::open(&Path::new(filename.as_slice())) 
                {
                    Ok(a) => { a },
                    Err(e) => 
                    {
                        println!("error {}", e);
                        break;
                    }
                };
                temp.push(FileReader(BufferedReader::new(file)));
            }

            temp
        };


    for r in readers.mut_iter()
    {
        match *r 
        { 
            FileReader(ref mut f)  => print_lines(f, &config),
            StdinReader(ref mut s) => print_lines(s, &config)
        };
    }
}
